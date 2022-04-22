#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]
#![feature(box_into_inner)]
mod kernel;
extern crate alloc;
use alloc::format;
use alloc::sync::Arc;
use core::{borrow::Borrow, ffi::c_void, mem::size_of};
use kernel::{
    config::*,
    kernel::*,
    linked_list::*,
    portable::*,
    portmacro::*,
    queue::{xQueueCreate, xQueueReceive, xQueueSend, QueueHandle_t},
    riscv_virt::*,
    tasks::*,
    *,
};
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;

#[no_mangle]
pub extern "C" fn main() -> ! {
    main_new();
    loop {}
}

fn delay(time: u32) {
    let mut x = 1;
    for i in 0..time {
        x += i;
    }
}
fn task1(t: *mut c_void) {
    let a = 0;
    let b = a + 1;
    vSendString("11111 gogogogo!!!");

    loop {
        // delay(10000);
        /*unsafe{
            vTaskPrioritySet(task1handler.clone(),1);
        }*/
        vSendString("11111 gogogogo!!!(in loop)");
        //vTaskSuspend(task1handler.clone());
        //vTaskDelay(10);
        /*unsafe{
            vTaskPrioritySet(None,2);
        }*/
        // taskYield();
    }
}
fn task2(t: *mut c_void) {
    let mut begin: TickType = 0;
    let increment: TickType = 100;
    vSendString("22222 gogogogo!!!");
    //vTaskDelete(None);
    loop {
        /*unsafe{
            vTaskPrioritySet(task2handler.clone(),1);
        }*/
        vSendString("22222 gogogogo!!!(in loop)");
        //vTaskResume(task1handler.clone());
        xTaskDelayUntil(&mut begin, increment);
        /*unsafe{
            vTaskPrioritySet(None,2);
        }*/
        // taskYield();
    }
}

fn task3(t: *mut c_void) {
    let b = 0;
    let a = b + 1;
    vSendString("33333 gogogogo!!!");
    loop {
        delay(10000);
        vSendString("33333 gogogogo!!!(in loop)");
    }
}
pub fn main_new() {
    main_new_1();
}
fn task_send(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let ulValueToSend = 100;
    let pcMessage1 = "Transfer1";
    let pcMessage2 = "Transfer2";
    let mut f = 1;
    let mut begin: TickType = 0;
    let increment: TickType = 5;

    loop {
        // xTaskDelayUntil(&mut begin, increment);
        unsafe {
            xQueueSend(xQueue.clone(), &ulValueToSend as *const _ as usize, 0);
        }
        vSendString("send gogogogo!!!(in loop)");
    }
}
fn task_rec(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let ulValueToSend = 99;
    let ulExpectedValue = 100;
    let pcMessage1 = "success";
    let pcMessage2 = "fail";
    let mut f = 1;
    loop {
        unsafe {
            xQueueReceive(xQueue.clone(), &ulValueToSend as *const _ as usize, 10);
        }
        if (ulExpectedValue == ulValueToSend) {
            vSendString(pcMessage1);
        } else {
             let s_ = format!("fail read{:X}", ulValueToSend);
            vSendString(&s_);
        }
    }
}
lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}
static mut xQueue: Option<QueueHandle_t> = None;
pub fn main_new_1() {
    // vSendString("111111");
    print("main new");
    // vSendString("24234234234234");
    let param1: Param_link = 0;
    let param2: Param_link = 0;
    // let param3: Param_link = 0;
    // let stack1ptr: StackType_t_link =
    //     &*TASK1_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
    //         - 4;
    // let stack2ptr: StackType_t_link =
    //     &*TASK2_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
    //         - 4;
    // let stack3ptr: StackType_t_link =
    //     &*TASK3_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
    //         - 4;
    unsafe {
        xQueue = Some(xQueueCreate(1, size_of::<u32>() as u32));
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
            4,
            Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
        );
    }
    //     // xTaskCreate(
    //     //     task2 as u32,
    //     //     "task2",
    //     //     USER_STACK_SIZE as u32,
    //     //     Some(param2),
    //     //     2,
    //     //     Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
    //     // );
    // }
    // unsafe {
    //     let handler = xTaskCreateStatic(
    //         task1 as u32,
    //         "task1",
    //         USER_STACK_SIZE as u32,
    //         Some(param1),
    //         Some(stack1ptr),
    //         Some(TCB1_p.clone()),
    //         2,
    //     );
    //     // task1handler.replace(handler.unwrap());
    // }

    // print("task insert");
    // // v_list_insert_end(&READY_TASK_LISTS[2], (TCB1_p.read().xStateListItem).clone());

    // print("task2handler");
    // // unsafe{
    // //     task2handler=
    // xTaskCreateStatic(
    //     task2 as u32,
    //     "task2",
    //     USER_STACK_SIZE as u32,
    //     Some(param2),
    //     Some(stack2ptr),
    //     Some(TCB2_p.clone()),
    //     2,
    // );
    // // }

    print("task insert");
    // v_list_insert_end(&READY_TASK_LISTS[1], (TCB2_p.read().xStateListItem).clone());

    // print("task3handler");
    // x_task_create_static(
    //     task3 as u32,
    //     "task3",
    //     USER_STACK_SIZE as u32,
    //     Some(param3),
    //     Some(stack3ptr),
    //     Some(TCB3_p.clone()),
    // );
    // print("task insert");
    // v_list_insert_end(&READY_TASK_LISTS[2], (TCB3_p.read().xStateListItem).clone());

    print("start scheduler!!!!!!!!!");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
