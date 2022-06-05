//! Semaphore ffi for C

use crate::ffi::queue::*;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::semphr::*;
use crate::kernel::tasks::*;
use crate::{portENTER_CRITICAL, portEXIT_CRITICAL, taskENTER_CRITICAL};
use crate::{vSemaphoreDelete, xSemaphoreCreateBinary};
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::ffi::c_void;
use core::mem::forget;
use core::mem::size_of;
use spin::RwLock;

use super::queue::xQueueGenericSendToC;
use super::queue::QueueHandle_c;

/// The C version of xSemaphoreCreateBinary.
#[no_mangle]
pub extern "C" fn xSemaphoreCreateBinaryToC() -> QueueHandle_c {
    let sem: QueueDefinition = xSemaphoreCreateBinary!();
    let temp = Arc::new(RwLock::new(sem));
    Arc::into_raw(temp)
}

/// The C version of xSemaphoreCreateCounting.
#[no_mangle]
pub extern "C" fn xSemaphoreCreateCountingToC(
    uxMaxCount: UBaseType,
    uxInitialCount: UBaseType,
) -> QueueHandle_c {
    let sem: QueueDefinition = xSemaphoreCreateCounting(uxMaxCount, uxInitialCount);
    let temp = Arc::new(RwLock::new(sem));
    Arc::into_raw(temp)
}

/// The C version of vSemaphoreDelete.
#[no_mangle]
pub extern "C" fn vSemaphoreDeleteToC(xQueue: QueueHandle_c) {
    vQueueDeleteToC(xQueue);
}

/// The C version of xSemaphoreGive.
#[no_mangle]
pub extern "C" fn xSemaphoreGiveToC(mut xQueue: QueueHandle_c) -> BaseType {
    let xReturn = xQueueGenericSendToC(xQueue, 0, semGIVE_BLOCK_TIME, queueSEND_TO_BACK);
    xReturn
}

/// The C version of xSemaphoreTake.
#[no_mangle]
pub extern "C" fn xSemaphoreTakeToC(mut xQueue: QueueHandle_c, xBlockTime: UBaseType) -> BaseType {
    let xReturn = xQueueReceiveToC(xQueue, 0, xBlockTime);
    xReturn
}

/// The C version of xQueueCreateMutex.
#[no_mangle]
pub extern "C" fn xQueueCreateMutexToC(ucQueueType: u8) -> QueueHandle_c {
    let temp = xQueueCreateMutex(ucQueueType);
    Arc::into_raw(temp)
}

/// The C version of prvInitialiseMutex.
#[no_mangle]
pub extern "C" fn prvInitialiseMutexToC(xQueue: QueueHandle_c) {
    taskENTER_CRITICAL!();
    let temp = unsafe { Arc::from_raw(xQueue) };
    prvInitialiseMutex(&temp);
    let xQueue_ = Arc::into_raw(temp);
    portEXIT_CRITICAL!();
}
