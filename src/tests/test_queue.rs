use crate::ffi::queue::*;
use crate::kernel::allocator::*;
use crate::kernel::config::*;
use crate::kernel::kernel::*;
use crate::kernel::linked_list::ux_list_remove;
use crate::kernel::linked_list::v_list_insert;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::projdefs::pdFALSE;
use crate::kernel::projdefs::pdTRUE;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::tasks::*;
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};

#[no_mangle]
pub extern "C" fn td_task_setFakeTaskPriority(priority: TickType) {
    vTaskPrioritySet(None, priority);
}

#[no_mangle]
pub extern "C" fn td_task_getFakeTaskPriority() -> TickType {
    get_current_tcb().unwrap().uxPriority
}

#[no_mangle]
pub extern "C" fn td_task_addFakeTaskWaitingToSendToQueue(xQueue: QueueHandle_c) {
    ux_list_remove(Arc::downgrade(&get_current_tcb().unwrap().xEventListItem));
    let xQueue_ = unsafe { Arc::from_raw(xQueue) };
    v_list_insert(
        &xQueue_.write().xTasksWaitingToSend,
        &get_current_tcb().unwrap().xEventListItem,
    );
    let temp = Arc::into_raw(xQueue_);
}

#[no_mangle]
pub extern "C" fn td_task_addFakeTaskWaitingToReceiveFromQueue(xQueue: QueueHandle_c) {
    ux_list_remove(Arc::downgrade(&get_current_tcb().unwrap().xEventListItem));
    let xQueue_ = unsafe { Arc::from_raw(xQueue) };
    v_list_insert(
        &xQueue_.write().xTasksWaitingToReceive,
        &get_current_tcb().unwrap().xEventListItem,
    );
    let temp = Arc::into_raw(xQueue_);
}
