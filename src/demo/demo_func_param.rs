extern crate alloc;
use crate::kernel;
use crate::portable::riscv_virt::*;
use crate::task1handler;
use crate::task2handler;
use crate::xQueue;
use alloc::slice;
use alloc::str;
use alloc::sync::Arc;
use alloc::{fmt::format, format};
use core::arch::asm;
use core::{borrow::Borrow, ffi::c_void, mem::size_of};
use kernel::projdefs::{pdPASS, pdTRUE};
use kernel::queue::QueueDefinition;
use kernel::{
    config::*, kernel::*, linked_list::*, portable::*, portmacro::*, queue::*, semphr::*, tasks::*,
    *,
};
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;

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

pub fn main_test_str() {
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
