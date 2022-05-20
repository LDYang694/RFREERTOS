use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::semphr::*;
use crate::{vSemaphoreDelete, xSemaphoreCreateBinary, xSemaphoreGive, xSemaphoreTake};
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::ffi::c_void;
use core::mem::forget;
use core::mem::size_of;
use spin::RwLock;

use super::queue::QueueHandle_c;

#[no_mangle]
pub extern "C" fn xSemaphoreCreateBinaryToC() -> QueueHandle_c {
    let sem: QueueDefinition = xSemaphoreCreateBinary!();
    let temp = Arc::new(RwLock::new(sem));
    Arc::into_raw(temp)
}

#[no_mangle]
pub extern "C" fn xSemaphoreCreateCountingToC(
    uxMaxCount: UBaseType,
    uxInitialCount: UBaseType,
) -> QueueHandle_c {
    let sem: QueueDefinition = xSemaphoreCreateCounting(uxMaxCount, uxInitialCount);
    let temp = Arc::new(RwLock::new(sem));
    Arc::into_raw(temp)
}

#[no_mangle]
pub extern "C" fn vSemaphoreDeleteToC(xQueue: QueueHandle_c) {
    vSemaphoreDelete!(unsafe { Arc::from_raw(xQueue) });
}

#[no_mangle]
pub extern "C" fn xSemaphoreGiveToC(xQueue: QueueHandle_c) -> BaseType {
    let temp = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xSemaphoreGive!(temp.clone());
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn xSemaphoreTakeToC(xQueue: QueueHandle_c, xBlockTime: UBaseType) -> BaseType {
    let temp = unsafe { Arc::from_raw(xQueue) };
    let xReturn = xSemaphoreTake!(temp.clone(), xBlockTime);
    let xQueue_ = Arc::into_raw(temp);
    xReturn
}

#[no_mangle]
pub extern "C" fn xQueueCreateMutexToC(ucQueueType: u8) -> QueueHandle_c {
    let temp = xQueueCreateMutex(ucQueueType);
    Arc::into_raw(temp)
}

#[no_mangle]
pub extern "C" fn prvInitialiseMutexToC(xQueue: QueueHandle_c) {
    let temp = unsafe { Arc::from_raw(xQueue) };
    prvInitialiseMutex(temp.clone());
    let xQueue_ = Arc::into_raw(temp);
}
