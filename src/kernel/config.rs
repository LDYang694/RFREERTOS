//! Application specific definitions. <br>
//! equal to FreeRTOSConfig.h

use crate::portmacro::*;

const CLINT_ADDR:UBaseType = 0x02000000;

pub const CONFIG_MTIME_BASE_ADDRESS:UBaseType = CLINT_ADDR+0xbff8;
pub const CONFIG_MTIMECMP_BASE_ADDRESS:UBaseType = CLINT_ADDR+0x4000;
pub const CONFIG_ISR_STACK_SIZE_WORDS:usize = 2048;
pub const CONFIG_TICK_RATE_HZ:TickType = 1000;
pub const CONFIG_CPU_CLOCK_HZ:UBaseType = 1000000;
pub const KERNEL_HEAP_SIZE: usize = 0x400000;
pub const USER_STACK_SIZE: usize = 0x10000;
pub const PORT_ISR_STACK_FILL_BYTE: BaseType = 0xee;

pub const PRIM_HART: usize = 0;
pub const CLINT_MSIP: u32 = 0x0000;
pub const CLINT_MTIMECMP: u32 = 0x4000;
pub const CLINT_MTIME: u32 = 0xbff8;

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
pub const LSR_DR: u8 = 	0x01; /* Data ready */
pub const LSR_OE: u8 = 	0x02; /* Overrun error */
pub const LSR_PE: u8 = 	0x04; /* Parity error */
pub const LSR_FE: u8 = 	0x08; /* Framing error */
pub const LSR_BI: u8 = 	0x10; /* Break interrupt */
pub const LSR_THRE: u8 = 0x20; /* Transmitter holding register empty */
pub const LSR_TEMT: u8 = 0x40; /* Transmitter empty */
pub const LSR_EIRF: u8 = 0x80; /* Error in RCVR FIFO */

pub const configMAX_PRIORITIES:UBaseType = 16;
