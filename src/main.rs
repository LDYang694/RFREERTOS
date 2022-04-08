#![no_std]
#![feature(alloc_error_handler)]
#![no_main]

extern crate alloc;

mod allocator;
mod config;
mod linked_list;
mod linked_list_test;
mod ns16550;
mod portable;
mod portmacro;
mod riscv_virt;
mod tasks;

use crate::alloc::sync::{Arc};
use core::arch::global_asm;
use core::ffi::c_void;
use core::include_str;
use core::panic::PanicInfo;
use lazy_static::*;
use linked_list::*;
use riscv_virt::*;
use spin::RwLock;
use tasks::*;
use crate::config::*;
use crate::allocator::init_heap;
use crate::portable::*;

global_asm!(include_str!("start.S"));

lazy_static! {
    //TODO: tmp size
    pub static ref READY_TASK_LISTS: [ListRealLink; 16] = Default::default();
    pub static ref TASK1_STACK:[u32;USER_STACK_SIZE]= [0;USER_STACK_SIZE] ;
    pub static ref TASK2_STACK:[u32;USER_STACK_SIZE]=[0;USER_STACK_SIZE];
    //pub static ref pxCurrentTCB_: RwLock<Option<TaskHandle_t>> = RwLock::new(None);
    pub static ref TCB1_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
    pub static ref TCB2_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
}

// pub static mut pxCurrentTCB:RwLock<Option<TaskHandle_t>> = RwLock::new(None);

// pub static mut TASK1_STACK: &'static mut [u8] = &mut [0; 1000];
// pub static mut TASK2_STACK: &'static mut [u8] = &mut [0; 1000];

static mut task1handler:Option<TaskHandle_t>=None;
static mut task2handler:Option<TaskHandle_t>=None;

fn delay(time: u32) {
    let mut x = 1;
    for i in 0..time {
        x += i;
    }
}
fn task1(t: *mut c_void) {
    let a=0;
    let b=a+1;
    v_send_string("11111 gogogogo!!!");
    
    loop {
        delay(10000);
        unsafe{
            vTaskPrioritySet(task1handler.clone(),1);
        }
        v_send_string("11111 gogogogo!!!(in loop)");
        unsafe{
            vTaskPrioritySet(task1handler.clone(),2);
        }
    }
}
fn task2(t: *mut c_void) {
    let b=0;
    let a=b+1;
    v_send_string("22222 gogogogo!!!");
    
    loop {
        delay(10000);
        unsafe{
            vTaskPrioritySet(task2handler.clone(),1);
        }
        v_send_string("22222 gogogogo!!!(in loop)");
        unsafe{
            vTaskPrioritySet(task2handler.clone(),2);
        }
    }
}

fn tf()
{
    let a=0;
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_heap();
    // ll_test();
    main_new();
    print("begin loop!!!!!");
    loop {}
}

fn main_new() {
    print("main new");
    let param1: Param_link = 0;
    let param2: Param_link = 0;
    let stack1ptr: StackType_t_link =
        &*TASK1_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
            - 4;
    let stack2ptr: StackType_t_link =
        &*TASK2_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
            - 4;

    print("task1handler");
    unsafe{
        task1handler=x_task_create_static(
            task1 as u32,
            "task1",
            USER_STACK_SIZE as u32,
            Some(param1),
            Some(stack1ptr),
            Some(TCB1_p.clone()),
            2
        );
    }
    
    print("task insert");
    v_list_insert_end(&READY_TASK_LISTS[2], (TCB1_p.read().xStateListItem).clone());

    print("task2handler");
    unsafe{
        task2handler=x_task_create_static(
            task2 as u32,
            "task2",
            USER_STACK_SIZE as u32,
            Some(param2),
            Some(stack2ptr),
            Some(TCB2_p.clone()),
            1
        );
    }
    
    print("task insert");
    v_list_insert_end(&READY_TASK_LISTS[1], (TCB2_p.read().xStateListItem).clone());

    print("start scheduler!!!!!!!!!");
    v_task_start_scheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print("R_FreeRTOS paniced!");
    loop {}
}
