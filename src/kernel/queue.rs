




//! queue impletation and api
extern crate alloc;
use crate::kernel::projdefs::*;
use crate::kernel::riscv_virt::print;
use crate::kernel::riscv_virt::*;
use crate::kernel::tasks::*;
use crate::{portYIELD_WITHIN_API, taskYIELD_IF_USING_PREEMPTION};

pub const queueSEND_TO_BACK: BaseType = 1;
pub const queueOVERWRITE: BaseType = 2;
pub const queueUNLOCKED:i8 = -1;
pub const queueLOCKED_UNMODIFIED:i8 = 0;
pub const queueINT8_MAX:i8 = 127;
pub const pdPass: BaseType = 0;

use crate::kernel::projdefs::*;
use crate::kernel::tasks::*;




#[macro_export]
macro_rules! queueYIELD_IF_USING_PREEMPTION {
    () => {
        taskYIELD_IF_USING_PREEMPTION!();
    };
}

#[macro_export]
macro_rules! prvLockQueue {
    ($pxQueue: expr ) => {
        taskENTER_CRITICAL!();
        {
            if $pxQueue.cRxLock==queueUNLOCKED{
                $pxQueue.cRxLock=queueLOCKED_UNMODIFIED;
            }
            if $pxQueue.cTxLock==queueUNLOCKED{
                $pxQueue.cTxLock=queueLOCKED_UNMODIFIED;
            }
        }
        taskEXIT_CRITICAL!();
    };
}

use super::{
    linked_list::ListRealLink,
    portmacro::{BaseType, UBaseType},
};
use crate::kernel::portmacro::*;
use crate::{
    kernel::{
        linked_list::*,
        tasks::{vTaskEnterCritical, vTaskExitCritical},
    },
    taskENTER_CRITICAL, taskEXIT_CRITICAL,
};

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::format;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::arch::asm;
use core::ffi::c_void;
use core::{alloc::Layout, mem};
use libc::*;
use spin::RwLock;
pub type QueueHandle_t = Arc<RwLock<QueueDefinition>>;
pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub type xQUEUE = QueueDefinition;
pub type Queue_t = xQUEUE;






#[derive(Default)]
pub struct QueueDefinition {
    pcMesQueue: Vec<u8>, //message space

    pcHead: usize,
    pcTail: usize,
    pcWriteTo: usize,
    pcReadFrom: usize,
    cRxLock:i8,
    cTxLock:i8,
    pub xTasksWaitingToSend: ListRealLink,
    pub xTasksWaitingToReceive: ListRealLink,
    pub uxMessagesWaiting: UBaseType,
    uxLength: UBaseType,
    uxItemSize: UBaseType,
}
impl QueueDefinition {
    pub fn xQueueCreate(uxQueueLength: UBaseType, uxItemSize: UBaseType)->Self{
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
        let pucQueueStorage:usize=self.pcMesQueue.as_ptr() as usize;
        // let pucQueueStorage:usize=self.pcMesQueue.
        if (uxItemSize == 0) {
            self.pcHead = pxNewQueue;
        } else {
            self.pcHead = pucQueueStorage;
        }
        self.uxLength = uxQueueLength;
        self.uxItemSize = uxItemSize;
        self.xQueueGenericReset(1);

    }
    pub fn xQueueGenericReset(&mut self, xNewQueue: BaseType) -> BaseType {
        // vTaskEnterCritical();
        {
            self.pcTail = self.pcHead + (self.uxLength * self.uxItemSize) as usize;
            self.uxMessagesWaiting = 0;
            self.pcWriteTo = self.pcHead;
            //TODO: union
            self.pcReadFrom = self.pcHead + ((self.uxLength - 1) * self.uxItemSize) as usize;
            self.cRxLock = queueUNLOCKED;
            self.cTxLock - queueUNLOCKED;
            //TODO:lock

            if (xNewQueue == 0) {
                //TODO:
            } else {
                //initial in Default::default()
            }
        }
        // vTaskExitCritical();
        1
    }
}




/// create queue <br>
/// interface for user
pub fn xQueueCreate(uxQueueLength: UBaseType, uxItemSize: UBaseType) -> QueueHandle_t {
    xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
}

/// create queue
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
    print("queue create 22222");
    //TODO:
    // if pxNewQueue_ptr!=NULL
    let pucQueueStorage: usize = pxNewQueue_ptr as usize + mem::size_of::<Queue_t>();
    // #if( configSUPPORT_STATIC_ALLOCATION == 1 )
    // 29 {
    // 30
    // 31 pxNewQueue->ucStaticallyAllocated = pdFALSE;
    // 32 }
    // 33 #endif
    print("queue create 3333");
    prvInitialiseNewQueue(
        uxQueueLength,
        uxItemSize,
        pucQueueStorage,
        ucQueueType,
        pxNewQueue_ptr as usize,
    );
    print("queue create 4444");
    let pxNewQueue = unsafe { Box::from_raw((pxNewQueue_ptr as *mut Queue_t)) };

    // unsafe {
    //     pxNewQueue = &*(pxNewQueue_ptr as *mut Queue_t );
    // }
    Arc::new(RwLock::new(Box::<QueueDefinition>::into_inner(pxNewQueue)))
}

/// initialise new queue
pub fn prvInitialiseNewQueue(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
    pucQueueStorage: usize,
    ucQueueType: u8,
    pxNewQueue: usize,
) {
    print("prvInitialiseNewQueue 11111");
    let pxNewQueue_ =
        unsafe { mem::transmute::<*mut Queue_t, &mut QueueDefinition>(pxNewQueue as *mut Queue_t) };

    // let pxNewQueueBox = unsafe { Box::from_raw((pxNewQueue as *mut Queue_t)) };
    // let mut pxNewQueue_ = Box::<QueueDefinition>::into_inner(pxNewQueueBox);
    if (uxItemSize == 0) {
        pxNewQueue_.pcHead = pxNewQueue;
    } else {
        pxNewQueue_.pcHead = pucQueueStorage;
    }
    pxNewQueue_.uxLength = uxQueueLength;
    pxNewQueue_.uxItemSize = uxItemSize;


    print("prvInitialiseNewQueue 2222");
    xQueueGenericReset(pxNewQueue_, 1);
    let x = pxNewQueue_.xTasksWaitingToReceive.read().ux_number_of_items;
    print("prvInitialiseNewQueue 3333");
    let s = format!(
        "pchead:{:X},pcreadfrom:{:X}",
        pxNewQueue_.pcHead, pxNewQueue_.pcReadFrom
    );
    print(&s)
}




/// reset target queue
pub fn xQueueGenericReset(xQueue: &mut Queue_t, xNewQueue: BaseType) -> BaseType {
    // taskENTER_CRITICAL!();
    vTaskEnterCritical();

    {
        xQueue.pcTail = xQueue.pcHead + (xQueue.uxLength * xQueue.uxItemSize) as usize;
        xQueue.uxMessagesWaiting = 0;
        xQueue.pcWriteTo = xQueue.pcHead;
        //TODO: union
        xQueue.pcReadFrom = xQueue.pcHead + ((xQueue.uxLength - 1) * xQueue.uxItemSize) as usize;
        //TODO:lock
        if (xNewQueue == 0) {
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
    // vTaskExitCritical();
    1
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

pub fn xQueueGenericSend(
    xQueue: &mut Queue_t,
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
            if xQueue.uxMessagesWaiting < xQueue.uxLength || xCopyPosition == queueOVERWRITE {
                if cfg!(feature = "configUSE_QUEUE_SETS") {
                    let uxPreviousMessagesWaiting = xQueue.uxMessagesWaiting;
                    xYieldRequired = prvCopyDataToQueue(xQueue, pvItemToQueue, xCopyPosition);
                    //todo
                } else {
                    xYieldRequired = prvCopyDataToQueue(xQueue, pvItemToQueue, xCopyPosition);
                    if list_is_empty(&xQueue.xTasksWaitingToReceive) == false {
                        if xTaskRemoveFromEventList(&xQueue.xTasksWaitingToReceive) == true {
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
        taskEXIT_CRITICAL!();

        vTaskSuspendAll();
        //todo:prvLockQueue
        prvLockQueue!(xQueue);
        if xTaskCheckForTimeOut(&mut xTimeout, &mut xTicksToWait) == pdFALSE {
            if prvIsQueueFull(xQueue) == true {
                //todo:vTaskPlaceOnEventList
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
    pdFAIL as BaseType
}

pub fn prvCopyDataToQueue(xQueue: &mut Queue_t, pvItemToQueue: usize, xPosition: BaseType) -> bool {
    let mut uxMessagesWaiting = xQueue.uxMessagesWaiting;
    if xQueue.uxItemSize == 0 {
        if cfg!(feature = "configUSE_MUTEXES") {
            //todo
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    } else if xPosition == queueSEND_TO_BACK {
        unsafe {
            let x = *(pvItemToQueue as *mut i32);
            let s_ = format!(
                "Send  xQueue.pcWriteTo{:X},pvItemToQueue{:X},value{},xQueue.uxItemSize{:X}",
                xQueue.pcWriteTo, pvItemToQueue, x, xQueue.uxItemSize
            );

            vSendString(&s_);
            memcpy(
                xQueue.pcWriteTo as *mut c_void,
                pvItemToQueue as *const c_void,
                xQueue.uxItemSize as usize,
            );
            let xx = *(xQueue.pcWriteTo as *mut i32);
            let ss_ = format!(
                "Send over xQueue.pcWriteTo{:X},pvItemToQueue{:X},value{},xQueue.uxItemSize{:X}",
                xQueue.pcWriteTo, pvItemToQueue, xx, xQueue.uxItemSize
            );
            vSendString(&ss_);
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

    false
}

pub fn prvIsQueueFull(xQueue: &mut Queue_t) -> bool {
    let xReturn: bool;
    taskENTER_CRITICAL!();
    {
        if xQueue.uxMessagesWaiting == xQueue.uxLength {
            xReturn = true;
        } else {
            xReturn = false;
        }
    }
    taskEXIT_CRITICAL!();
    xReturn
}
pub fn prvIsQueueEmpty(xQueue: &mut Queue_t) -> bool {
    let xReturn: bool;
    taskENTER_CRITICAL!();
    {
        if xQueue.uxMessagesWaiting == 0 {
            xReturn = true;
        } else {
            xReturn = false;
        }
    }
    taskEXIT_CRITICAL!();
    xReturn
}
pub fn vQueueDelete(xQueue: QueueHandle_t) {
    let pxQueue = &*xQueue.write();
    let alloc_size: usize =
        mem::size_of::<xQUEUE>() + (xQueue.read().uxLength * xQueue.read().uxItemSize) as usize;
    let layout = Layout::from_size_align(alloc_size as usize, 4)
        .ok()
        .unwrap();
    unsafe {
        alloc::alloc::dealloc(pxQueue as *const Queue_t as *mut u8, layout);
    }
}
//消息队列发送
pub fn xQueueSend(
    xQueue: Option<QueueHandle_t>,
    pvItemToQueue: usize,
    xTicksToWait: TickType,
) -> BaseType {
    xQueueGenericSend(
        &mut *xQueue.unwrap().write(),
        pvItemToQueue,
        xTicksToWait,
        queueSEND_TO_BACK,
    )
}
//消息队列读取
pub fn prvCopyDataFromQueue(xQueue: &mut Queue_t, pvBuffer: usize) {
    if xQueue.uxItemSize != 0 {
        xQueue.pcReadFrom += xQueue.uxItemSize as usize;
        if xQueue.pcReadFrom >= xQueue.pcTail {
            xQueue.pcReadFrom = xQueue.pcHead;
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        unsafe {
            let xx = *(xQueue.pcReadFrom as *mut i32);
            let s = format!(
                "Read     xQueue.pcReadFrom{:X},pvItemToQueue{:X},value{},xQueue.uxItemSize{:X}",
                xQueue.pcReadFrom, pvBuffer, xx, xQueue.uxItemSize
            );
            vSendString(&s);
            memcpy(
                pvBuffer as *mut c_void,
                xQueue.pcReadFrom as *const c_void,
                xQueue.uxItemSize as usize,
            );
            let x = *(pvBuffer as *mut i32);
            let s_ = format!(
                "Read     xQueue.pcReadFrom{:X},pvItemToQueue{:X},value{},xQueue.uxItemSize{:X}",
                xQueue.pcReadFrom, pvBuffer, x, xQueue.uxItemSize
            );
            vSendString(&s_);
        }
    }
}
pub fn xQueueReceive(
    xQueue: Option<QueueHandle_t>,
    pvBuffer: usize,
    mut xTicksToWait: TickType,
) -> BaseType {
    // xQueueGenericReceive(xQueue, pvBuffer, xTicksToWait, pdFALSE as i32)
    let mut xEntryTimeSet: BaseType = pdFALSE;
    let mut xTimeOut: TimeOut = Default::default();
    let mut pcOriginalReadPosition: usize = 0;
    let xq = xQueue.unwrap();
    let xQueue = &mut (*xq.write());
    loop {
        taskENTER_CRITICAL!();
        {
            let uxMessagesWaiting = xQueue.uxMessagesWaiting;
            if uxMessagesWaiting > 0 {
                //TODO:
                pcOriginalReadPosition = xQueue.pcReadFrom;
                if xQueue.uxItemSize>0{
                    prvCopyDataFromQueue(xQueue, pvBuffer);
                }
                
                xQueue.uxMessagesWaiting = uxMessagesWaiting - 1;

                if list_is_empty(&xQueue.xTasksWaitingToSend) == false {
                    if (xTaskRemoveFromEventList(&xQueue.xTasksWaitingToSend) != false) {
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
            if (prvIsQueueEmpty(xQueue) != false) {
                //TOOD:vTaskPlaceOnEventList
                // vTaskPlaceOnEventList( &( pxQueue->xTasksWaitingToSend ), xTicksToWait );

                // /* Unlocking the queue means queue events can effect the
                //  * event list. It is possible that interrupts occurring now
                //  * remove this task from the event list again - but as the
                //  * scheduler is suspended the task will go onto the pending
                //  * ready list instead of the actual ready list. */
                // prvUnlockQueue( pxQueue );
                //TODO:prvUnlockQueue
                // /* Resuming the scheduler will move tasks from the pending
                //  * ready list into the ready list - so it is feasible that this
                //  * task is already in the ready list before it yields - in which
                //  * case the yield will not cause a context switch unless there
                //  * is also a higher priority task in the pending ready list. */
                if (vTaskResumeAll() == false) {
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
pub fn xQueuePeek(xQueue: QueueHandle_t, pvBuffer: usize, mut xTicksToWait: TickType) -> BaseType {
    let mut xEntryTimeSet: BaseType = pdFALSE;
    let mut xTimeOut: TimeOut = Default::default();
    let mut pcOriginalReadPosition: usize = 0;
    let xQueue = &mut *xQueue.write();
    loop {
        taskENTER_CRITICAL!();
        {
            let uxMessagesWaiting = xQueue.uxMessagesWaiting;
            if uxMessagesWaiting > 0 {
                //TODO:
                pcOriginalReadPosition = xQueue.pcReadFrom;
                prvCopyDataFromQueue(xQueue, pvBuffer);
                //different from queuereceive
                // xQueue.uxMessagesWaiting = uxMessagesWaiting - 1;
                /* The data is not being removed, so reset the read pointer. */
                xQueue.pcReadFrom = pcOriginalReadPosition;
                if list_is_empty(&xQueue.xTasksWaitingToReceive) == false {
                    if (xTaskRemoveFromEventList(&xQueue.xTasksWaitingToReceive) != false) {
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
            if (prvIsQueueEmpty(xQueue) != false) {
                //TOOD:vTaskPlaceOnEventList
                // vTaskPlaceOnEventList( &( pxQueue->xTasksWaitingToSend ), xTicksToWait );

                // /* Unlocking the queue means queue events can effect the
                //  * event list. It is possible that interrupts occurring now
                //  * remove this task from the event list again - but as the
                //  * scheduler is suspended the task will go onto the pending
                //  * ready list instead of the actual ready list. */
                // prvUnlockQueue( pxQueue );
                //TODO:prvUnlockQueue
                // /* Resuming the scheduler will move tasks from the pending
                //  * ready list into the ready list - so it is feasible that this
                //  * task is already in the ready list before it yields - in which
                //  * case the yield will not cause a context switch unless there
                //  * is also a higher priority task in the pending ready list. */
                if (vTaskResumeAll() == false) {
                    portYIELD_WITHIN_API!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                //TODO: prvUnlockQueue( pxQueue );
                prvUnlockQueue(xQueue);
                vTaskResumeAll();
            }
        } else {
            //TODO:prvUnlockQueue
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
// pub fn xQueueGenericReceive(
//     xQueue: QueueHandle_t,
//     pvBuffer: usize,
//     mut xTicksToWait: TickType,
//     xJustPeeking: BaseType,
// ) -> BaseType {
// }

pub fn prvUnlockQueue(xQueue: &mut QueueDefinition){
    taskENTER_CRITICAL!();
    {
        let mut cTxLock:i8=xQueue.cTxLock;
        while cTxLock>queueUNLOCKED{
            if cfg!(feature="configUSE_QUEUE_SETS"){
                //todo
            }
            else{
                if list_is_empty(&xQueue.xTasksWaitingToReceive)==false{
                    if xTaskRemoveFromEventList(&xQueue.xTasksWaitingToReceive) != false{
                        vTaskMissedYield!();
                    }
                    else{
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
                else{
                    break;
                }
            }
            cTxLock-=1;
        }
        xQueue.cTxLock=queueUNLOCKED;
    }
    taskEXIT_CRITICAL!();

    taskENTER_CRITICAL!();
    {
        let mut cRxLock:i8=xQueue.cTxLock;
        while cRxLock>queueUNLOCKED{
            if cfg!(feature="configUSE_QUEUE_SETS"){
                //todo
            }
            else{
                if list_is_empty(&xQueue.xTasksWaitingToReceive)==false{
                    if xTaskRemoveFromEventList(&xQueue.xTasksWaitingToReceive) != false{
                        vTaskMissedYield!();
                    }
                    else{
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
                else{
                    break;
                }
            }
            cRxLock-=1;
        }
        xQueue.cRxLock=queueUNLOCKED;
    }
    taskEXIT_CRITICAL!();
}
