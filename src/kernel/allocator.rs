use buddy_system_allocator::LockedHeap;
use crate::kernel::{config::{KERNEL_HEAP_SIZE}, riscv_virt::print};
// use alloc::alloc::{GlobalAlloc, Layout};

pub fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    print("heap initialized.")
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();
// static DYNAMIC_ALLOCATOR: HeapAlloc = HeapAlloc{};

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}

// static mut START_ADDR: usize = 0;
// static mut MAX_SIZE: usize = 0;
// static mut USED_SIZE: usize = 0;
// pub struct HeapAlloc{}
// unsafe impl GlobalAlloc for HeapAlloc {
//     unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
//         let alloc_ptr = (START_ADDR + USED_SIZE) as *mut u8;
//         USED_SIZE += _layout.size();
//         alloc_ptr
//     }
//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
//     }
// }

// impl HeapAlloc {
//     pub fn init(&self, _start_addr: usize, _max_size: usize){
//         unsafe {
//             START_ADDR = _start_addr;
//             MAX_SIZE = _max_size;
//             USED_SIZE = 0;
//         }
//     }
// }
