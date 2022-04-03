use crate::config::*;
use crate::linked_list::*;
use crate::alloc::string::ToString;
use crate::mt_coverage_test_marker;
use crate::pdFALSE;
use crate::port_disable_interrupts;
use crate::port_enable_interrupts;
use crate::portable::*;
use spin::RwLock;
use alloc::format;
pub type StackType_t=usize;
pub type StackType_t_link = usize;
pub type Param_link = usize;
pub type TCB_t_link = Arc<RwLock<TCB_t>>;
pub type UBaseType_t=usize;
// pub type TaskFunction_t = dyn Fn(usize);
// pub type TaskFunction_t=fn(*mut c_void);
pub type TaskFunction_t=*mut fn(*mut c_void);
// use std::cell::RefCell;
use crate::alloc::sync::{Arc, Weak};
use alloc::string::String;
use crate::riscv_virt::*;
use core::ffi::c_void;
use core::arch::asm;

pub static mut xSchedulerRunning: bool = pdFALSE!();

extern "C"{
    pub fn pxPortInitialiseStack(
        pxTopOfStack: *mut StackType_t,
        pxCode: u32,
        pvParameters: *mut c_void,
    ) -> *mut StackType_t;
}

#[derive(Debug)]
pub struct tskTaskControlBlock {
    pub pxTopOfStack: StackType_t_link,
    pxStack: StackType_t_link,
    pcTaskName: String,
    pub xStateListItem: ListItemLink,
    pub uxCriticalNesting: UBaseType_t,
}
impl Default for tskTaskControlBlock {
    fn default() -> Self {
        tskTaskControlBlock {
            pxStack: 0,
            pxTopOfStack: 0,
            pcTaskName: String::new(),
            xStateListItem: Default::default(),
            uxCriticalNesting: 0
        }
    }
}
pub fn TCB_set_pxStack(tcb: &TCB_t_link, item: StackType_t_link) {
    //TODO: item owner
    tcb.write().pxStack = item;
}

pub type tskTCB = tskTaskControlBlock;
pub type TCB_t = tskTCB;
//TaskHandle_t=tskTaskControlBlock*
pub type TaskHandle_t = Arc<RwLock<tskTaskControlBlock>>;

pub fn xTaskCreateStatic(
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    puxStackBuffer: Option<StackType_t_link>,
    pxTaskBuffer: Option<TCB_t_link>,
) -> Option<TaskHandle_t> {
    let xReturn = Arc::new(RwLock::new(tskTaskControlBlock::default()));
    // let xxReturn=
    // if (pxTaskBuffer!=None && puxStackBuffer!=None){

    //     None
    // }else{
    //     None
    // }
    print("xTaskCreateStatic 1111");
    //TODO:assert if =true
    let pxNewTCB: TCB_t_link = pxTaskBuffer.unwrap().clone();
    TCB_set_pxStack(&pxNewTCB, puxStackBuffer.unwrap());
    print("xTaskCreateStatic 2222"); 
    let xReturn = prvInitialiseNewTask(
        pxTaskCode,
        pcName,
        ulStackDepth,
        pvParameters,
        &xReturn,
        pxNewTCB,
    );
    print("xTaskCreateStatic 3333"); 
    Some(xReturn)
}

pub fn prvInitialiseNewTask(
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    pxCreatedTask: &TaskHandle_t,
    pxNewTCB: TCB_t_link,
) -> TaskHandle_t {
    let mut pxTopOfStack: StackType_t_link =
        pxNewTCB.read().pxStack;
    pxTopOfStack = pxTopOfStack & (!(0x0007usize));
    
    let mut x: UBaseType = 0;
    //TODO: name length
    print("prvInitialiseNewTask 1111");
    pxNewTCB.write().pcTaskName = pcName.to_string();
    //TODO:auto init
    print("prvInitialiseNewTask 2222");
    list_item_set_owner(
        &pxNewTCB.write().xStateListItem,
        Arc::downgrade(&pxNewTCB),
    );
    print("prvInitialiseNewTask 33333");
    //TODO: connect
    let s_=format!("top of stack{:X}",pxTopOfStack);
    print(&s_);
    unsafe{
        pxNewTCB.write().pxTopOfStack = pxPortInitialiseStack(pxTopOfStack as *mut _,pxTaskCode,0 as *mut _) as usize;
        pxNewTCB.write().uxCriticalNesting = 0;
    }
    let s_=format!("top of stack{:X}",pxNewTCB.read().pxTopOfStack);
    print(&s_);
    print("prvInitialiseNewTask 4444");
    //TODO: return
    pxNewTCB
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        println!("test world!");
    }
}

// pub fn x_task_create_static() {}
fn prvInitialiseTaskLists() {
    //initial in list impl
}

pub fn vTaskEnterCritical(){
    port_disable_interrupts!();
    unsafe{
        if xSchedulerRunning != pdFALSE!() {
            (*(pxCurrentTCB_.unwrap() as *mut tskTaskControlBlock)).uxCriticalNesting += 1;
            if  (*(pxCurrentTCB_.unwrap())).uxCriticalNesting == 1{
                // TODO: portASSERT_IF_IN_ISR
            }
        }
        else {
            mt_coverage_test_marker!();
        }
    }
    
}

pub fn vTaskExitCritical(){
    unsafe{
        let curTCB = pxCurrentTCB_.unwrap();
        if xSchedulerRunning != pdFALSE!() {
            if (*curTCB).uxCriticalNesting > 0{
                (*(curTCB as *mut tskTaskControlBlock)).uxCriticalNesting -= 1;
                if (*(curTCB)).uxCriticalNesting == 0{
                    port_enable_interrupts!();
                }
                else{
                    mt_coverage_test_marker!();
                }
            }
            else {
                mt_coverage_test_marker!();
            }
        }
        else {
            mt_coverage_test_marker!();
        }
    }
}
