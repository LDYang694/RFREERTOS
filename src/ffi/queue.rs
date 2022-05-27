use crate::kernel::linked_list::*;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::tasks::vTaskExitCritical;
use crate::kernel::tasks::*;
use crate::projdefs::*;
use crate::vTaskEnterCritical;
use crate::{
    mtCOVERAGE_TEST_MARKER, portENTER_CRITICAL, portEXIT_CRITICAL, portYIELD, portYIELD_WITHIN_API,
    prvLockQueue, queueYIELD_IF_USING_PREEMPTION, taskENTER_CRITICAL, taskEXIT_CRITICAL,
    taskYIELD_IF_USING_PREEMPTION,
};
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::arch::asm;
use core::ffi::c_void;
use core::mem::forget;
use core::mem::size_of;
use spin::RwLock;

pub type QueueHandle_c = *const RwLock<QueueDefinition>;

#[no_mangle]
pub extern "C" fn xQueueCreateToC(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
) -> QueueHandle_c {
    let mut temp = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(
        uxQueueLength,
        uxItemSize,
    )));
    Arc::into_raw(temp)
}

#[no_mangle]
pub extern "C" fn uxQueueMessagesWaiting(xQueue: QueueHandle_c) -> UBaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let ret = temp.read().uxMessagesWaiting;
    let xQueue_ = Arc::into_raw(temp);
    ret
}

#[no_mangle]
pub extern "C" fn cGetQueueRxLock(xQueue: QueueHandle_t) -> i8 {
    //forget(&xQueue);
    xQueue.read().cRxLock
}

#[no_mangle]
pub extern "C" fn cGetQueueTxLock(xQueue: QueueHandle_t) -> i8 {
    //forget(&xQueue);
    xQueue.read().cTxLock
}

#[no_mangle]
pub extern "C" fn xQueueSendToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvItemToQueue: usize,
    xTicksToWait: TickType,
) -> BaseType {
    let xReturn = xQueueGenericSendToC(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK);
    xReturn
}

#[no_mangle]
pub extern "C" fn vQueueDeleteToC(xQueue: QueueHandle_c) {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    //vQueueDelete(temp); todo:fix dealloc bug
}
/*
#[no_mangle]
pub extern "C" fn xQueueSendFromISRToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvItemToQueue: usize,
    pxHigherPriorityTaskWoken: *mut BaseType,
) -> BaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xQueueSendFromISR(temp.clone(), pvItemToQueue, unsafe {
        &mut *pxHigherPriorityTaskWoken
    });
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn xQueueReceiveFromISRToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvBuffer: usize,
    pxHigherPriorityTaskWoken: *mut BaseType,
) -> BaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xQueueReceiveFromISR(temp.clone(), pvBuffer, unsafe {
        &mut *pxHigherPriorityTaskWoken
    });
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn xQueuePeekFromISRToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvBuffer: usize,
) -> BaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xQueuePeekFromISR(temp.clone(), pvBuffer);
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}*/

pub fn xQueueGenericSendToC(
    mut xQueue: QueueHandle_c,
    pvItemToQueue: usize,
    mut xTicksToWait: TickType,
    xCopyPosition: BaseType,
) -> BaseType {
    let mut xYieldRequired: bool = false;
    let mut xEntryTimeSet: bool = false;
    let mut xTimeout: TimeOut = Default::default();
    loop {
        taskENTER_CRITICAL!();
        let mut xQueue_ = unsafe { Arc::from_raw(xQueue) };

        {
            if xQueue_.read().uxMessagesWaiting < xQueue_.read().uxLength
                || xCopyPosition == queueOVERWRITE
            {
                if cfg!(feature = "configUSE_QUEUE_SETS") {
                    let uxPreviousMessagesWaiting = xQueue_.read().uxMessagesWaiting;
                    xYieldRequired =
                        prvCopyDataToQueue(&mut xQueue_.write(), pvItemToQueue, xCopyPosition);
                    //todo
                } else {
                    xYieldRequired =
                        prvCopyDataToQueue(&mut xQueue_.write(), pvItemToQueue, xCopyPosition);
                    if list_is_empty(&xQueue_.write().xTasksWaitingToReceive) == false {
                        if xTaskRemoveFromEventList(&xQueue_.write().xTasksWaitingToReceive) == true
                        {
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
                xQueue = Arc::into_raw(xQueue_);
                taskEXIT_CRITICAL!();
                return pdPASS as BaseType;
            } else {
                if xTicksToWait == 0 {
                    xQueue = Arc::into_raw(xQueue_);
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

        prvLockQueue!(xQueue_);
        if xTaskCheckForTimeOut(&mut xTimeout, &mut xTicksToWait) == pdFALSE {
            if prvIsQueueFull(&xQueue_) == true {
                vTaskPlaceOnEventList(&xQueue_.write().xTasksWaitingToSend, xTicksToWait);
                prvUnlockQueue(&xQueue_);
                xQueue = Arc::into_raw(xQueue_);
                if vTaskResumeAll() == false {
                    portYIELD_WITHIN_API!();
                }
            } else {
                prvUnlockQueue(&xQueue_);
                xQueue = Arc::into_raw(xQueue_);
                vTaskResumeAll();
            }
        } else {
            prvUnlockQueue(&xQueue_);
            xQueue = Arc::into_raw(xQueue_);
            vTaskResumeAll();
            return errQUEUE_FULL;
        }
    }
}

#[no_mangle]
pub extern "C" fn xQueueReceiveToC(
    mut xQueue: QueueHandle_c,
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
        print("in recv loop");
        let mut xQueue_ = unsafe { Arc::from_raw(xQueue) };
        {
            let uxMessagesWaiting = xQueue_.read().uxMessagesWaiting;
            if uxMessagesWaiting > 0 {
                //TODO:
                pcOriginalReadPosition = xQueue_.read().pcReadFrom;
                if xQueue_.read().uxItemSize > 0 {
                    prvCopyDataFromQueue(&mut xQueue_.write(), pvBuffer);
                }

                xQueue_.write().uxMessagesWaiting = uxMessagesWaiting - 1;

                if cfg!(feature = "configUSE_MUTEXES") {
                    if xQueue_.read().ucQueueType == queueQUEUE_TYPE_MUTEX
                        || xQueue_.read().ucQueueType == queueQUEUE_TYPE_RECURSIVE_MUTEX
                    {
                        xQueue_.write().xMutexHolder = pvTaskIncrementMutexHeldCount();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }

                if list_is_empty(&xQueue_.write().xTasksWaitingToSend) == false {
                    if (xTaskRemoveFromEventList(&xQueue_.write().xTasksWaitingToSend) != false) {
                        queueYIELD_IF_USING_PREEMPTION!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    //list empty
                    mtCOVERAGE_TEST_MARKER!();
                }
                xQueue = Arc::into_raw(xQueue_);
                taskEXIT_CRITICAL!();
                return pdPASS;
            } else {
                if xTicksToWait == 0 {
                    xQueue = Arc::into_raw(xQueue_);
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
        print("ready to suspend");
        vTaskSuspendAll();
        taskEXIT_CRITICAL!();

        prvLockQueue!(xQueue_.clone());
        if xTaskCheckForTimeOut(&mut xTimeOut, &mut xTicksToWait) == pdFALSE {
            print("no timeout");
            if (prvIsQueueEmpty(&xQueue_) != false) {
                print("place on");
                vTaskPlaceOnEventList(&xQueue_.write().xTasksWaitingToReceive, xTicksToWait);
                print("place on complete");
                // /* Unlocking the queue means queue events can effect the
                //  * event list. It is possible that interrupts occurring now
                //  * remove this task from the event list again - but as the
                //  * scheduler is suspended the task will go onto the pending
                //  * ready list instead of the actual ready list. */
                if cfg!(feature = "configUSE_MUTEXES") {
                    if xQueue_.read().ucQueueType == queueQUEUE_TYPE_MUTEX
                        || xQueue_.read().ucQueueType == queueQUEUE_TYPE_RECURSIVE_MUTEX
                    {
                        taskENTER_CRITICAL!();
                        print("inheriting");
                        xInheritanceOccurred =
                            xTaskPriorityInherit(xQueue_.write().xMutexHolder.as_ref());
                        taskEXIT_CRITICAL!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }

                prvUnlockQueue(&xQueue_);
                xQueue = Arc::into_raw(xQueue_);
                if (vTaskResumeAll() == false) {
                    portYIELD_WITHIN_API!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                prvUnlockQueue(&xQueue_);
                xQueue = Arc::into_raw(xQueue_);
                vTaskResumeAll();
            }
        } else {
            print("timeout");
            prvUnlockQueue(&xQueue_);

            if prvIsQueueEmpty(&xQueue_) != false {
                if cfg!(feature = "configUSE_MUTEXES") {
                    if xInheritanceOccurred != pdFALSE {
                        taskENTER_CRITICAL!();
                        let uxHighestWaitingPriority =
                            prvGetDisinheritPriorityAfterTimeout(&xQueue_);
                        vTaskPriorityDisinheritAfterTimeout(
                            xQueue_.write().xMutexHolder.as_ref(),
                            uxHighestWaitingPriority,
                        );
                        taskEXIT_CRITICAL!();
                    }
                }
                xQueue = Arc::into_raw(xQueue_);
                vTaskResumeAll();
                return errQUEUE_EMPTY;
            } else {
                xQueue = Arc::into_raw(xQueue_);
                vTaskResumeAll();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn xQueuePeekToC(
    mut xQueue: QueueHandle_c,
    pvBuffer: usize,
    mut xTicksToWait: TickType,
) -> BaseType {
    let mut xEntryTimeSet: BaseType = pdFALSE;
    let mut xTimeOut: TimeOut = Default::default();
    let mut pcOriginalReadPosition: usize = 0;

    loop {
        taskENTER_CRITICAL!();
        let mut xQueue_ = unsafe { Arc::from_raw(xQueue) };
        {
            let uxMessagesWaiting = xQueue_.read().uxMessagesWaiting;
            if uxMessagesWaiting > 0 {
                //TODO:
                pcOriginalReadPosition = xQueue_.read().pcReadFrom;
                prvCopyDataFromQueue(&mut xQueue_.write(), pvBuffer);
                //different from queuereceive
                // xQueue.uxMessagesWaiting = uxMessagesWaiting - 1;
                /* The data is not being removed, so reset the read pointer. */
                xQueue_.write().pcReadFrom = pcOriginalReadPosition;
                if list_is_empty(&xQueue_.write().xTasksWaitingToReceive) == false {
                    if (xTaskRemoveFromEventList(&xQueue_.write().xTasksWaitingToReceive) != false)
                    {
                        queueYIELD_IF_USING_PREEMPTION!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    //list empty
                    mtCOVERAGE_TEST_MARKER!();
                }
                xQueue = Arc::into_raw(xQueue_);
                taskEXIT_CRITICAL!();
                return pdPASS;
            } else {
                if xTicksToWait == 0 {
                    xQueue = Arc::into_raw(xQueue_);
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

        prvLockQueue!(xQueue_);
        if xTaskCheckForTimeOut(&mut xTimeOut, &mut xTicksToWait) == pdFALSE {
            if (prvIsQueueEmpty(&xQueue_) != false) {
                vTaskPlaceOnEventList(&xQueue_.write().xTasksWaitingToReceive, xTicksToWait);

                prvUnlockQueue(&xQueue_);
                xQueue = Arc::into_raw(xQueue_);
                if (vTaskResumeAll() == false) {
                    portYIELD_WITHIN_API!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                prvUnlockQueue(&xQueue_);
                xQueue = Arc::into_raw(xQueue_);
                vTaskResumeAll();
            }
        } else {
            prvUnlockQueue(&xQueue_);
            let empty = prvIsQueueEmpty(&xQueue_);
            xQueue = Arc::into_raw(xQueue_);
            vTaskResumeAll();
            if empty != false {
                return errQUEUE_EMPTY;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
    }
}
