extern crate alloc;
use crate::kernel::projdefs::*;
use crate::kernel::tasks::*;
use crate::{portYIELD_WITHIN_API, taskYIELD_IF_USING_PREEMPTION};

pub const queueSEND_TO_BACK: BaseType = 1;
pub const queueOVERWRITE: BaseType = 2;
pub const pdPass: BaseType = 0;

#[macro_export]
macro_rules! queueYIELD_IF_USING_PREEMPTION {
    () => {
        taskYIELD_IF_USING_PREEMPTION!();
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
use alloc::sync::{Arc, Weak};
use core::arch::asm;
use core::ffi::c_void;
use core::{alloc::Layout, mem};
use libc::*;
use spin::RwLock;
pub type QueueHandle_t = Arc<RwLock<QueueDefinition>>;
pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub type xQUEUE = QueueDefinition;
pub type Queue_t = xQUEUE;
pub struct QueueDefinition {
    pcHead: usize,
    pcTail: usize,
    pcWriteTo: usize,
    pcReadFrom: usize,
    xTasksWaitingToSend: ListRealLink,
    xTasksWaitingToReceive: ListRealLink,
    uxMessagesWaiting: UBaseType,
    uxLength: UBaseType,
    uxItemSize: UBaseType,
}
//TODO: xqueue default
pub fn xQueueCreate(uxQueueLength: UBaseType, uxItemSize: UBaseType) -> QueueHandle_t {
    xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
}
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
    //TODO:
    // if pxNewQueue_ptr!=NULL
    let pucQueueStorage: usize = pxNewQueue_ptr as usize + mem::size_of::<Queue_t>();
    // #if( configSUPPORT_STATIC_ALLOCATION == 1 )
    // 29 {
    // 30
    // 31 pxNewQueue->ucStaticallyAllocated = pdFALSE;
    // 32 }
    // 33 #endif
    prvInitialiseNewQueue(
        uxQueueLength,
        uxItemSize,
        pucQueueStorage,
        ucQueueType,
        pxNewQueue_ptr as usize,
    );
    let pxNewQueue = unsafe { Box::from_raw((pxNewQueue_ptr as *mut Queue_t)) };

    // unsafe {
    //     pxNewQueue = &*(pxNewQueue_ptr as *mut Queue_t );
    // }
    Arc::new(RwLock::new(Box::<QueueDefinition>::into_inner(pxNewQueue)))
}
pub fn prvInitialiseNewQueue(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
    pucQueueStorage: usize,
    ucQueueType: u8,
    pxNewQueue: usize,
) {
    let pxNewQueueBox = unsafe { Box::from_raw((pxNewQueue as *mut Queue_t)) };
    let mut pxNewQueue_ = Box::<QueueDefinition>::into_inner(pxNewQueueBox);
    if (uxItemSize == 0) {
        pxNewQueue_.pcHead = pxNewQueue;
    } else {
        pxNewQueue_.pcHead = pucQueueStorage;
    }
    pxNewQueue_.uxLength = uxQueueLength;
    pxNewQueue_.uxItemSize = uxItemSize;
    xQueueGenericReset(&mut pxNewQueue_, 1);
}

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
            xQueue.xTasksWaitingToSend = Arc::new(RwLock::new(XList::default()));
            xQueue.xTasksWaitingToReceive = Arc::new(RwLock::new(XList::default()));
            mem::forget(&xQueue.xTasksWaitingToSend);
            mem::forget(&xQueue.xTasksWaitingToReceive);
        }
    }
    // taskEXIT_CRITICAL!();
    vTaskExitCritical();
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
        if xTaskCheckForTimeOut(&mut xTimeout, &mut xTicksToWait) == pdFALSE {
            if prvIsQueueFull(xQueue) == true {
                //todo:vTaskPlaceOnEventList
                //todo:prvUnlockQueue
                if vTaskResumeAll() == false {
                    portYIELD_WITHIN_API!();
                }
            } else {
                //todo:prvUnlockQueue
                vTaskResumeAll();
            }
        } else {
            //todo:prvUnlockQueue
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
            memcpy(
                xQueue.pcReadFrom as *mut c_void,
                pvItemToQueue as *const c_void,
                xQueue.uxItemSize as usize,
            );
        }
        xQueue.pcWriteTo += xQueue.uxItemSize as usize;
        if xQueue.pcWriteTo >= xQueue.pcTail {
            xQueue.pcWriteTo = xQueue.pcTail;
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
        xQueue.pcWriteTo -= xQueue.uxItemSize as usize;
        if xQueue.pcWriteTo < xQueue.pcHead {
            xQueue.pcWriteTo = xQueue.pcTail - xQueue.uxItemSize as usize;
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
pub fn xQueueSend(xQueue: QueueHandle_t, pvItemToQueue: usize, xTicksToWait: TickType) -> BaseType {
    xQueueGenericSend(
        &mut *xQueue.write(),
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
            memcpy(
                pvBuffer as *mut c_void,
                xQueue.pcReadFrom as *const c_void,
                xQueue.uxItemSize as usize,
            );
        }
    }
}
pub fn xQueueReceive(xQueue: QueueHandle_t, pvBuffer: usize, mut xTicksToWait: TickType) -> BaseType {
    // xQueueGenericReceive(xQueue, pvBuffer, xTicksToWait, pdFALSE as i32)
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
        //TODO:prvLockQueue
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
                vTaskResumeAll();
            }
        } else {
            //TODO:prvUnlockQueue
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
        //TODO:prvLockQueue
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
                vTaskResumeAll();
            }
        } else {
            //TODO:prvUnlockQueue
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
