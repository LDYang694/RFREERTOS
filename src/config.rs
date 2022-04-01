use crate::portmacro::*;

const CLINT_ADDR:UBaseType = 0x02000000;

pub const CONFIG_MTIME_BASE_ADDRESS:UBaseType = CLINT_ADDR+0xbff8;
pub const CONFIG_MTIMECMP_BASE_ADDRESS:UBaseType = CLINT_ADDR+0x4000;
pub const CONFIG_ISR_STACK_SIZE_WORDS:usize = 2048;
pub const CONFIG_TICK_RATE_HZ:TickType = 1000;
pub const CONFIG_CPU_CLOCK_HZ:UBaseType = 1000000;