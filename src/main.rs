#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]
#![feature(box_into_inner)]
#[allow(dead_code)]
mod kernel;
extern crate alloc;
use alloc::slice;
use alloc::str;
use alloc::sync::Arc;
use alloc::{fmt::format, format};
use core::arch::asm;
use core::{borrow::Borrow, ffi::c_void, mem::size_of};
use kernel::projdefs::{pdPASS, pdTRUE};
use kernel::queue::QueueDefinition;
use kernel::{
    config::*, kernel::*, linked_list::*, portable::*, portmacro::*, queue::*, riscv_virt::*,
    semphr::*, tasks::*, *,
};
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;
mod test_task_param;
use test_task_param::main_test_str;
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
        vSendString("11111 gogogogo!!!(in loop)");
        // vTaskDelay(100);
    }
}
fn task2(t: *mut c_void) {
    let b: i32 = unsafe { *(t as *mut i32) };
    
    vSendString("22222 gogogogo!!!");
    // let s = format!("bbbb={}", b);
    loop {
        // vSendString(&s);
        vTaskDelete(Some(Arc::clone(&(task2handler.as_ref().unwrap()))));
 
        vSendString("22222 gogogogo!!!(in loop)");
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
    // main_test_str();
    main_new_1();
}
pub fn vApplicationIdleHook(){
    vSendString("hook!!!!!");
}
fn task_send(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let ulValueToSend = 100;
    let pcMessage1 = "Transfer1";
    let pcMessage2 = "Transfer2";
    let mut f = 1;
    let mut result: BaseType = 0;
    let s1 = "sending";
    let s2 = "send correct";
    let s3 = "send incorrect";
    let s4 = "send give";
    let temp: &mut QueueDefinition;
    unsafe {
        temp = xQueue.as_mut().unwrap();
    }

    let mut cnt = 0;
    loop {
        testfunc1();

        // xTaskDelayUntil(&mut begin, increment);
        vSendString(&s1);
        //xQueueGenericSend(temp, &ulValueToSend as *const _ as usize, 0,queueSEND_TO_BACK);
        result = xSemaphoreTake!(temp, 0);

        if result == pdTRUE {
            vSendString(&s2);
        } else {
            vSendString(&s3);
            vTaskDelay(5000);
            continue;
        }
        vSendString(&s4);
        xSemaphoreGive!(temp);
        //let s=format!("send:{}",ulValueToSend);
        //vSendString(&s);
        vTaskDelay(5000);

        testfunc2();
        //vSendString("send gogogogo!!!(in loop)");
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
    let s = "take success";
    let s_ = "take fail";
    let temp: &mut QueueDefinition;
    unsafe {
        temp = xQueue.as_mut().unwrap();
    }
    let mut cnt = 0;
    loop {
        testfunc1();
        //xQueueReceive(temp, &ulValueToSend as *const _ as usize, 10);

        result = xSemaphoreTake!(temp, 0);
        //let s=format!("recv:{}",ulValueToSend);
        //vSendString(&s);
        if result == pdTRUE {
            vSendString(&s);
        } else {
            vSendString(&s_);
            continue;
        }

        for i in 0..100000 {
            taskYield();
            //mtCOVERAGE_TEST_MARKER!()
        }
        //ulValueToSend=99;
        //vTaskDelay(100);
        //taskENTER_CRITICAL!();
        xSemaphoreGive!(temp);
        testfunc2();
    }
}

fn task_temp() {
    let s = "temp gogogo";
    loop {
        vSendString(&s);
        vTaskDelay(5000);
    }
}
lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task3handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}
static mut xQueue: Option<QueueDefinition> = None;
pub fn main_new_1() {
    // vSendString("111111");
    print("main new");
    // vSendString("24234234234234");
    let param_a: i32 = 100;
    let param_b: i32 = 200;
    let param1: Param_link = &param_a as *const i32 as usize;
    let param2: Param_link = &param_b as *const i32 as usize;
    let param3: Param_link = 0;
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
    // unsafe {
    //     xQueue = Some(xQueueCreate(1, size_of::<u32>() as u32));
    // }
    unsafe {
        /*xQueue = Some(QueueDefinition::xQueueCreate(
            2,
            size_of::<u32>() as u32,
        ));*/
        xQueue = Some(xQueueCreateMutex(queueQUEUE_TYPE_MUTEX));
    }
    //let s=format!("create1:{}",list_get_num_items(Arc::downgrade(&xQueue.clone().unwrap().read().xTasksWaitingToReceive)))
    /*unsafe {
        let s = format!(
            "create1:{}",
            xQueue
                .clone()
                .unwrap()
                .read()
                .xTasksWaitingToReceive
                .read()
                .ux_number_of_items
        );
        print(&s);
    }*/
    print("task1handler");
    unsafe {
        print("xTaskCreate start");
        let x = print("xTaskCreate 1111");
        xTaskCreate(
            task1 as u32,
            "task1",
            USER_STACK_SIZE as u32,
            Some(param1),
            3,
            Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
        );
        xTaskCreate(
            task2 as u32,
            "task2",
            USER_STACK_SIZE as u32,
            Some(param2),
            3,
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
    /*unsafe {
        let s = format!(
            "create2:{}",
            xQueue
                .clone()
                .unwrap()
                .read()
                .xTasksWaitingToReceive
                .read()
                .ux_number_of_items
        );
        print(&s);
    }*/
    print("start scheduler!!!!!!!!!");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
pub fn testfunc1() {
    mtCOVERAGE_TEST_MARKER!();
}

pub fn testfunc2() {
    mtCOVERAGE_TEST_MARKER!();
}
