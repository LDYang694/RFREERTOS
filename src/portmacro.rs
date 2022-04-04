use crate::config::*;


pub type StackType = u32;
pub type BaseType = i32;
pub type UBaseType = u32;
pub type TickType = u32;

pub const PORT_MAX_DELAY: TickType = 0xffffffff;
pub const PORT_TICK_TYPE_IS_ATOMIC: BaseType = 1;
pub const PORT_STACK_GROWTH: BaseType = -1;
pub const PORT_TICK_PERIOD_MS: TickType = 1000/CONFIG_TICK_RATE_HZ;
pub const PORT_BYTE_ALIGNMENT: BaseType = 16;

#[macro_export]
macro_rules! port_yield{
    () =>{
        unsafe{
            asm!("ecall");
        }
    }
}

#[macro_export]
macro_rules! port_end_switching_isr{
    ($x:expr)=>{
        if $x {
            v_task_switch_context();
        }
    }
}

#[macro_export]
macro_rules! port_yield_from_isr{
    ($x:expr)=>{
        port_end_switching_isr!($x);
    }
}

pub const PORT_CRITICAL_NESTING_IN_TCB:BaseType = 1;

#[macro_export]
macro_rules! port_set_interrupt_mask_from_isr {
    () => {
        0
    };
}

#[macro_export]
macro_rules!  port_clear_interrupt_mask_from_isr{
    ($x:expr)=>{
        $x
    }
}

#[macro_export]
macro_rules! port_disable_interrupts{
    ()=>{
        unsafe{
            asm!("csrci mstatus, 8");
        }
    }
}

#[macro_export]
macro_rules! port_enable_interrupts{
    ()=>{
        unsafe{
            asm!("csrsi mstatus, 8");
        }
    }
}

#[macro_export]
macro_rules! port_enter_critical{
    ()=>{
        v_task_enter_critical();
    }
}

#[macro_export]
macro_rules! port_exit_critical{
    ()=>{
        v_task_exit_critical();
    }
}

pub const USE_PORT_OPTIMISED_TASK_SELECTION:BaseType = 1;

#[macro_export]
macro_rules! port_record_ready_priority{
    ($x:expr,$y:expr)=>{
        $y=$y|(1<<$x);
    }
}

#[macro_export]
macro_rules! port_reset_ready_priority{
    ($x:expr,$y:expr)=>{
        $y=$y&(~(1<<$x));
    }
}

#[macro_export]
macro_rules! port_get_highest_priority{
    ($x:expr,$y:expr)=>{
        $x=31-$y.leading_zeros();
    }
}

#[macro_export]
macro_rules! port_nop{
    ()=>{
        unsafe{
            asm!("nop");
        }
    }
}

#[macro_export]
macro_rules! port_memory_barrier{
    ()=>{}
        
}

#[macro_export]
macro_rules! mt_coverage_test_marker{
    ()=>{
        unsafe{
            asm!("nop");
        }
    }
        
}
