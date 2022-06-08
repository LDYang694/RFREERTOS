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
mod tests;
extern crate alloc;
use alloc::sync::Arc;
use core::arch::asm;
use core::ffi::c_void;
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

extern "C" {
    fn main_blinky() -> BaseType;
    fn test_() -> BaseType;
}

pub fn main_new() {
    main_new_1();
    loop {}
}

fn task_send(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let ulValueToSend = 100;
    let pcMessage1 = "Transfer1";
    let pcMessage2 = "Transfer2";
    let mut f = 1;
    let mut result: BaseType = 0;
    let s1 = "sending";

    let mut cnt = 0;
    loop {
        // xTaskDelayUntil(&mut begin, increment);
        //vSendString(&s1);
        //xQueueGenericSend(temp, &ulValueToSend as *const _ as usize, 0,queueSEND_TO_BACK);
        //result=xSemaphoreTake!(temp,0);

        //xSemaphoreGive!(temp);
        //let s=format!("send:{}",ulValueToSend);
        //vSendString(&s);
        let result: BaseType;
        vSendString("sending");
        unsafe {
            //taskENTER_CRITICAL!();
            result = xQueueSend(
                &xQueue.as_ref().unwrap().clone(),
                (&ulValueToSend) as *const BaseType as usize,
                10,
            );
            //taskEXIT_CRITICAL!();
            //
            //result = xSemaphoreGive!(&xQueue.clone().unwrap());
            //
        }
        vSendString("send complete");
    }
}
fn task_rec(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let mut ulValueToSend = 99;
    let ulExpectedValue = 100;
    let pcMessage1 = "success";
    let pcMessage2 = "fail";
    let mut f = 1;
    let mut result: BaseType = 0;
    // vTaskDelay(1000);
    vSendString("receiving");
    let s = "taking";

    let mut cnt = 0;
    loop {
        vSendString("receiving");
        let result: BaseType;
        unsafe {
            //taskENTER_CRITICAL!();
            result = xQueueReceive(
                &xQueue.as_ref().unwrap().clone(),
                (&ulValueToSend) as *const BaseType as usize,
                10,
            );
            //taskEXIT_CRITICAL!();
            //result = xSemaphoreTake!(&xQueue.clone().unwrap(), 0);
        }
        vSendString("receive complete");
    }
}

lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    //pub static ref task3handler: Option<TaskHandle_t> =
    //    Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}
static mut xQueue: Option<QueueHandle_t> = None;
static mut xEvent: Option<EventGroupHandle> = None;
pub fn main_new_1() {
    let param1: Param_link = 0;
    let param2: Param_link = 0;
    let param3: Param_link = 0;

    unsafe {
        xQueue = Some(xQueueCreateMutex(queueQUEUE_TYPE_MUTEX));
        xEvent = Some(Arc::new(RwLock::new(
            EventGroupDefinition::xEventGroupCreate(),
        )));
    }

    print("task1handler");
    unsafe {
        print("xTaskCreate start");
        let x = print("xTaskCreate 1111");
        xTaskCreate(
            task_send as u32,
            "task1",
            USER_STACK_SIZE as u32,
            Some(param1),
            3,
            Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
        );
        xTaskCreate(
            task_rec as u32,
            "task2",
            USER_STACK_SIZE as u32,
            Some(param2),
            3,
            Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
        );
    }

    print("task insert");

    print("start scheduler!!!!!!!!!");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
