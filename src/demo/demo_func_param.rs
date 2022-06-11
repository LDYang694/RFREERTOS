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


lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}
static mut xQueue: Option<QueueHandle_t> = None;


fn task1_str(t: *mut c_void) {
    let a = 0;
    let b = a + 1;
    let s = unsafe { *(t as *const &str) };
    vSendString("11111 gogogogo!!!");

    loop {
        vSendString(s);
        vSendString("11111 gogogogo!!!(in loop)");
    }
}
fn task2_str(t: *mut c_void) {
    let b: i32 = unsafe { *(t as *mut i32) };
    vSendString("22222 gogogogo!!!");
    let s = format!("bbbb={}", b);
    loop {
        vSendString(&s);
        vSendString("22222 gogogogo!!!(in loop)");
    }
}

pub fn main_new() {
    print("main new");
    let param_a: i32 = 100;
    let param_b: i32 = 200;
    let param_str = "testtestteststr";
    let param1: Param_link = &param_str as *const _ as usize;
    let param2: Param_link = &param_b as *const i32 as usize;
    let param3: Param_link = param_str.as_ptr() as usize;



    print("xTaskCreate start");
    let x = print("xTaskCreate 1111");
    xTaskCreate(
        task1_str as usize,
        "task1",
        USER_STACK_SIZE as u32,
        Some(param1),
        2,
        Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
    );
    xTaskCreate(
        task2_str as usize,
        "task2",
        USER_STACK_SIZE as u32,
        Some(param2),
        3,
        Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
    );

    print("start scheduler!!!!!!!!!");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}

