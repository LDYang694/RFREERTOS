#![no_std]
#![feature(alloc_error_handler)]
#![no_main]

extern crate alloc;

mod ns16550;
mod riscv_virt;
mod allocator;
mod linked_list;
mod linked_list_test;
mod portmacro;
mod portable;
mod config;

use core::arch::global_asm;
use core::include_str;
use core::panic::PanicInfo;

use allocator::HeapAlloc;
use linked_list_test::ll_test;
use portable::x_port_start_scheduler;
// use buddy_system_allocator::LockedHeap;

global_asm!(include_str!("start.S"));

pub const KERNEL_HEAP_SIZE: usize = 0x8000;

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_heap();
    x_port_start_scheduler();
    ll_test();
    loop {}
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
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
        // DYNAMIC_ALLOCATOR
        //     .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: HeapAlloc = HeapAlloc{};
// static DYNAMIC_ALLOCATOR: LockedHeap::<1> = LockedHeap::<1>::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}
