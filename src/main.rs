#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]
#![feature(box_into_inner)]
#[allow(dead_code)]
mod kernel;
extern crate alloc;
use alloc::sync::Arc;
use alloc::{fmt::format, format};
use kernel::projdefs::{pdPASS, pdTRUE, pdFAIL, pdFALSE};
use core::{borrow::Borrow, ffi::c_void, mem::size_of};
use kernel::queue::QueueDefinition;
use kernel::{
    config::*,
    kernel::*,
    linked_list::*,
    portable::*,
    portmacro::*,
    queue::*,
    riscv_virt::*,
    tasks::*,
    semphr::*,
    event_group::*,
    *
};
use core::arch::asm;
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

pub fn testfunc1(){
    mtCOVERAGE_TEST_MARKER!();
}

pub fn testfunc2(){
    mtCOVERAGE_TEST_MARKER!();
}

fn task_send(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let ulValueToSend = 100;
    let pcMessage1 = "Transfer1";
    let pcMessage2 = "Transfer2";
    let mut f = 1;
    let mut result:BaseType=0;
    let s1="sending";
    
        let mut cnt=0;
        loop {

            // xTaskDelayUntil(&mut begin, increment);
            //vSendString(&s1);
                //xQueueGenericSend(temp, &ulValueToSend as *const _ as usize, 0,queueSEND_TO_BACK);
            //result=xSemaphoreTake!(temp,0);
                
            //xSemaphoreGive!(temp);
            //let s=format!("send:{}",ulValueToSend);
            //vSendString(&s);
            vSendString(&s1);
            unsafe{

                let temp=xEvent.as_mut().unwrap();
                xEventGroupWaitBits(temp, 1, pdTRUE, 
                pdFALSE, 100);

            }
            
        }
}
fn task_rec(t: *mut c_void) {
    let mut xNextWakeTime: TickType;
    let mut ulValueToSend = 99;
    let ulExpectedValue = 100;
    let pcMessage1 = "success";
    let pcMessage2 = "fail";
    let mut f = 1;
    let mut result:BaseType=0;
    // vTaskDelay(1000);
    vSendString("receiving");
    let s="taking";

    
    let mut cnt=0;
    loop {
        vSendString(&s);
        unsafe{

            let temp=xEvent.as_mut().unwrap();
            xEventGroupSetBits(temp, 1);

        }
    }       
}

fn task_temp(){
    let s="temp gogogo";
    loop{
        vSendString(&s);
        vTaskDelay(5000);
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
static mut xQueue: Option<QueueDefinition> = None;
static mut xEvent: Option<EventGroupDefinition> = None;
pub fn main_new_1() {
    // vSendString("111111");
    print("main new");
    // vSendString("24234234234234");
    let param1: Param_link = 0;
    let param2: Param_link = 0;
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
        xQueue=Some(xQueueCreateMutex(queueQUEUE_TYPE_MUTEX));
        xEvent=Some(EventGroupDefinition::xEventGroupCreate() )  ;
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
            2,
            Some(Arc::clone(&(task2handler.as_ref().unwrap()))),
        );
        /*xTaskCreate(
            task_temp as u32,
            "task3",
            USER_STACK_SIZE as u32,
            Some(param3),
            2,
            Some(Arc::clone(&(task3handler.as_ref().unwrap()))),
        );*/
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
