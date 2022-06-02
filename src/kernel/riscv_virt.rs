// use crate::ns16550::{v_out_ns16550, Device};
use core::arch::asm;
use crate::tasks::*;
use crate::uart0::*;
use crate::portmacro::*;

#[inline]
pub fn x_get_core_id() -> i32 {
	let id: i32;
	unsafe{
        asm!(
            "csrr {}, mhartid", out(reg) id
        );
	}
	id
}

#[inline]
pub fn x_get_mtvec() -> UBaseType {
	let res: UBaseType;
	unsafe{
        asm!(
            "csrr {}, mtvec", out(reg) res
        );
	}
	res
}

pub fn print( s: &str )
{
	// 初始化串口
	let dev = Device{
		// addr: NS16550_ADDR,
		addr: 0x02500000
	};
	for c in s.bytes(){
		sys_uart_putc(&dev, c);

	}
	sys_uart_putc( &dev, '\n' as u8 );
	sys_uart_putc( &dev, '\r' as u8 );
}

// 暴露给应用的打印字符串接口
pub fn vSendString( s: &str )
{
	// 初始化串口
	let dev = Device{
		// addr: NS16550_ADDR,
		addr: 0x02500000
	};
	vTaskEnterCritical();

	for c in s.bytes(){
		sys_uart_putc(&dev, c);
	}
	sys_uart_putc( &dev, ('\n' as u8) );
	sys_uart_putc( &dev, ('\r' as u8) );

	vTaskExitCritical();
}

fn handle_trap()
{
    #[warn(while_true)]
	while true{

	}
}
