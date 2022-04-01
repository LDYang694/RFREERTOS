use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
static mut START_ADDR: usize = 0;
static mut MAX_SIZE: usize = 0;
static mut USED_SIZE: usize = 0;
pub struct HeapAlloc{}
unsafe impl GlobalAlloc for HeapAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let alloc_ptr = (START_ADDR + USED_SIZE) as *mut u8;
        // self.used_size = self.used_size + _layout.size();
        USED_SIZE += _layout.size();
        alloc_ptr
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // USED_SIZE -= _layout.size();
        // panic!("dealloc should be never called")
    }
}

impl HeapAlloc {
    pub fn init(&self, _start_addr: usize, _max_size: usize){
        unsafe {
            START_ADDR = _start_addr;
            MAX_SIZE = _max_size;
            USED_SIZE = 0;
        }
    }
}
