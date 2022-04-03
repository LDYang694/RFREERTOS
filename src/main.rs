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

use crate::alloc::sync::{Arc, Weak};
use alloc::boxed::Box;
use alloc::format;
use allocator::HeapAlloc;
use buddy_system_allocator::LockedHeap;
use core::arch::global_asm;
use core::ffi::c_void;
use core::include_str;
use core::panic::PanicInfo;
use lazy_static::*;
use linked_list::*;
use linked_list_test::*;
use portable::*;
use riscv_virt::*;
use spin::RwLock;
use tasks::*;
// use buddy_system_allocator::LockedHeap;

global_asm!(include_str!("start.S"));

pub const KERNEL_HEAP_SIZE: usize = 0x400000;

pub const user_stack_size: usize = 0x10000;

lazy_static! {
    //TODO: tmp size
    pub static ref READY_TASK_LISTS: [ListRealLink; 16] = Default::default();
    pub static ref TASK1_STACK:[u32;user_stack_size]= [0;user_stack_size] ;
    pub static ref TASK2_STACK:[u32;user_stack_size]=[0;user_stack_size];
    //pub static ref pxCurrentTCB_: RwLock<Option<TaskHandle_t>> = RwLock::new(None);
    pub static ref TCB1_p:TCB_t_link = Arc::new(RwLock::new(TCB_t::default()));
}

// pub static mut pxCurrentTCB:RwLock<Option<TaskHandle_t>> = RwLock::new(None);

// pub static mut TASK1_STACK: &'static mut [u8] = &mut [0; 1000];
// pub static mut TASK2_STACK: &'static mut [u8] = &mut [0; 1000];
fn task1(t: *mut c_void) {
    vSendString("tsask1 gogogogo!!!");
    loop {
        vSendString("tsask1 gogogogo!!!(in loop)");
    }
}
fn task2(t: *mut c_void) {}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_heap();
    // ll_test();
    main_new();
    print("begin loop!!!!!");
    loop {}
}

fn main_new() {
    // let Task1TCB = TCB_t::default();
    print("main new");
    let Task2TCB = TCB_t::default();
    print("task2tcb");
    let param1: Param_link = 0;
    let param2: Param_link = 0;
    let Stack1ptr: StackType_t_link =
        &*TASK1_STACK as *const [u32; user_stack_size] as *const u32 as usize + user_stack_size * 4
            - 4;
    let Stack2ptr: StackType_t_link =
        &*TASK2_STACK as *const [u32; user_stack_size] as *const u32 as usize + user_stack_size * 4
            - 4;
    //println!("{:?}", *TASK1_STACK);
    // let TCB1_p = Arc::new(RwLock::new(*Task1TCB));
    // let TCB2_p = Arc::new(RwLock::new(Task2TCB));
    let task1_box: Box<fn(*mut c_void)> = Box::new(task1);
    let taks1_fn: TaskFunction_t = task1_box.as_ref() as *const fn(*mut c_void) as TaskFunction_t;
    print("task1handler");
    let Task1Handler = xTaskCreateStatic(
        task1 as u32,
        "task1",
        user_stack_size as u32,
        Some(param1),
        Some(Stack1ptr),
        Some(TCB1_p.clone()),
    );

    print("task insert");
    v_list_insert_end(&READY_TASK_LISTS[1], (TCB1_p.read().xStateListItem).clone());
    // let list: List<u32> = List::new();
    //println!("{:?}", READY_TASK_LISTS[1]);
    let a: ListItemT = ListItemT::default();
    let mut l: ListT = ListT::default();
    // let a_p = Arc::new(RwLock::new(a));
    // let l_p = Arc::new(RwLock::new(l));
    // let a_p2 = Arc::new(RwLock::new(ListItemT::new(2)));
    // let a_p3 = Arc::new(RwLock::new(ListItemT::new(3)));
    // let a_p5 = Arc::new(RwLock::new(ListItemT::new(5)));
    // // v_list_insert_end(&l_p, a_p.clone());
    // v_list_insert(&l_p, a_p2.clone());
    // v_list_insert(&l_p, a_p3.clone());

    // let a_p4 = Arc::new(RwLock::new(ListItemT::new(4)));
    // v_list_insert(&l_p, a_p4.clone());
    // ux_list_remove(Arc::downgrade(&a_p2.clone()));
    // v_list_insert(&l_p, a_p5.clone());
    // l.v_list_insert_end(Arc::downgrade(&Arc::new(RefCell::new(a))));
    // println!("{:?}", a);
    // println!("{:?}", l);
    // println!(
    //     "a_p strong = {}, weak = {}",
    //     Arc::strong_count(&a_p),
    //     Arc::weak_count(&a_p),
    // );
    // println!("{:?}",*READY_TASK_LISTS);
    // println!("Hello, world!");
    print("start scheduler!!!!!!!!!");
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
pub fn set_current_tcb_test(tcb: Option<*const tskTaskControlBlock>) {
    unsafe {
        // pxCurrentTCB_ = Some((&*TCB1_p.read()));
        pxCurrentTCB_ = tcb;
    }
}

pub fn vTaskStartScheduler() {
    //*pxCurrentTCB_.write() = Some(TCB1_p.clone());
    unsafe{xSchedulerRunning = pdTRUE!();}
    set_current_tcb_test(Some((&*TCB1_p.read())));
    // unsafe {
    //     pxCurrentTCB_ = Some((&*TCB1_p.read()));
    //     let s = format!(
    //         "TCB1_p:{:X}",
    //         &(*TCB1_p.read()) as *const tskTaskControlBlock as usize
    //     );
    //     vSendString(&s);
    // }
    // pxCurrentTCB=&(*CURRENT_TCB.read().unwrap().as_ref().clone().read()) as * const tskTaskControlBlock as usize;
    if x_port_start_scheduler() != pdFALSE!() {
        panic!("error scheduler!!!!!!");
    }
}
#[macro_export]
macro_rules! pdFALSE {
    () => {
        false
    };
}

#[macro_export]
macro_rules! pdTRUE {
    () => {
        true
    };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // let mut host_stderr = HStderr::new();

    // // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    // writeln!(host_stderr, "{}", info).ok();

    loop {}
}

fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
        // DYNAMIC_ALLOCATOR
        //     .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[global_allocator]
// static DYNAMIC_ALLOCATOR: HeapAlloc = HeapAlloc{};
static DYNAMIC_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}
