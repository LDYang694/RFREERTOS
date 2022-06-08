//! portable apis

extern crate alloc;
use crate::kernel::allocator::DYNAMIC_ALLOCATOR;
use crate::kernel::config::*;
use crate::kernel::kernel::*;
use crate::kernel::linked_list::*;
use crate::kernel::projdefs::pdFALSE;
use crate::kernel::tasks::*;
use crate::portDISABLE_INTERRUPTS;
use crate::portENABLE_INTERRUPTS;
use crate::portable::portmacro::{BaseType, StackType, UBaseType};
use crate::portable::riscv_virt::*;
use alloc::alloc::Global;
use alloc::format;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::arch::asm;
use core::intrinsics::forget;
use lazy_static::lazy_static;
use spin::RwLock;
//use crate::pxCurrentTCB_;
// use crate::pxCurrentTCB;
extern "C" {
    fn xPortStartFirstTask();
}

#[no_mangle]
pub static mut uxTimerIncrementsForOneTick: UBaseType = 0;

#[no_mangle]
pub static mut pxCurrentTCB: UBaseType = 0;

#[no_mangle]
pub static mut pullNextTime: UBaseType = 0;

#[no_mangle]
pub static mut xISRStackTop: *const StackType = 0 as *const StackType;

#[no_mangle]
pub static mut pullMachineTimerCompareRegister: UBaseType = 0;

pub static mut pxCurrentTCB_: Option<*const tskTaskControlBlock> = None;

static mut ULL_NEXT_TIME: u64 = 0;

lazy_static! {
    pub static ref ULL_MACHINE_TIMER_COMPARE_REGISTER_BASE: UBaseType =
        *CONFIG_MTIMECMP_BASE_ADDRESS;
    pub static ref X_ISRSTACK_: Arc<RwLock<Vec<StackType>>> =
        Arc::new(RwLock::new(Vec::with_capacity(unsafe {
            CONFIG_ISR_STACK_SIZE_WORDS
        })));
}

//todo:safe  global var and pointer

/// Get current mtime.
fn get_mtime() -> u64 {
    let mut result: u64 = 0;
    let pul_time_high: *const UBaseType = (*CONFIG_MTIME_BASE_ADDRESS + 4) as *const UBaseType;
    let pul_time_low: *const UBaseType = (*CONFIG_MTIME_BASE_ADDRESS) as *const UBaseType;
    unsafe {
        let mut ul_current_time_low: UBaseType = *pul_time_high;
        let mut ul_current_time_high: UBaseType = *pul_time_low;
        result = ul_current_time_high as u64;
        result = result << 32;
        result += (ul_current_time_low + uxTimerIncrementsForOneTick) as u64;
    }

    result
}

/// Setup timer interrupt during startup.
pub fn v_port_setup_timer_interrupt() {
    let mut ul_hart_id: UBaseType;
    let pul_time_high: *const UBaseType = (*CONFIG_MTIME_BASE_ADDRESS + 4) as *const UBaseType;
    let pul_time_low: *const UBaseType = (*CONFIG_MTIME_BASE_ADDRESS) as *const UBaseType;
    let mut ul_current_time_low: UBaseType;
    let mut ul_current_time_high: UBaseType;

    unsafe {
        pullNextTime = &ULL_NEXT_TIME as *const u64 as usize;
        uxTimerIncrementsForOneTick = CONFIG_CPU_CLOCK_HZ / CONFIG_TICK_RATE_HZ;
        asm!("csrr {0}, mhartid",out(reg) ul_hart_id);
        pullMachineTimerCompareRegister = *ULL_MACHINE_TIMER_COMPARE_REGISTER_BASE + ul_hart_id * 4;
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

/// Copy current tcb to pxCurrentTCB for C interface.
pub fn auto_set_currentTcb() {
    unsafe {
        match get_current_tcb() {
            Some(x) => pxCurrentTCB = x as *const tskTaskControlBlock as usize,
            None => pxCurrentTCB = 0,
        }
    }
}

/// Start up scheduler.
pub fn x_port_start_scheduler() -> BaseType {
    unsafe {
        xISRStackTop = &(X_ISRSTACK_.read()[CONFIG_ISR_STACK_SIZE_WORDS - 1]) as *const usize;
    }
    v_port_setup_timer_interrupt();
    let mut tmp: usize = 0x800;
    if *CONFIG_MTIME_BASE_ADDRESS != 0 && *CONFIG_MTIMECMP_BASE_ADDRESS != 0 {
        tmp = 0x880;
    }
    print("start first task");
    unsafe {
        auto_set_currentTcb();
        asm!("csrs mie,{0}",in(reg) tmp);
        xPortStartFirstTask();
    }
    pdFALSE
}

/// Set current tcb. <br>
/// Use with auto_set_currentTcb().
pub fn set_current_tcb(tcb: Option<ListItemOwnerWeakLink>) {
    unsafe {
        match tcb {
            Some(x) => {
                pxCurrentTCB_ = Some(&*(*x.into_raw()).read());
            }
            None => {
                pxCurrentTCB_ = None;
            }
        }
    }
}

/// get current tcb
pub fn get_current_tcb() -> Option<&'static mut tskTaskControlBlock> {
    let xReturn: Option<&'static mut tskTaskControlBlock>;
    unsafe {
        xReturn = match pxCurrentTCB_ {
            Some(x) => Some(&mut *(x as *mut tskTaskControlBlock)),
            None => None,
        }
    }
    xReturn
}

/// return if target tcb is current tcb
pub fn is_current_tcb(tcb: ListItemOwnerWeakLink) -> bool {
    unsafe {
        match pxCurrentTCB_ {
            Some(x) => {
                let temp: *const tskTaskControlBlock = &*(*tcb.into_raw()).read();
                temp == x
            }
            None => false,
        }
    }
}

pub fn is_current_tcb_raw(tcb: &mut tskTaskControlBlock) -> bool {
    unsafe {
        match pxCurrentTCB_ {
            Some(x) => {
                let temp: *const tskTaskControlBlock = tcb as *const tskTaskControlBlock;
                temp == x
            }
            None => false,
        }
    }
}

/// switch context
#[no_mangle]
pub extern "C" fn vTaskSwitchContext() {
    //todo
    // // print("vTaskSwitchContext");
    //port_disable_interrupts!();
    unsafe {
        if uxSchedulerSuspended == 0 {
            xYieldPending = false;
            taskSELECT_HIGHEST_PRIORITY_TASK();
        } else {
            xYieldPending = true;
        }
    }
}
