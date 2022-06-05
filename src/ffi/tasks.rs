//! Task ffi for C

extern crate libc;

use crate::ffi::ffi::get_str_from_cchar;
use crate::kernel::portmacro::*;
use crate::kernel::tasks::*;
use crate::{portENTER_CRITICAL, portEXIT_CRITICAL};
use alloc::sync::Arc;
use core::ffi::c_void;
use spin::RwLock;

pub type TaskHandle_c = *const RwLock<tskTaskControlBlock>;

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/// Malloc new task handle from Rust and return to C.<br>
#[no_mangle]
pub fn get_task_handle() -> TaskHandle_c {
    let temp = Arc::new(RwLock::new(tskTaskControlBlock::default()));
    Arc::into_raw(temp)
}

/// The C version of xTaskCreateStatic.<br>
/// pxTaskBuffer must be initialized with get_task_handle() unless it's NULL.
#[no_mangle]
pub extern "C" fn xTaskCreateStaticToC(
    pxTaskCode: UBaseType,
    pcName: usize,
    ulStackDepth: UBaseType,
    pvParameters: usize,
    puxStackBuffer: usize,
    mut pxTaskBuffer: TaskHandle_c,
    uxPriority: UBaseType,
) -> TaskHandle_c {
    if (pxTaskBuffer as usize) == 0 {
        pxTaskBuffer = get_task_handle();
    }
    let name = get_str_from_cchar(pcName);
    let handle: TaskHandle_t = xTaskCreateStatic(
        pxTaskCode,
        &name,
        ulStackDepth,
        Some(pvParameters),
        Some(puxStackBuffer),
        unsafe { Some(&Arc::from_raw(pxTaskBuffer)) },
        uxPriority,
    )
    .unwrap();
    handle.write().build_from_c = true;
    let owner_c: TaskHandle_c = Arc::as_ptr(&handle);
    handle.write().xEventListItem.write().pv_owner_c = owner_c as usize;
    handle.write().xStateListItem.write().pv_owner_c = owner_c as usize;
    let xReturn = Arc::into_raw(handle);
    xReturn
}

/// The C version of xTaskCreate.<br>
/// pxCreatedTask must be initialized with get_task_handle() unless it's NULL.
#[no_mangle]
pub extern "C" fn xTaskCreateToC(
    pxTaskCode: UBaseType,
    pcName: usize,
    ulStackDepth: UBaseType,
    pvParameters: usize,
    uxPriority: UBaseType,
    mut pxCreatedTask: TaskHandle_c,
) -> BaseType {
    if (pxCreatedTask as usize) == 0 {
        pxCreatedTask = get_task_handle();
    }
    let name = get_str_from_cchar(pcName);
    let temp = unsafe { Arc::from_raw(pxCreatedTask) };

    let xReturn: BaseType = xTaskCreate(
        pxTaskCode,
        &name,
        ulStackDepth,
        Some(pvParameters),
        uxPriority,
        Some(temp.clone()),
    );
    temp.write().build_from_c = true;
    let owner_c: TaskHandle_c = Arc::as_ptr(&temp);
    temp.write().xEventListItem.write().pv_owner_c = owner_c as usize;
    temp.write().xStateListItem.write().pv_owner_c = owner_c as usize;
    let pxCreatedTask_ = Arc::into_raw(temp);
    xReturn
}

/// The C version of vTaskSuspend.
#[no_mangle]
pub extern "C" fn vTaskSuspendToC(xTaskToSuspend_: TaskHandle_c) {
    if xTaskToSuspend_ as usize == 0 {
        vTaskSuspend(None);
    } else {
        let temp = unsafe { Some(Arc::from_raw(xTaskToSuspend_)) };
        vTaskSuspend(temp.clone());
        let xTaskToSuspend = Arc::into_raw(temp.unwrap());
    }
}

/// The C version of vTaskResume.
#[no_mangle]
pub extern "C" fn vTaskResumeToC(xTaskToResume_: TaskHandle_c) {
    if xTaskToResume_ as usize == 0 {
        vTaskSuspend(None);
    } else {
        let temp = unsafe { Some(Arc::from_raw(xTaskToResume_)) };
        vTaskSuspend(temp.clone());
        let xTaskToResume = Arc::into_raw(temp.unwrap());
    }
}

/// The C version of taskENTER_CRITICAL.
#[no_mangle]
pub extern "C" fn taskENTER_CRITICAL_ToC() {
    portENTER_CRITICAL!();
}

/// The C version of taskEXIT_CRITICAL.
#[no_mangle]
pub extern "C" fn taskEXIT_CRITICAL_ToC() {
    portEXIT_CRITICAL!();
}

/// Return xTickCount to C program.
#[no_mangle]
pub extern "C" fn xTaskGetTickCountToC() -> TickType {
    return unsafe { xTickCount };
}
