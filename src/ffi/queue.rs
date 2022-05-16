use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::*;
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
    //vQueueDelete(temp); todo:fix dealloc bug
}

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
}
