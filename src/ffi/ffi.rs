use crate::kernel::portmacro::*;
use crate::kernel::riscv_virt::*;
use crate::portYIELD;
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::arch::asm;
use core::ffi::c_void;
use core::mem::forget;
use core::mem::size_of;
use spin::RwLock;

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

#[no_mangle]
pub extern "C" fn rustAssert(val: bool) {
    assert!(val);
}

#[no_mangle]
pub extern "C" fn rustPrint(val: usize) {
    let s = get_str_from_cchar(val);
    print(&s);
}

#[no_mangle]
pub extern "C" fn rustVSendString(val: usize) {
    let s = get_str_from_cchar(val);
    vSendString(&s);
}

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

#[no_mangle]
pub extern "C" fn rustYield() {
    portYIELD!();
}
