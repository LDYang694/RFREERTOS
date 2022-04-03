use crate::alloc::sync::{Arc, Weak};
use crate::portmacro::*;
use crate::riscv_virt::*;
use crate::task1;
use crate::tasks::*;
use crate::{config::*};
use crate::{TCB1_p, TCB2_p};
use alloc::format;
use core::arch::asm;
//use crate::pxCurrentTCB_;
// use crate::pxCurrentTCB;
extern "C" {
    fn xPortStartFirstTask();
    fn testfunc(x: u32) -> u32;
}

#[no_mangle]
pub static mut uxTimerIncrementsForOneTick: UBaseType = 0;

#[no_mangle]
pub static mut pxCurrentTCB: UBaseType = 0;

pub static mut pxCurrentTCB_: Option<*const tskTaskControlBlock> = None;

static mut X_ISRSTACK: [StackType; CONFIG_ISR_STACK_SIZE_WORDS] = [0; CONFIG_ISR_STACK_SIZE_WORDS];

static mut ULL_NEXT_TIME: u64 = 0;
pub const ULL_MACHINE_TIMER_COMPARE_REGISTER_BASE: UBaseType = CONFIG_MTIMECMP_BASE_ADDRESS;

extern "C" {
    pub static mut pullMachineTimerCompareRegister: u32;
    pub static mut pullNextTime: u32;
    pub static mut xISRStackTop: *const StackType;
}
//todo:safe  global var and pointer

pub fn v_port_setup_timer_interrupt() {
    let mut ul_hart_id: UBaseType;
    let pul_time_high: *const UBaseType = (CONFIG_MTIME_BASE_ADDRESS + 4) as *const UBaseType;
    let pul_time_low: *const UBaseType = CONFIG_MTIME_BASE_ADDRESS as *const UBaseType;
    let mut ul_current_time_low: UBaseType;
    let mut ul_current_time_high: UBaseType;

    unsafe {
        pullNextTime = &ULL_NEXT_TIME as *const u64 as u32;
        let s = format!("pullNextTime={:X}", pullNextTime);
        print(&s);
        uxTimerIncrementsForOneTick = CONFIG_CPU_CLOCK_HZ / CONFIG_TICK_RATE_HZ;
        asm!("csrr {0}, mhartid",out(reg) ul_hart_id);
        pullMachineTimerCompareRegister = ULL_MACHINE_TIMER_COMPARE_REGISTER_BASE + ul_hart_id * 4;
        loop {
            ul_current_time_high = *pul_time_high;
            ul_current_time_low = *pul_time_low;
            if ul_current_time_high == *pul_time_high {
                break;
            }
        }
        ULL_NEXT_TIME = ul_current_time_high as u64;
        ULL_NEXT_TIME <<= 32;
        ULL_NEXT_TIME += (ul_current_time_low + uxTimerIncrementsForOneTick) as u64;
        let pointer: *mut u64 = pullMachineTimerCompareRegister as *mut u64;
        *pointer = ULL_NEXT_TIME;
        ULL_NEXT_TIME += uxTimerIncrementsForOneTick as u64;
    }

    //todo
}

pub fn auto_set_currentTcb() {
    unsafe {
        match pxCurrentTCB_ {
            Some(x) => pxCurrentTCB = x as u32,
            None => pxCurrentTCB = 0,
        }
    }
}
pub fn x_port_start_scheduler() -> bool {
    unsafe {
        xISRStackTop = (&X_ISRSTACK[CONFIG_ISR_STACK_SIZE_WORDS - 1]) as *const u32;
    }
    v_port_setup_timer_interrupt();
    let mut tmp: u32 = 0x800;
    if CONFIG_MTIME_BASE_ADDRESS != 0 && CONFIG_MTIMECMP_BASE_ADDRESS != 0 {
        tmp = 0x880;
    }
    print("start first task");
    unsafe {
        auto_set_currentTcb();
        asm!("csrs mie,{0}",in(reg) tmp);
        xPortStartFirstTask();
    }
    false
}

pub fn set_current_tcb(tcb: Option<*const tskTaskControlBlock>) {
    unsafe {
        pxCurrentTCB_ = tcb;
    }
}

#[no_mangle]
pub extern "C" fn xTaskIncrementTick() {
    //todo
}

#[no_mangle]
pub extern "C" fn vTaskSwitchContext() {
    //todo
    // // print("vTaskSwitchContext");
    unsafe {
        if pxCurrentTCB_.unwrap() == &*TCB1_p.read() {
            set_current_tcb(Some(&*TCB2_p.read()));
        } else {
            if pxCurrentTCB_.unwrap() == &*TCB2_p.read() {
                set_current_tcb(Some(&*TCB1_p.read()));
            }
        }
        auto_set_currentTcb();
    }
}
