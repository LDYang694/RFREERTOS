#![no_std]
// #![feature(alloc_error_handler)]
// #![no_main]

extern crate alloc;



use crate::allocator::init_heap;
use crate::config::*;
use crate::linked_list::*;
use crate::portable::*;
use crate::riscv_virt::*;
use crate::tasks::*;
use alloc::sync::Arc;
use core::arch::global_asm;
use core::ffi::c_void;
use core::include_str;
use core::panic::PanicInfo;
use lazy_static::*;
use spin::RwLock;

global_asm!(include_str!("start.S"));

lazy_static! {
    //TODO: tmp size
    pub static ref READY_TASK_LISTS: [ListRealLink; 16] = Default::default();
    pub static ref DELAYED_TASK_LIST: ListRealLink = Default::default();
    pub static ref OVERFLOW_DELAYED_TASK_LIST: ListRealLink = Default::default();
    pub static ref SUSPENDED_TASK_LIST: ListRealLink = Default::default();
    //TODO:tmp use
    pub static ref CURRENT_TCB: RwLock<Option<TaskHandle_t>> = RwLock::new(None);
    //todo: overflow task list
    pub static ref TASK1_STACK:[u32;USER_STACK_SIZE]= [0;USER_STACK_SIZE] ;
    pub static ref TASK2_STACK:[u32;USER_STACK_SIZE]=[0;USER_STACK_SIZE];
    pub static ref TASK3_STACK:[u32;USER_STACK_SIZE]=[0;USER_STACK_SIZE];
    pub static ref IDLE_STACK:[u32;USER_STACK_SIZE]=[0;USER_STACK_SIZE];
    //pub static ref pxCurrentTCB_: RwLock<Option<TaskHandle_t>> = RwLock::new(None);
    pub static ref TCB1_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
    pub static ref TCB2_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
    pub static ref TCB3_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
    pub static ref IDLE_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
}
#[macro_export]
macro_rules! get_current_task_handle {
    () => {
        crate::CURRENT_TCB
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    };
}
pub enum SchedulerState {
    Not_Started,
    Suspended,
    Running,
}


#[no_mangle]
pub extern "C" fn kernel_init() {
    init_heap();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print("R_FreeRTOS paniced!");
    loop {}
}
