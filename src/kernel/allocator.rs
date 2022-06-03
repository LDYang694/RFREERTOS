//! Global Allocatot Definition <br>
//! use buddy_system_allocator and wrap as SimpleAllocator
use super::tasks::{vTaskEnterCritical, vTaskExitCritical};
use buddy_system_allocator::LockedHeap;

use crate::kernel::{config::{KERNEL_HEAP_SIZE}};
// use alloc::alloc::{GlobalAlloc, Layout};

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;


use super::riscv_virt::{print, vSendString};

/// INITIAL Start should init_heap first
pub fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .Buddy_System_Allocator
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    print("heap initialized.")
}

#[global_allocator]
/// DYNAMIC_ALLOCATOR as global_allocator
static DYNAMIC_ALLOCATOR: SimpleAllocator = SimpleAllocator::empty();

#[alloc_error_handler]
/// alloc_error_handler function
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}

/// Critical Wrapped Buddy System Allocator
struct SimpleAllocator {
    Buddy_System_Allocator: LockedHeap<32>,
}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        vTaskEnterCritical();
        let x = self.Buddy_System_Allocator.alloc(layout);
        vTaskExitCritical();
        x
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        vTaskEnterCritical();
        self.Buddy_System_Allocator.dealloc(ptr, layout);
        vTaskExitCritical();
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        vTaskEnterCritical();
        let x = self.Buddy_System_Allocator.alloc_zeroed(layout);
        vTaskExitCritical();
        x
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        vTaskEnterCritical();
        let x = self.Buddy_System_Allocator.realloc(ptr, layout, new_size);
        vTaskExitCritical();
        x
    }
}

impl SimpleAllocator {
    pub const fn empty() -> Self {
        SimpleAllocator {
            Buddy_System_Allocator: LockedHeap::<32>::empty(),
        }
    }
}
