//! serial interface api

use crate::config::*;

pub struct Device {
    /// Base addr of Serial Interface.
    pub addr: usize,
}

/// Read byte from target address.
fn readb(addr: &usize) -> u8 {
    let p = *addr as *const u8;
    unsafe { *p }
}

/// Write byte to target address.
fn writeb(b: &u8, addr: &usize) {
    let p = *addr as *mut u8;
    unsafe { *p = *b };
}

/// Write byte to serial interface. <br>
/// Stall until writing is completed.
pub fn v_out_ns16550(dev: &Device, c: &u8) {
    let addr = dev.addr;

    while (readb(&(addr + REG_LSR)) & LSR_THRE) == 0 { /* busy wait */ }

    writeb(c, &(addr + REG_THR));
}
