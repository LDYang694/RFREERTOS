use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::vSendString;
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::ffi::c_void;
use core::mem::forget;
use core::mem::size_of;
use spin::RwLock;

#[no_mangle]
pub extern "C" fn xQueueCreateToC(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
) -> *const RwLock<QueueDefinition> {
    let mut temp = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(
        uxQueueLength,
        uxItemSize,
    )));
    let s = format!("{:X}", &mut *temp.write() as *mut QueueDefinition as usize);
    vSendString(&s);
    let s_ = format!("{:X}", size_of::<QueueDefinition>());
    vSendString(&s_);
    //forget(temp.clone());
    Arc::into_raw(temp)
}

#[no_mangle]
pub extern "C" fn uxQueueMessagesWaiting(xQueue: *const RwLock<QueueDefinition>) -> UBaseType {
    //forget(xQueue.clone());
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
pub extern "C" fn rustAssert(val: bool) {
    assert!(val);
}

#[no_mangle]
pub extern "C" fn xQueueSendToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvItemToQueue: usize,
    xTicksToWait: TickType,
) -> BaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xQueueSend(temp.clone(), pvItemToQueue, xTicksToWait);
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn xQueueReceiveToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvBuffer: usize,
    mut xTicksToWait: TickType,
) -> BaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xQueueReceive(temp.clone(), pvBuffer, xTicksToWait);
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn xQueuePeekToC(
    xQueue: *mut RwLock<QueueDefinition>,
    pvBuffer: usize,
    mut xTicksToWait: TickType,
) -> BaseType {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xQueuePeek(temp.clone(), pvBuffer, xTicksToWait);
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn vQueueDeleteToC(xQueue: *mut RwLock<QueueDefinition>) {
    let temp: QueueHandle_t = unsafe { Arc::from_raw(xQueue) };
    vQueueDelete(temp);
}
