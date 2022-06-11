#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]
#![feature(box_into_inner)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#[macro_use]
mod ffi;
#[allow(dead_code)]
#[macro_use]
mod kernel;
#[macro_use]
mod portable;
// mod tests;
extern crate alloc;
use alloc::format;
use alloc::sync::Arc;
use core::arch::asm;
use core::ffi::c_void;
use core::intrinsics::size_of;
use kernel::projdefs::{pdFALSE, pdTRUE};
use kernel::{config::*, event_group::*, queue::*, semphr::*, tasks::*, *};
use lazy_static::lazy_static;
use portable::portmacro::*;
use portable::portmacro::*;
use portable::riscv_virt::*;
use spin::RwLock;

#[no_mangle]
pub extern "C" fn main() -> ! {
    main_new();
    loop {}
}

fn task_high_priority(t: *mut c_void) {
    loop {
        vSendString("high priority task running");
        vTaskDelay(10);
    }
}
fn task_low_priority(t: *mut c_void) {
    loop {
        vSendString("low priority task running");
    }
}

lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}

pub fn main_new() {
    let param1: Param_link = 0;
    let param2: Param_link = 0;

    xTaskCreate(
        task_high_priority as usize,
        "task_high_priority",
        USER_STACK_SIZE as u32,
        Some(param1),
        3,
        Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
    );
    xTaskCreate(
        task_low_priority as usize,
        "task_low_priority ",
        USER_STACK_SIZE as u32,
        Some(param2),
        2,
        Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
    );

    print("start scheduler");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
