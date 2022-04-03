// #include <FreeRTOS.h>

// #include <string.h>

// #include "riscv-virt.h"
// #include "ns16550.h"
use crate::ns16550::*;
use core::arch::asm;
use crate::tasks::*;
pub const PRIM_HART: usize = 0;
pub const CLINT_ADDR: u32 = 0x02000000;
pub const CLINT_MSIP: u32 = 0x0000;
pub const CLINT_MTIMECMP: u32 = 0x4000;
pub const CLINT_MTIME: u32 = 0xbff8;
// 串口基址
pub const NS16550_ADDR: usize = 0x10000000;

#[inline]
pub fn xGetCoreID() -> i32 {
	let id: i32;
	unsafe{
        asm!(
            "csrr {}, mhartid", out(reg) id
        );
	}
	id
}

// 暴露给应用的打印字符串接口
pub fn vSendString( s: &str )
{
	
	// 初始化串口
	let dev = Device{
		addr: NS16550_ADDR,
	};

	// vTaskEnterCritical();

	for c in s.bytes(){
		vOutNS16550( &dev, &c );
		// vOutNS16550( &dev, &('c' as u8) );
	}
	vOutNS16550( &dev, &('\n' as u8) );

	// vTaskExitCritical();
}

pub fn vSendStringDebug( s: &str )
{
	
	// 初始化串口
	let dev = Device{
		addr: NS16550_ADDR,
	};

	vTaskEnterCritical();

	for c in s.bytes(){
		vOutNS16550( &dev, &c );
		// vOutNS16550( &dev, &('c' as u8) );
	}
	vOutNS16550( &dev, &('\n' as u8) );

	vTaskExitCritical();
}

fn handle_trap()
{
    #[warn(while_true)]
	while true{

	}
}

// #[macro_export]
// #[allow_internal_unstable(print_internals, format_args_nl)]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ({
//         $crate::io::_print($crate::format_args_nl!($($arg)*));
//     })
// }
