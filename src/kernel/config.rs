//! Application specific definitions. <br>
//! equal to FreeRTOSConfig.h

use crate::portable::portmacro::*;
use lazy_static::lazy_static;

extern "C" {
    pub static CLINT_ADDR_: UBaseType;
    pub static CONFIG_TICK_RATE_HZ: TickType;
    pub static CONFIG_CPU_CLOCK_HZ: UBaseType;
    pub static CONFIG_ISR_STACK_SIZE_WORDS: usize;
    pub static configMAX_PRIORITIES_: UBaseType;
}

lazy_static! {
    pub static ref CONFIG_MTIME_BASE_ADDRESS: UBaseType = unsafe { CLINT_ADDR_ + 0xbff8 };
    pub static ref CONFIG_MTIMECMP_BASE_ADDRESS: UBaseType = unsafe { CLINT_ADDR_ + 0x4000 };
    pub static ref portTICK_RATE_MS: TickType = 1000 / unsafe { CONFIG_TICK_RATE_HZ };
    pub static ref configMAX_PRIORITIES: UBaseType = unsafe { configMAX_PRIORITIES_ };
}

// Cannot copy from config.h. Must manually set.
pub const KERNEL_HEAP_SIZE: usize = 0x400000;
pub const USER_STACK_SIZE: usize = 0x10000;

/* register definitions */
pub const REG_RBR: usize = 0x00; /* Receiver buffer reg. */
pub const REG_THR: usize = 0x00; /* Transmitter holding reg. */
pub const REG_IER: usize = 0x01; /* Interrupt enable reg. */
pub const REG_IIR: usize = 0x02; /* Interrupt ID reg. */
pub const REG_FCR: usize = 0x02; /* FIFO control reg. */
pub const REG_LCR: usize = 0x03; /* Line control reg. */
pub const REG_MCR: usize = 0x04; /* Modem control reg. */
pub const REG_LSR: usize = 0x05; /* Line status reg. */
pub const REG_MSR: usize = 0x06; /* Modem status reg. */
pub const REG_SCR: usize = 0x07; /* Scratch reg. */
pub const REG_BRDL: usize = 0x00; /* Divisor latch (LSB) */
pub const REG_BRDH: usize = 0x01; /* Divisor latch (MSB) */

/* Line status */
pub const LSR_DR: u8 = 0x01; /* Data ready */
pub const LSR_OE: u8 = 0x02; /* Overrun error */
pub const LSR_PE: u8 = 0x04; /* Parity error */
pub const LSR_FE: u8 = 0x08; /* Framing error */
pub const LSR_BI: u8 = 0x10; /* Break interrupt */
pub const LSR_THRE: u8 = 0x20; /* Transmitter holding register empty */
pub const LSR_TEMT: u8 = 0x40; /* Transmitter empty */
pub const LSR_EIRF: u8 = 0x80; /* Error in RCVR FIFO */
