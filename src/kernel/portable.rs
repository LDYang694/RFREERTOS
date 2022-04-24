extern crate alloc;
use alloc::sync::{Arc, Weak};
use crate::config::*;
use crate::linked_list::*;
use crate::portDISABLE_INTERRUPTS;
use crate::portENABLE_INTERRUPTS;
use crate::portmacro::*;
use crate::riscv_virt::*;
use crate::tasks::*;
use crate::kernel::kernel::READY_TASK_LISTS;
use crate::kernel::kernel::{TCB1_p, TCB2_p};
use alloc::format;
use core::arch::asm;
use spin::RwLock;

use super::projdefs::pdFALSE;
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
pub static mut xISRStackTop: *const StackType= 0 as *const StackType;

#[no_mangle]
pub static mut pullMachineTimerCompareRegister:UBaseType = 0;

pub static mut pxCurrentTCB_: Option<*const tskTaskControlBlock> = None;

static mut X_ISRSTACK: [StackType; CONFIG_ISR_STACK_SIZE_WORDS] = [0; CONFIG_ISR_STACK_SIZE_WORDS];

static mut ULL_NEXT_TIME: u64 = 0;
pub const ULL_MACHINE_TIMER_COMPARE_REGISTER_BASE: UBaseType = CONFIG_MTIMECMP_BASE_ADDRESS;

/* 
extern "C" {
    pub static mut : u32;
    
}*/
//todo:safe  global var and pointer

fn get_mtime()->u64{
    let mut result:u64=0;
    let pul_time_high: *const UBaseType = (CONFIG_MTIME_BASE_ADDRESS + 4) as *const UBaseType;
    let pul_time_low: *const UBaseType = CONFIG_MTIME_BASE_ADDRESS as *const UBaseType;
    unsafe{
        let mut ul_current_time_low: UBaseType= *pul_time_high;
        let mut ul_current_time_high: UBaseType= *pul_time_low;
        result= ul_current_time_high as u64;
        result=result<<32;
        result+= (ul_current_time_low + uxTimerIncrementsForOneTick) as u64;
    }
    
    result
}

pub fn v_port_setup_timer_interrupt() {
    let mut ul_hart_id: UBaseType;
    let pul_time_high: *const UBaseType = (CONFIG_MTIME_BASE_ADDRESS + 4) as *const UBaseType;
    let pul_time_low: *const UBaseType = CONFIG_MTIME_BASE_ADDRESS as *const UBaseType;
    let mut ul_current_time_low: UBaseType;
    let mut ul_current_time_high: UBaseType;

    unsafe {
        pullNextTime = &ULL_NEXT_TIME as *const u64 as u32;
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
        match get_current_tcb(){
            Some(x) => pxCurrentTCB = x as *const tskTaskControlBlock as u32,
            None => pxCurrentTCB = 0,
        }
    }
}
pub fn x_port_start_scheduler() -> BaseType {
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
    pdFALSE
}

pub fn set_current_tcb(tcb: Option<ListItemOwnerWeakLink>) {
    unsafe {
        match tcb{
            Some(x)=>{
                pxCurrentTCB_=Some(&*(*x.into_raw()).read());
            }
            None=>{
                pxCurrentTCB_=None;
            }
        }
    }
}

pub fn get_current_tcb()->Option<&'static mut tskTaskControlBlock>{
    unsafe{
        match pxCurrentTCB_{
            Some(x)=>{
                Some(&mut *(x as *mut tskTaskControlBlock))
            }
            None=>{
                None
            }
        }
    }
}

pub fn is_current_tcb(tcb: ListItemOwnerWeakLink)->bool{
    unsafe{
        match pxCurrentTCB_{
            Some(x)=>{
                let temp:*const tskTaskControlBlock=&*(*tcb.into_raw()).read();
                temp==x
            }
            None=>false
        }
    }
}





pub fn vTaskPrioritySet(pxTask:Option<TaskHandle_t>,uxNewPriority:UBaseType) 
{
    vTaskEnterCritical();
    match pxTask{
        Some(x)=>{
            ux_list_remove(Arc::downgrade(&x.read().xStateListItem));
            v_list_insert_end(&READY_TASK_LISTS[uxNewPriority as usize],Arc::clone(&x.read().xStateListItem));
            x.write().uxPriority=uxNewPriority;
        }
        None=>{
            unsafe{
                match get_current_tcb(){
                    Some(x)=>{
                        
                        ux_list_remove(Arc::downgrade(&(*x).xStateListItem));
                        v_list_insert_end(&READY_TASK_LISTS[uxNewPriority as usize],Arc::clone(&(*x).xStateListItem));
                        x.uxPriority=uxNewPriority;
                    }
                    None=>{}
                }
            }
        }
    }
    vTaskExitCritical();
}

pub fn uxTaskPriorityGet(pxTask:Option<TaskHandle_t>)->UBaseType
{
    unsafe{
        match get_current_tcb(){
            Some(x)=>unsafe {
                return (*x).uxPriority;
            }
            None=>{return 0;}
        }
    }
    
}

#[no_mangle]
pub extern "C" fn vTaskSwitchContext() {
    //todo
    // // print("vTaskSwitchContext");
    //port_disable_interrupts!();
    unsafe{
        if uxSchedulerSuspended==0{
                xYieldPending=false;
            taskSELECT_HIGHEST_PRIORITY_TASK();
        }
        else{
                xYieldPending=true;
        }
    }
    
    
}
