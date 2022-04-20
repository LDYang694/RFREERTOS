#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]
mod kernel;
extern crate alloc;
use alloc::sync::Arc;
use core::{borrow::Borrow, ffi::c_void};
use kernel::{config::*, kernel::*, linked_list::*, riscv_virt::*, tasks::*, portable::*,portmacro::*,*};
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;
use alloc::format;

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
    let mut begin:TickType=0;
    let increment:TickType=100;
    vSendString("22222 gogogogo!!!");
    //vTaskDelete(None);
    loop {
        /*unsafe{
            vTaskPrioritySet(task2handler.clone(),1);
        }*/
        vSendString("22222 gogogogo!!!(in loop)");
        //vTaskResume(task1handler.clone());
        xTaskDelayUntil(&mut begin,increment);
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
lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref task2handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}

pub fn main_new_1() {
    print("main new");
    let param1: Param_link = 0;
    let param2: Param_link = 0;
    let param3: Param_link = 0;
    let stack1ptr: StackType_t_link =
        &*TASK1_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
            - 4;
    let stack2ptr: StackType_t_link =
        &*TASK2_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
            - 4;
    let stack3ptr: StackType_t_link =
        &*TASK3_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
            - 4;

    print("task1handler");
    unsafe {
        print("xTaskCreate start");
        let x = print("xTaskCreate 1111");
        xTaskCreate(
            task1 as u32,
            "task1",
            USER_STACK_SIZE as u32,
            Some(param1),
            1,
            Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
        );
        xTaskCreate(
            task2 as u32,
            "task2",
            USER_STACK_SIZE as u32,
            Some(param2),
            2,
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
