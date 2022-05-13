use crate::{kernel::config::KERNEL_HEAP_SIZE, portDISABLE_INTERRUPTS, portENABLE_INTERRUPTS};
use buddy_system_allocator::LockedHeap;
use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;

use super::riscv_virt::{print, vSendString};
use super::tasks::{vTaskEnterCritical, vTaskExitCritical};

// pub fn init_heap() {
//     static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
//     unsafe {
//         DYNAMIC_ALLOCATOR
//             .lock()
//             .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
//     }
// }
pub fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .Buddy_System_Allocator
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[global_allocator]
// static DYNAMIC_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();
static DYNAMIC_ALLOCATOR: SimpleAllocator = SimpleAllocator::empty();
#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}

struct SimpleAllocator {
    Buddy_System_Allocator: LockedHeap<32>,
}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        vTaskEnterCritical();
        // print("alloc");
        let x = self.Buddy_System_Allocator.alloc(layout);
        vTaskExitCritical();
        x
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        vTaskEnterCritical();
        // vSendString("dealloc");
        self.Buddy_System_Allocator.dealloc(ptr, layout);
        vTaskExitCritical();
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.Buddy_System_Allocator.alloc_zeroed(layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.Buddy_System_Allocator.realloc(ptr, layout, new_size)
    }
}
impl SimpleAllocator {
    pub const fn empty() -> Self {
        SimpleAllocator {
            Buddy_System_Allocator: LockedHeap::<32>::empty(),
        }
    }
    // pub unsafe fn init(&mut self, start: usize, size: usize) {
    //     self.Buddy_System_Allocator.lock().init(start, size)
    // }
}
