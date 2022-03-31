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

pub struct Device {
	// 串口基址
	pub addr: usize
}
fn readb(addr: &usize) -> u8
{
    let p = *addr as *const u8;
	unsafe{ *p }
}

fn writeb(b: &u8, addr: &usize)
{
    let p = *addr as *mut u8;
	unsafe{ *p = *b };
}
// 向串口写单个字符，阻塞式
pub fn vOutNS16550( dev: &Device, c: &u8 )
{
	let addr = dev.addr;

	while (readb( &(addr + REG_LSR) ) & LSR_THRE) == 0 {
		/* busy wait */
	}

	writeb( c, &(addr + REG_THR) );
}
