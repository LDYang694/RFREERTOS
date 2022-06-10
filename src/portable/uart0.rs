extern "C"{
	fn sys_uart_putc_c(uart_num: u8, ch: u8);
}

pub struct Device {
	// 串口基址
	pub addr: usize
}
pub fn sys_uart_putc( dev: &Device, c: u8 )
{
	unsafe{
		sys_uart_putc_c(0, c);
	}
}
