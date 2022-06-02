#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
#![allow(non_snake_case)]
// use core::panic::PanicInfo;
// pub extern "C" fn main() -> ! {
//     rust_main();
//     loop{}
// }
mod kernel;
extern crate alloc;
use alloc::{sync::Arc, string::ToString};
use core::{borrow::Borrow, ffi::c_void};
use kernel::{config::*, kernel::*, riscv_virt::*, linked_list::*, portable::*, tasks::*, *};
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;
use alloc::string::String;
extern "C"{
	fn _putchar(ch: u8);
    fn sdelay(us: u64);
    fn all_interrupt_enable();
    fn clint_timer_init();
}

static A: u32 = 9;

#[no_mangle]
pub extern "C" fn main_rust() {
    main_new_1();
    loop{}


    print("enter main_rust");
    print(&(8.to_string()));
    print(&(A.to_string()));
    let ptr: usize = &String::from("77") as *const String as usize;
    print(&(ptr as u64).to_string());
    let mut stack1ptr: usize =
        &*TASK1_STACK as *const [usize; USER_STACK_SIZE] as *const usize as usize + USER_STACK_SIZE * 8
            - 8;
    print(&"1111");
    let stack2ptr: usize =
        &*TASK2_STACK as *const [usize; USER_STACK_SIZE] as *const usize as usize + USER_STACK_SIZE * 8
            - 8;
    let stack3ptr: usize =
        &*TASK3_STACK as *const [usize; USER_STACK_SIZE] as *const usize as usize + USER_STACK_SIZE * 8
            - 8;
    stack1ptr = stack1ptr + 1;
    print("qq");
    if(stack1ptr > 1){
        print("pp");
    }
    print(&(stack1ptr as u64).to_string());
    // print(&AAA.to_string());
    print("22222");
    // unsafe{
    //     _putchar(('r' as u8));
    //     _putchar(('u' as u8));
    //     _putchar(('s' as u8));
    //     _putchar(('t' as u8));
    //     _putchar(('\n' as u8));
    //     _putchar(('\r' as u8));
    //     loop{
    //         _putchar(('l' as u8));
    //         _putchar(('o' as u8));
    //         _putchar(('o' as u8));
    //         _putchar(('p' as u8));
    //         _putchar(('\n' as u8));
    //         _putchar(('\r' as u8));
    //         sdelay(1000000);
    //     }

    // }
    loop{}
}
fn task1(t: *mut c_void) {
    let mut a = 0;
    let b = a + 1;
    let c = a + 3;
    print("yyy");
    vSendString("11111 gogogogo!!!");

    loop {
        // delay(10000);
        /*unsafe{
            vTaskPrioritySet(task1handler.clone(),1);
        }*/
        vSendString("11111 gogogogo!!!(in loop)");
        // vTaskSuspend(task1handler.clone());
        // vTaskDelay(10);
        /*unsafe{
            vTaskPrioritySet(None,2);
        }*/
        // taskYield();
    }
}
fn task2(t: *mut c_void) {
    let b = 0;
    let a = b + 1;
    vSendString("22222 gogogogo!!!");
    //vTaskDelete(None);
    loop {
        /*unsafe{
            vTaskPrioritySet(task2handler.clone(),1);
        }*/
        vSendString("22222 gogogogo!!!(in loop)");
        // vTaskResume(task1handler.clone());
        /*unsafe{
            vTaskPrioritySet(None,2);
        }*/
        // taskYield();
    }
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
        &*TASK1_STACK as *const [usize; USER_STACK_SIZE] as *const usize as usize + USER_STACK_SIZE * 8
            - 8;
    let stack2ptr: StackType_t_link =
        &*TASK2_STACK as *const [usize; USER_STACK_SIZE] as *const usize as usize + USER_STACK_SIZE * 8
            - 8;
    let stack3ptr: StackType_t_link =
        &*TASK3_STACK as *const [usize; USER_STACK_SIZE] as *const usize as usize + USER_STACK_SIZE * 8
            - 8;
    print("task1handler");
    unsafe {
        print("xTaskCreate start");
        let x = print("xTaskCreate 1111");
        match &*task1handler {
            Some(x)=>{print("some");}
            None=>{print("none");}
        }
        print("test!");
        let temp=Arc::clone(&(task1handler.as_ref().unwrap()));
        xTaskCreate(
            task1 as usize,
            "task1",
            USER_STACK_SIZE as u32,
            Some(param1),
            2,
            Some(temp),
        );
        xTaskCreate(
            task2 as usize,
            "task2",
            USER_STACK_SIZE as u32,
            Some(param2),
            2,
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

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     loop {}
// }
