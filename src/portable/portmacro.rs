//! portable macros

use crate::config::*;

pub type StackType = usize;
pub type BaseType = isize;
pub type UBaseType = usize;
pub type TickType = usize;
#[cfg(target_arch = "riscv64")]
pub const PORT_MAX_DELAY: TickType = 0xffffffffffffffff;
#[cfg(target_arch = "riscv32")]
pub const PORT_MAX_DELAY: TickType = 0xffffffff;
pub const PORT_TICK_TYPE_IS_ATOMIC: BaseType = 1;
pub const PORT_STACK_GROWTH: BaseType = -1;
pub const PORT_BYTE_ALIGNMENT: BaseType = 16;

#[macro_export]
macro_rules! portYIELD {
    () => {
        unsafe {
            asm!("ecall");
        }
    };
}

#[macro_export]
macro_rules! portYIELD_WITHIN_API {
    () => {
        portYIELD!();
    };
}

#[macro_export]
macro_rules! portEND_SWITCHING_ISR {
    ($x:expr) => {
        if $x {
            vTaskSwitchContext();
        }
    };
}

#[macro_export]
macro_rules! portYIELD_FROM_ISR {
    ($x:expr) => {
        portEND_SWITCHING_ISR!($x);
    };
}

pub const PORT_CRITICAL_NESTING_IN_TCB: BaseType = 1;

#[macro_export]
macro_rules! port_set_interrupt_mask_from_isr {
    () => {
        0
    };
}

#[macro_export]
macro_rules! portCLEAR_INTERRUPT_MASK_FROM_ISR {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! portDISABLE_INTERRUPTS {
    () => {
        unsafe {
            asm!("csrci mstatus, 8");
        }
    };
}

#[macro_export]
macro_rules! portENABLE_INTERRUPTS {
    () => {
        unsafe {
            asm!("csrsi mstatus, 8");
        }
    };
}

#[macro_export]
macro_rules! portENTER_CRITICAL {
    () => {
        vTaskEnterCritical();
    };
}

#[macro_export]
macro_rules! portEXIT_CRITICAL {
    () => {
        vTaskExitCritical();
    };
}

pub const USE_PORT_OPTIMISED_TASK_SELECTION: BaseType = 1;

#[macro_export]
macro_rules! portRECORD_READY_PRIORITY {
    ($x:expr,$y:expr) => {
        $y = $y | (1 << $x);
    };
}

#[macro_export]
macro_rules! portRESET_READY_PRIORITY{
    ($x:expr,$y:expr)=>{
        $y=$y&(~(1<<$x));
    }
}

#[macro_export]
macro_rules! portGET_HIGHEST_PRIORITY {
    ($x:expr,$y:expr) => {
        $x = 31 - $y.leading_zeros();
    };
}

#[macro_export]
macro_rules! portNOP {
    () => {
        unsafe {
            asm!("nop");
        }
    };
}

#[macro_export]
macro_rules! portMEMORY_BARRIER {
    () => {
        //TODO:
    };
}

#[macro_export]
macro_rules! mtCOVERAGE_TEST_MARKER {
    () => {
        unsafe {
            asm!("nop");
        }
    };
}
