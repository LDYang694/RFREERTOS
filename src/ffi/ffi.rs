//! ffi general utils

use crate::kernel::riscv_virt::*;
use crate::portYIELD;
use crate::portable::portmacro::*;
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use core::arch::asm;
use spin::RwLock;

/// Transform C char into rust string
pub fn get_str_from_cchar(mut pcName: usize) -> String {
    let mut name = String::new();
    loop {
        let val: u8 = unsafe { *(pcName as *const u8) };
        if val == 0 {
            return name;
        }
        let c = val as char;
        name.push(c);
        pcName += 1;
    }
}

/// Assert from C
#[no_mangle]
pub extern "C" fn rustAssert(val: bool) {
    assert!(val);
}

/// Print from C.<br>
/// Does not support formatting.
#[no_mangle]
pub extern "C" fn rustPrint(val: usize) {
    let s = get_str_from_cchar(val);
    print(&s);
}

/// vSendString from C.<br>
/// Does not support formatting.
#[no_mangle]
pub extern "C" fn rustVSendString(val: usize) {
    let s = get_str_from_cchar(val);
    vSendString(&s);
}

/// Malloc memory from C
#[no_mangle]
pub extern "C" fn rustMalloc(size_: usize) -> usize {
    use alloc::alloc::Layout;

    let layout = Layout::from_size_align(size_ as usize, 4).ok().unwrap();
    let stack_ptr: *mut u8;
    unsafe {
        stack_ptr = alloc::alloc::alloc(layout);
    }
    stack_ptr as usize
}

/// Yield from C
#[no_mangle]
pub extern "C" fn rustYield() {
    portYIELD!();
}
