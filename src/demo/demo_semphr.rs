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

fn task_give(t: *mut c_void) {
    let ulValueToSend: UBaseType = 100;
    loop {
        let result: BaseType;
        vSendString("giving");
        unsafe {
            result = xSemaphoreGive!(xQueue.as_ref().unwrap());
        }
        if result == pdFALSE {
            vSendString("give fail");
        } else {
            vSendString("give success");
        }
    }
}
fn task_take(t: *mut c_void) {
    let mut ulValueReceived: UBaseType = 99;
    let ulExpectedValue = 100;
    loop {
        vSendString("taking");
        let result: BaseType;
        unsafe {
            result = xSemaphoreTake!(xQueue.as_ref().unwrap(), 10);
        }
        if result == pdFALSE {
            vSendString("take fail");
        } else {
            vSendString("take success")
        }
        ulValueReceived = 99;
    }
}

lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}
static mut xQueue: Option<QueueHandle_t> = None;

pub fn main_new() {
    let param1: Param_link = 0;
    let param2: Param_link = 0;
    let param3: Param_link = 0;

    unsafe {
        xQueue = Some(Arc::new(RwLock::new(xSemaphoreCreateBinary!())));
        //xQueue = Some(Arc::new(RwLock::new(xSemaphoreCreateCounting(5,2))));
    }

    unsafe {
        xTaskCreate(
            task_give as usize,
            "task1",
            USER_STACK_SIZE as u32,
            Some(param1),
            2,
            Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
        );
        xTaskCreate(
            task_take as usize,
            "task2",
            USER_STACK_SIZE as u32,
            Some(param2),
            3,
            Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
        );
    }

    print("start scheduler");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
