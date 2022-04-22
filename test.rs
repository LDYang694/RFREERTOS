#![no_std]
#![feature(core_intrinsics)]
use core::intrinsics;
use core::mem::size_of;
use core::panic::PanicInfo;

fn debug(num: usize) {
    let tmp = intrinsics::ctlz(num as u64) as u32 as usize;
}
fn main() {
    let num: usize = 944;
    debug(num);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
