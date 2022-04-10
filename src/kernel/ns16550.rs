use crate::config::*;

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
pub fn v_out_ns16550( dev: &Device, c: &u8 )
{
	let addr = dev.addr;

	while (readb( &(addr + REG_LSR) ) & LSR_THRE) == 0 {
		/* busy wait */
	}

	writeb( c, &(addr + REG_THR) );
}
