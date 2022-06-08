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
lazy_static! {
    pub static ref TASK1_STACK: [usize; USER_STACK_SIZE] = [0; USER_STACK_SIZE];
    pub static ref TASK2_STACK: [usize; USER_STACK_SIZE] = [0; USER_STACK_SIZE];
    pub static ref TCB1_p: TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
    pub static ref TCB2_p: TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
}
pub fn main_test_static() {
    let param_a: i32 = 100;
    let param_b: i32 = 200;
    let param_str = "testtestteststr";
    let param1: Param_link = &param_str as *const _ as usize;
    let param2: Param_link = &param_b as *const i32 as usize;
    let param3: Param_link = param_str.as_ptr() as usize;

    let arch = size_of::<usize>();
    let stack1ptr: StackType_t_link = &*TASK1_STACK as *const [usize; USER_STACK_SIZE] as *const u32
        as usize
        + USER_STACK_SIZE * arch
        - arch;
    let stack2ptr: StackType_t_link = &*TASK2_STACK as *const [usize; USER_STACK_SIZE] as *const u32
        as usize
        + USER_STACK_SIZE * arch
        - arch;
    xTaskCreateStatic(
        task1_str as usize,
        "task1",
        USER_STACK_SIZE as u32,
        Some(param1),
        Some(stack1ptr),
        Some(&TCB1_p.clone()),
        3,
    );
    xTaskCreateStatic(
        task2_str as usize,
        "task2",
        USER_STACK_SIZE as u32,
        Some(param2),
        Some(stack2ptr),
        Some(&TCB2_p.clone()),
        3,
    );

    print("start scheduler!!!!!!!!!");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
