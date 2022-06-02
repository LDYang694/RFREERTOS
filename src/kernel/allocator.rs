use buddy_system_allocator::LockedHeap;
use crate::kernel::{config::{KERNEL_HEAP_SIZE}, riscv_virt::print};

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

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}
