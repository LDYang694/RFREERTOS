extern crate libc;

use crate::ffi::ffi::get_str_from_cchar;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::tasks::*;
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::ffi::c_void;
use core::mem::forget;
use core::mem::size_of;
use spin::RwLock;

pub type TaskHandle_c = *const RwLock<tskTaskControlBlock>;

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub fn get_task_handle() -> TaskHandle_c {
    let temp = Arc::new(RwLock::new(tskTaskControlBlock::default()));
    Arc::into_raw(temp)
}

#[no_mangle]
pub extern "C" fn xTaskCreateStaticToC(
    pxTaskCode: UBaseType,
    pcName: usize,
    ulStackDepth: UBaseType,
    pvParameters: usize,
    puxStackBuffer: usize,
    pxTaskBuffer: TaskHandle_c,
    uxPriority: UBaseType,
) -> TaskHandle_c {
    let name = get_str_from_cchar(pcName);
    let handle: Option<TaskHandle_t> = xTaskCreateStatic(
        pxTaskCode,
        &name,
        ulStackDepth,
        Some(pvParameters),
        Some(puxStackBuffer),
        unsafe { Some(Arc::from_raw(pxTaskBuffer)) },
        uxPriority,
    );
    Arc::into_raw(handle.unwrap())
}

#[no_mangle]
pub extern "C" fn xTaskCreateToC(
    pxTaskCode: UBaseType,
    pcName: usize,
    ulStackDepth: UBaseType,
    pvParameters: usize,
    uxPriority: UBaseType,
    pxCreatedTask: TaskHandle_c,
) -> BaseType {
    let name = get_str_from_cchar(pcName);
    print(&name);
    /*unsafe {
        let val = RwLock::new(tskTaskControlBlock::default());
        memcpy(
            pxCreatedTask as *mut c_void,
            &val as TaskHandle_c as *const c_void,
            size_of::<RwLock<tskTaskControlBlock>>(),
        );
    }*/
    let temp = unsafe { Arc::from_raw(pxCreatedTask) };

    let xReturn: BaseType = xTaskCreate(
        pxTaskCode,
        &name,
        ulStackDepth,
        Some(pvParameters),
        uxPriority,
        Some(temp.clone()),
    );
    let pxCreatedTask_ = Arc::into_raw(temp);
    print("create complete");
    xReturn
}

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
