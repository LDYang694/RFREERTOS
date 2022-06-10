//! Queue Definition and api
extern crate alloc;
use crate::configMAX_PRIORITIES;
use crate::kernel::projdefs::*;
use crate::kernel::tasks::*;
use crate::kernel::{
    linked_list::*,
    tasks::{vTaskEnterCritical, vTaskExitCritical},
};
use crate::portable::portmacro::*;
use crate::portable::portmacro::{BaseType, UBaseType};
use crate::portable::riscv_virt::*;
use crate::{
    mtCOVERAGE_TEST_MARKER, portENTER_CRITICAL, portEXIT_CRITICAL, portYIELD, portYIELD_WITHIN_API,
    taskENTER_CRITICAL, taskEXIT_CRITICAL, taskYIELD_IF_USING_PREEMPTION,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::arch::asm;
use core::ffi::c_void;
use core::{alloc::Layout, mem};
use spin::RwLock;

pub type QueueHandle_t = Arc<RwLock<QueueDefinition>>;
pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub const queueQUEUE_TYPE_MUTEX: u8 = 1;
pub const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub const queueQUEUE_TYPE_RECURSIVE_MUTEX: u8 = 4;
pub type xQUEUE = QueueDefinition;
pub type Queue_t = xQUEUE;

pub const queueSEND_TO_BACK: BaseType = 1;
pub const queueOVERWRITE: BaseType = 2;
pub const queueUNLOCKED: i8 = -1;
pub const queueLOCKED_UNMODIFIED: i8 = 0;
pub const queueINT8_MAX: i8 = 127;
pub const pdPass: BaseType = 0;

#[macro_export]
macro_rules! queueYIELD_IF_USING_PREEMPTION {
    () => {
        taskYIELD_IF_USING_PREEMPTION!();
    };
}

/// lock queue <br>
/// accept QueueHandle_t
#[macro_export]
macro_rules! prvLockQueue {
    ($pxQueue: expr ) => {
        taskENTER_CRITICAL!();
        {
            if $pxQueue.read().cRxLock == queueUNLOCKED {
                $pxQueue.write().cRxLock = queueLOCKED_UNMODIFIED;
            }
            if $pxQueue.read().cTxLock == queueUNLOCKED {
                $pxQueue.write().cTxLock = queueLOCKED_UNMODIFIED;
            }
        }
        taskEXIT_CRITICAL!();
    };
}

#[derive(Default)]
pub struct QueueDefinition {
    ///queue message space
    pcMesQueue: Vec<u8>,
    /// queue header pointer
    pcHead: usize,
    /// queue tail pointer
    pcTail: usize,
    /// position to write data
    pcWriteTo: usize,
    /// position to read data
    pub pcReadFrom: usize,
    pub cRxLock: i8,
    pub cTxLock: i8,
    pub xTasksWaitingToSend: ListRealLink,
    pub xTasksWaitingToReceive: ListRealLink,
    pub uxMessagesWaiting: UBaseType,
    pub uxLength: UBaseType,
    pub uxItemSize: UBaseType,
    pub xMutexHolder: Option<TaskHandle_t>,
    pub uxRecursiveCallCount: UBaseType,
    pub ucQueueType: u8,
}
impl QueueDefinition {
    /// QueueCreate function
    /// # Examples
    /// ```
    /// let xQueue = QueueDefinition::xQueueCreate(2,size_of::<u32>() as u32);
    ///
    /// ```
    pub fn xQueueCreate(uxQueueLength: UBaseType, uxItemSize: UBaseType) -> Self {
        QueueDefinition::xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
    }
    #[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]
    pub fn xQueueGenericCreate(
        uxQueueLength: UBaseType,
        uxItemSize: UBaseType,
        ucQueueType: u8,
    ) -> Self {
        let mut queue: QueueDefinition = Default::default();
        let mut xQueueSizeInBytes: isize;
        if uxItemSize == 0 {
            xQueueSizeInBytes = 0;
        } else {
            xQueueSizeInBytes = (uxQueueLength * uxItemSize) as isize;
        }
        queue.pcMesQueue = Vec::with_capacity(xQueueSizeInBytes as usize);
        queue.prvInitialiseNewQueue(uxQueueLength, uxItemSize, ucQueueType);
        queue
    }
    pub fn prvInitialiseNewQueue(
        &mut self,
        uxQueueLength: UBaseType,
        uxItemSize: UBaseType,
        ucQueueType: u8,
    ) {
        let pxNewQueue: usize = self as *mut QueueDefinition as usize;
        let pucQueueStorage: usize = self.pcMesQueue.as_ptr() as usize;
        // let pucQueueStorage:usize=self.pcMesQueue.
        if uxItemSize == 0 {
            self.pcHead = pxNewQueue;
        } else {
            self.pcHead = pucQueueStorage;
        }
        self.uxLength = uxQueueLength;
        self.uxItemSize = uxItemSize;
        self.ucQueueType = ucQueueType;
        self.xQueueGenericReset(1);
    }
    pub fn xQueueGenericReset(&mut self, xNewQueue: BaseType) -> BaseType {
        vTaskEnterCritical();
        {
            self.pcTail = self.pcHead + (self.uxLength * self.uxItemSize) as usize;
            self.uxMessagesWaiting = 0;
            self.pcWriteTo = self.pcHead;
            //TODO: union
            self.pcReadFrom = self.pcHead + ((self.uxLength - 1) * self.uxItemSize) as usize;
            self.cRxLock = queueUNLOCKED;
            self.cTxLock = queueUNLOCKED;
            //TODO:lock

            if xNewQueue == 0 {
                //TODO:
            } else {
                //initial in Default::default()
            }
        }
        vTaskExitCritical();
        1
    }
}

/// Create queue (user interface).
pub fn xQueueCreate(uxQueueLength: UBaseType, uxItemSize: UBaseType) -> QueueHandle_t {
    xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
}

/// Create queue.
pub fn xQueueGenericCreate(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
    ucQueueType: u8,
) -> QueueHandle_t {
    assert!(uxQueueLength > 0);
    let mut xQueueSizeInBytes: isize;
    if uxItemSize == 0 {
        xQueueSizeInBytes = 0;
    } else {
        xQueueSizeInBytes = (uxQueueLength * uxItemSize) as isize;
    }
    let alloc_size = mem::size_of::<Queue_t>() + xQueueSizeInBytes as usize;
    let layout = Layout::from_size_align(alloc_size as usize, 4)
        .ok()
        .unwrap();
    let pxNewQueue_ptr: *mut u8;
    unsafe {
        pxNewQueue_ptr = alloc::alloc::alloc(layout);
    }

    let pucQueueStorage: usize = pxNewQueue_ptr as usize + mem::size_of::<Queue_t>();

    prvInitialiseNewQueue(
        uxQueueLength,
        uxItemSize,
        pucQueueStorage,
        ucQueueType,
        pxNewQueue_ptr as usize,
    );

    let pxNewQueue = unsafe { Box::from_raw(pxNewQueue_ptr as *mut Queue_t) };

    Arc::new(RwLock::new(Box::<QueueDefinition>::into_inner(pxNewQueue)))
}

/// Initialise new queue.
pub fn prvInitialiseNewQueue(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
    pucQueueStorage: usize,
    ucQueueType: u8,
    pxNewQueue: usize,
) {
    let pxNewQueue_ =
        unsafe { mem::transmute::<*mut Queue_t, &mut QueueDefinition>(pxNewQueue as *mut Queue_t) };

    // let pxNewQueueBox = unsafe { Box::from_raw((pxNewQueue as *mut Queue_t)) };
    // let mut pxNewQueue_ = Box::<QueueDefinition>::into_inner(pxNewQueueBox);
    if uxItemSize == 0 {
        pxNewQueue_.pcHead = pxNewQueue;
    } else {
        pxNewQueue_.pcHead = pucQueueStorage;
    }
    pxNewQueue_.uxLength = uxQueueLength;
    pxNewQueue_.uxItemSize = uxItemSize;

    xQueueGenericReset(pxNewQueue_, 1);
    let x = pxNewQueue_.xTasksWaitingToReceive.read().ux_number_of_items;
}

/// Reset target queue.
pub fn xQueueGenericReset(xQueue: &mut Queue_t, xNewQueue: BaseType) -> BaseType {
    vTaskEnterCritical();

    {
        xQueue.pcTail = xQueue.pcHead + (xQueue.uxLength * xQueue.uxItemSize) as usize;
        xQueue.uxMessagesWaiting = 0;
        xQueue.pcWriteTo = xQueue.pcHead;
        //TODO: union
        xQueue.pcReadFrom = xQueue.pcHead + ((xQueue.uxLength - 1) * xQueue.uxItemSize) as usize;
        //TODO:lock
        if xNewQueue == 0 {
            //TODO:
        } else {
            let mut rec = Arc::new(RwLock::new(XList::default()));
            let mut send = Arc::new(RwLock::new(XList::default()));

            unsafe {
                Arc::increment_strong_count(Arc::into_raw(rec.clone()));
                Arc::increment_strong_count(Arc::into_raw(send.clone()));
                memcpy(
                    &mut xQueue.xTasksWaitingToReceive as *mut Arc<RwLock<XList>> as *mut c_void,
                    &mut rec as *mut Arc<RwLock<XList>> as *mut c_void,
                    4,
                );
                memcpy(
                    &mut xQueue.xTasksWaitingToSend as *mut Arc<RwLock<XList>> as *mut c_void,
                    &mut send as *mut Arc<RwLock<XList>> as *mut c_void,
                    4,
                );
            }
            // *xQueue.xTasksWaitingToReceive.write() = XList::default();
            // mem::take(&mut (xQueue.xTasksWaitingToReceive));
            // xQueue.xTasksWaitingToSend = Arc::new(RwLock::new(XList::default()));
            // xQueue.xTasksWaitingToReceive = Arc::new(RwLock::new(XList::default()));
            mem::forget(&rec);
            mem::forget(&send);
        }
    }
    //TODO: tmp remove
    vTaskExitCritical();
    pdTRUE
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/// Send data to queue.
pub fn xQueueGenericSend(
    xQueue: &QueueHandle_t,
    pvItemToQueue: usize,
    mut xTicksToWait: TickType,
    xCopyPosition: BaseType,
) -> BaseType {
    let mut xYieldRequired: bool = false;
    let mut xEntryTimeSet: bool = false;
    let mut xTimeout: TimeOut = Default::default();
    loop {
        taskENTER_CRITICAL!();

        {
            //let mut xQueue_ = xQueue.write();
            if xQueue.read().uxMessagesWaiting < xQueue.read().uxLength
                || xCopyPosition == queueOVERWRITE
            {
                if cfg!(feature = "configUSE_QUEUE_SETS") {
                    let uxPreviousMessagesWaiting = xQueue.read().uxMessagesWaiting;
                    xYieldRequired =
                        prvCopyDataToQueue(&mut xQueue.write(), pvItemToQueue, xCopyPosition);
                    //todo
                } else {
                    xYieldRequired =
                        prvCopyDataToQueue(&mut xQueue.write(), pvItemToQueue, xCopyPosition);
                    if list_is_empty(&xQueue.read().xTasksWaitingToReceive) == false {
                        if xTaskRemoveFromEventList(&xQueue.read().xTasksWaitingToReceive) == true {
                            queueYIELD_IF_USING_PREEMPTION!();
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    } else if xYieldRequired == true {
                        queueYIELD_IF_USING_PREEMPTION!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
                taskEXIT_CRITICAL!();
                return pdPASS as BaseType;
            } else {
                if xTicksToWait == 0 {
                    taskEXIT_CRITICAL!();
                    return errQUEUE_FULL;
                } else if xEntryTimeSet == false {
                    vTaskInternalSetTimeOutState(&mut xTimeout);
                    xEntryTimeSet = true;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
        }
        vTaskSuspendAll();
        taskEXIT_CRITICAL!();

        prvLockQueue!(xQueue);
        if xTaskCheckForTimeOut(&mut xTimeout, &mut xTicksToWait) == pdFALSE {
            if prvIsQueueFull(xQueue) == true {
                vTaskPlaceOnEventList(&xQueue.write().xTasksWaitingToSend, xTicksToWait);
                prvUnlockQueue(xQueue);
                if vTaskResumeAll() == false {
                    portYIELD_WITHIN_API!();
                }
            } else {
                prvUnlockQueue(xQueue);
                vTaskResumeAll();
            }
        } else {
            prvUnlockQueue(xQueue);
            vTaskResumeAll();
            return errQUEUE_FULL;
        }
    }
}

/// Copy data from target address to queue.
pub fn prvCopyDataToQueue(
    xQueue: &mut QueueDefinition,
    pvItemToQueue: usize,
    xPosition: BaseType,
) -> bool {
    let mut uxMessagesWaiting = xQueue.uxMessagesWaiting;
    let mut xReturn: bool = false;
    if xQueue.uxItemSize == 0 {
        if cfg!(feature = "configUSE_MUTEXES") {
            if xQueue.ucQueueType == queueQUEUE_TYPE_MUTEX
                || xQueue.ucQueueType == queueQUEUE_TYPE_RECURSIVE_MUTEX
            {
                xReturn = xTaskPriorityDisinherit(xQueue.xMutexHolder.as_ref()) == pdTRUE;
                xQueue.xMutexHolder = None;
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    } else if xPosition == queueSEND_TO_BACK {
        unsafe {
            memcpy(
                xQueue.pcWriteTo as *mut c_void,
                pvItemToQueue as *const c_void,
                xQueue.uxItemSize as usize,
            );
        }
        xQueue.pcWriteTo += xQueue.uxItemSize as usize;
        if xQueue.pcWriteTo >= xQueue.pcTail {
            xQueue.pcWriteTo = xQueue.pcHead;
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    } else {
        unsafe {
            memcpy(
                xQueue.pcReadFrom as *mut c_void,
                pvItemToQueue as *const c_void,
                xQueue.uxItemSize as usize,
            );
        }
        xQueue.pcReadFrom -= xQueue.uxItemSize as usize;
        if xQueue.pcReadFrom < xQueue.pcHead {
            xQueue.pcReadFrom = xQueue.pcTail - xQueue.uxItemSize as usize;
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        if xPosition == queueOVERWRITE {
            if uxMessagesWaiting > 0 {
                uxMessagesWaiting -= 1;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }

    xQueue.uxMessagesWaiting = uxMessagesWaiting + 1;

    xReturn
}

/// Return if queue is full.
pub fn prvIsQueueFull(xQueue: &QueueHandle_t) -> bool {
    let xReturn: bool;
    taskENTER_CRITICAL!();
    {
        if xQueue.read().uxMessagesWaiting == xQueue.read().uxLength {
            xReturn = true;
        } else {
            xReturn = false;
        }
    }
    taskEXIT_CRITICAL!();
    xReturn
}

/// Return if queue is empty.
pub fn prvIsQueueEmpty(xQueue: &QueueHandle_t) -> bool {
    let xReturn: bool;
    taskENTER_CRITICAL!();
    {
        if xQueue.read().uxMessagesWaiting == 0 {
            xReturn = true;
        } else {
            xReturn = false;
        }
    }
    taskEXIT_CRITICAL!();
    xReturn
}

/// Delete target queue.
pub fn vQueueDelete(xQueue: QueueHandle_t) {
    let alloc_size: usize =
        mem::size_of::<xQUEUE>() + (xQueue.read().uxLength * xQueue.read().uxItemSize) as usize;
    let pxQueue = &*xQueue.write();

    let layout = Layout::from_size_align(alloc_size as usize, 4)
        .ok()
        .unwrap();
    unsafe {
        alloc::alloc::dealloc(pxQueue as *const Queue_t as *mut u8, layout);
    }
}
/// Send data to queue (user interface).
pub fn xQueueSend(
    xQueue: &QueueHandle_t,
    pvItemToQueue: usize,
    xTicksToWait: TickType,
) -> BaseType {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
}

/// Copy data from queue to target address.
pub fn prvCopyDataFromQueue(xQueue: &mut QueueDefinition, pvBuffer: usize) {
    if xQueue.uxItemSize != 0 {
        xQueue.pcReadFrom += xQueue.uxItemSize as usize;
        if xQueue.pcReadFrom >= xQueue.pcTail {
            xQueue.pcReadFrom = xQueue.pcHead;
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        unsafe {
            memcpy(
                pvBuffer as *mut c_void,
                xQueue.pcReadFrom as *const c_void,
                xQueue.uxItemSize as usize,
            );
        }
    }
}

/// Receive data from queue.<br>
/// Received data is removed from queue.
pub fn xQueueReceive(
    xQueue: &QueueHandle_t,
    pvBuffer: usize,
    mut xTicksToWait: TickType,
) -> BaseType {
    // xQueueGenericReceive(xQueue, pvBuffer, xTicksToWait, pdFALSE as i32)
    let mut xEntryTimeSet: BaseType = pdFALSE;
    let mut xTimeOut: TimeOut = Default::default();
    let mut pcOriginalReadPosition: usize = 0;
    let mut xInheritanceOccurred: BaseType = pdFALSE;
    //let xq = xQueue.unwrap();
    //let xQueue = &mut (*xq.write());
    loop {
        taskENTER_CRITICAL!();
        {
            //let mut xQueue_ = xQueue.write();

            let uxMessagesWaiting = xQueue.read().uxMessagesWaiting;
            if uxMessagesWaiting > 0 {
                //TODO:
                pcOriginalReadPosition = xQueue.read().pcReadFrom;
                if xQueue.read().uxItemSize > 0 {
                    prvCopyDataFromQueue(&mut xQueue.write(), pvBuffer);
                }

                xQueue.write().uxMessagesWaiting = uxMessagesWaiting - 1;

                if cfg!(feature = "configUSE_MUTEXES") {
                    if xQueue.read().ucQueueType == queueQUEUE_TYPE_MUTEX
                        || xQueue.read().ucQueueType == queueQUEUE_TYPE_RECURSIVE_MUTEX
                    {
                        xQueue.write().xMutexHolder = pvTaskIncrementMutexHeldCount();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }

                if list_is_empty(&xQueue.read().xTasksWaitingToSend) == false {
                    if xTaskRemoveFromEventList(&xQueue.read().xTasksWaitingToSend) != false {
                        queueYIELD_IF_USING_PREEMPTION!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    //list empty
                    mtCOVERAGE_TEST_MARKER!();
                }

                taskEXIT_CRITICAL!();
                return pdPASS;
            } else {
                if xTicksToWait == 0 {
                    taskEXIT_CRITICAL!();
                    return errQUEUE_EMPTY;
                } else if xEntryTimeSet == pdFALSE {
                    vTaskInternalSetTimeOutState(&mut xTimeOut);
                    xEntryTimeSet = pdTRUE;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
        }
        vTaskSuspendAll();
        taskEXIT_CRITICAL!();

        prvLockQueue!(xQueue);
        if xTaskCheckForTimeOut(&mut xTimeOut, &mut xTicksToWait) == pdFALSE {
            if prvIsQueueEmpty(xQueue) != false {
                vTaskPlaceOnEventList(&xQueue.write().xTasksWaitingToReceive, xTicksToWait);

                // /* Unlocking the queue means queue events can effect the
                //  * event list. It is possible that interrupts occurring now
                //  * remove this task from the event list again - but as the
                //  * scheduler is suspended the task will go onto the pending
                //  * ready list instead of the actual ready list. */
                if cfg!(feature = "configUSE_MUTEXES") {
                    if xQueue.read().ucQueueType == queueQUEUE_TYPE_MUTEX
                        || xQueue.read().ucQueueType == queueQUEUE_TYPE_RECURSIVE_MUTEX
                    {
                        taskENTER_CRITICAL!();
                        xInheritanceOccurred =
                            xTaskPriorityInherit(xQueue.write().xMutexHolder.as_ref());
                        taskEXIT_CRITICAL!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }

                prvUnlockQueue(xQueue);
                if vTaskResumeAll() == false {
                    portYIELD_WITHIN_API!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                prvUnlockQueue(xQueue);
                vTaskResumeAll();
            }
        } else {
            prvUnlockQueue(xQueue);
            vTaskResumeAll();

            if prvIsQueueEmpty(xQueue) != false {
                if cfg!(feature = "configUSE_MUTEXES") {
                    if xInheritanceOccurred != pdFALSE {
                        taskENTER_CRITICAL!();
                        let uxHighestWaitingPriority = prvGetDisinheritPriorityAfterTimeout(xQueue);
                        vTaskPriorityDisinheritAfterTimeout(
                            xQueue.write().xMutexHolder.as_ref(),
                            uxHighestWaitingPriority,
                        );
                        taskEXIT_CRITICAL!();
                    }
                }
                return errQUEUE_EMPTY;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
    }
}

/// Receive data from queue in ISR. <br>
/// Received data is removed from queue.
pub fn xQueueReceiveFromISR(
    xQueue: &QueueHandle_t,
    pvBuffer: usize,
    pxHigherPriorityTaskWoken: &mut BaseType,
) -> BaseType {
    let mut xReturn: BaseType = pdFALSE;
    //todo:portASSERT_IF_INTERRUPT_PRIORITY_INVALID();
    if xQueue.read().uxMessagesWaiting > 0 {
        let cRxLock: i8 = xQueue.read().cRxLock;
        prvCopyDataFromQueue(&mut xQueue.write(), pvBuffer);
        xQueue.write().uxMessagesWaiting -= 1;
        if cRxLock == queueUNLOCKED {
            if list_is_empty(&xQueue.write().xTasksWaitingToSend) == false {
                if xTaskRemoveFromEventList(&xQueue.write().xTasksWaitingToSend) == true {
                    *pxHigherPriorityTaskWoken = pdTRUE;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            xQueue.write().cRxLock = cRxLock + 1;
        }
        xReturn = pdTRUE;
    }
    xReturn
}

/// Send data to queue in ISR (user interface).
pub fn xQueueSendFromISR(
    xQueue: &QueueHandle_t,
    pvBuffer: usize,
    pxHigherPriorityTaskWoken: &mut BaseType,
) -> BaseType {
    xQueueGenericSendFromISR(
        xQueue,
        pvBuffer,
        pxHigherPriorityTaskWoken,
        queueSEND_TO_BACK,
    )
}

/// Send data to queue in ISR.
pub fn xQueueGenericSendFromISR(
    xQueue: &QueueHandle_t,
    pvBuffer: usize,
    pxHigherPriorityTaskWoken: &mut BaseType,
    xCopyPosition: BaseType,
) -> BaseType {
    let mut xReturn: BaseType = pdFALSE;
    if xQueue.read().uxMessagesWaiting < xQueue.read().uxLength || xCopyPosition == queueOVERWRITE {
        let cTxLock: i8 = xQueue.write().cTxLock;

        prvCopyDataToQueue(&mut xQueue.write(), pvBuffer, xCopyPosition);
        if cTxLock == queueUNLOCKED {
            //todo:configUSE_QUEUE_SETS
            if list_is_empty(&xQueue.write().xTasksWaitingToReceive) == false {
                if xTaskRemoveFromEventList(&xQueue.write().xTasksWaitingToReceive) == true {
                    *pxHigherPriorityTaskWoken = pdTRUE;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            xQueue.write().cTxLock = cTxLock + 1;
        }
        xReturn = pdTRUE;
    }
    xReturn
}

/// Get data from queue without removing it from queue.
pub fn xQueuePeek(xQueue: &QueueHandle_t, pvBuffer: usize, mut xTicksToWait: TickType) -> BaseType {
    let mut xEntryTimeSet: BaseType = pdFALSE;
    let mut xTimeOut: TimeOut = Default::default();
    let mut pcOriginalReadPosition: usize = 0;

    loop {
        taskENTER_CRITICAL!();
        {
            let mut xQueue_ = &mut *xQueue.write();
            let uxMessagesWaiting = xQueue_.uxMessagesWaiting;
            if uxMessagesWaiting > 0 {
                //TODO:
                pcOriginalReadPosition = xQueue_.pcReadFrom;
                prvCopyDataFromQueue(&mut xQueue_, pvBuffer);
                //different from queuereceive
                // xQueue.uxMessagesWaiting = uxMessagesWaiting - 1;
                /* The data is not being removed, so reset the read pointer. */
                xQueue_.pcReadFrom = pcOriginalReadPosition;
                if list_is_empty(&xQueue_.xTasksWaitingToReceive) == false {
                    if xTaskRemoveFromEventList(&xQueue_.xTasksWaitingToReceive) != false {
                        queueYIELD_IF_USING_PREEMPTION!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    //list empty
                    mtCOVERAGE_TEST_MARKER!();
                }
                taskEXIT_CRITICAL!();
                return pdPASS;
            } else {
                if xTicksToWait == 0 {
                    taskEXIT_CRITICAL!();
                    return errQUEUE_EMPTY;
                } else if xEntryTimeSet == pdFALSE {
                    vTaskInternalSetTimeOutState(&mut xTimeOut);
                    xEntryTimeSet = pdTRUE;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
        }
        // {if xQueue.uxMessagesWaiting<xQueue.uxLength||xC}
        taskEXIT_CRITICAL!();
        vTaskSuspendAll();
        prvLockQueue!(xQueue);
        if xTaskCheckForTimeOut(&mut xTimeOut, &mut xTicksToWait) == pdFALSE {
            if prvIsQueueEmpty(xQueue) != false {
                vTaskPlaceOnEventList(&xQueue.write().xTasksWaitingToReceive, xTicksToWait);

                prvUnlockQueue(xQueue);
                if vTaskResumeAll() == false {
                    portYIELD_WITHIN_API!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                prvUnlockQueue(xQueue);
                vTaskResumeAll();
            }
        } else {
            prvUnlockQueue(xQueue);
            vTaskResumeAll();
            if prvIsQueueEmpty(xQueue) != false {
                return errQUEUE_EMPTY;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
    }
}

/// Get data from queue in ISR without removing it from queue.
pub fn xQueuePeekFromISR(xQueue: &QueueHandle_t, pvBuffer: usize) -> BaseType {
    let mut xReturn: BaseType = pdFALSE;
    if xQueue.read().uxMessagesWaiting > 0 {
        let pcOriginalReadPosition = xQueue.read().pcReadFrom;
        prvCopyDataFromQueue(&mut xQueue.write(), pvBuffer);
        xQueue.write().pcReadFrom = pcOriginalReadPosition;
        xReturn = pdTRUE;
    }
    xReturn
}

/// Unlock queue and deal with operations during lock.
pub fn prvUnlockQueue(xQueue: &QueueHandle_t) {
    taskENTER_CRITICAL!();
    {
        let mut cTxLock: i8 = xQueue.read().cTxLock;
        while cTxLock > queueUNLOCKED {
            if cfg!(feature = "configUSE_QUEUE_SETS") {
                //todo
            } else {
                if list_is_empty(&xQueue.write().xTasksWaitingToReceive) == false {
                    if xTaskRemoveFromEventList(&xQueue.write().xTasksWaitingToReceive) != false {
                        vTaskMissedYield!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    break;
                }
            }
            cTxLock -= 1;
        }
        xQueue.write().cTxLock = queueUNLOCKED;
    }
    taskEXIT_CRITICAL!();

    taskENTER_CRITICAL!();
    {
        let mut cRxLock: i8 = xQueue.read().cTxLock;
        while cRxLock > queueUNLOCKED {
            if cfg!(feature = "configUSE_QUEUE_SETS") {
                //todo
            } else {
                if list_is_empty(&xQueue.write().xTasksWaitingToReceive) == false {
                    if xTaskRemoveFromEventList(&xQueue.write().xTasksWaitingToReceive) != false {
                        vTaskMissedYield!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    break;
                }
            }
            cRxLock -= 1;
        }
        xQueue.write().cRxLock = queueUNLOCKED;
    }
    taskEXIT_CRITICAL!();
}

/// Get disinherit priority for mutex.
pub fn prvGetDisinheritPriorityAfterTimeout(xQueue: &QueueHandle_t) -> UBaseType {
    let uxHighestPriorityOfWaitingTasks: UBaseType;
    if list_current_list_length(&xQueue.write().xTasksWaitingToReceive) > 0 {
        uxHighestPriorityOfWaitingTasks = *configMAX_PRIORITIES
            - list_get_value_of_head_entry(&xQueue.write().xTasksWaitingToReceive);
    } else {
        uxHighestPriorityOfWaitingTasks = tskIDLE_PRIORITY;
    }
    uxHighestPriorityOfWaitingTasks
}
