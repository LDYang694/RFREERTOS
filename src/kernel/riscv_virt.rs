use crate::ns16550::{v_out_ns16550, Device};
use core::arch::asm;
use crate::tasks::*;

// 串口基址
pub const NS16550_ADDR: usize = 0x10000000;

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

pub fn print( s: &str )
{
	
	// 初始化串口
	let dev = Device{
		addr: NS16550_ADDR,
	};
	for c in s.bytes(){
		v_out_ns16550( &dev, &c );
	}
	v_out_ns16550( &dev, &('\n' as u8) );
}

// 暴露给应用的打印字符串接口
pub fn vSendString( s: &str )
{
	// 初始化串口
	let dev = Device{
		addr: NS16550_ADDR,
	};
	vTaskEnterCritical();

	for c in s.bytes(){
		v_out_ns16550( &dev, &c );
	}
	v_out_ns16550( &dev, &('\n' as u8) );

	vTaskExitCritical();
}

fn handle_trap()
{
    #[warn(while_true)]
	while true{

	}
}
