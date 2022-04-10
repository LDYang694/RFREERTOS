extern crate alloc;

use crate::kernel::linked_list::*;
use crate::kernel::portable::*;
use crate::mt_coverage_test_marker;
use crate::pdFALSE;
use crate::port_disable_interrupts;
use crate::port_enable_interrupts;
use crate::port_yield;
use alloc::string::ToString;
use alloc::sync::{Arc, Weak};
use crate::portmacro::*;
use crate::kernel::kernel::TCB1_p;
use crate::kernel::kernel::DELAYED_TASK_LIST;
use crate::kernel::kernel::READY_TASK_LISTS;
use alloc::format;
use spin::RwLock;
pub type StackType_t = usize;
pub type StackType_t_link = usize;
pub type Param_link = usize;
pub type TCB_t_link = Arc<RwLock<TCB_t>>;
pub type UBaseType_t = usize;
// pub type TaskFunction_t = dyn Fn(usize);
// pub type TaskFunction_t=fn(*mut c_void);
pub type TaskFunction_t = *mut fn(*mut c_void);
// use std::cell::RefCell;
// use alloc::sync::{Arc, Weak};
use crate::riscv_virt::*;
use alloc::string::String;
use core::arch::asm;
use core::ffi::c_void;

pub static mut X_SCHEDULER_RUNNING: bool = pdFALSE!();
pub static mut xTickCount: UBaseType = 0;
pub static mut xNextTaskUnblockTime: UBaseType = PORT_MAX_DELAY;

#[macro_export]
macro_rules! pdFALSE {
    () => {
        false
    };
}

#[macro_export]
macro_rules! pdTRUE {
    () => {
        true
    };
}

extern "C" {
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
    pub priority: UBaseType,
}
impl Default for tskTaskControlBlock {
    fn default() -> Self {
        tskTaskControlBlock {
            pxStack: 0,
            pxTopOfStack: 0,
            pcTaskName: String::new(),
            xStateListItem: Default::default(),
            uxCriticalNesting: 0,
            priority: 0,
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

pub fn x_task_create_static(
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    puxStackBuffer: Option<StackType_t_link>,
    pxTaskBuffer: Option<TCB_t_link>,
    priority: UBaseType,
) -> Option<TaskHandle_t> {
    let xReturn = Arc::new(RwLock::new(tskTaskControlBlock::default()));
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
        priority,
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
    priority: UBaseType,
    pxNewTCB: TCB_t_link,
) -> TaskHandle_t {
    let mut pxTopOfStack: StackType_t_link = pxNewTCB.read().pxStack;
    pxTopOfStack = pxTopOfStack & (!(0x0007usize));

    let mut x: UBaseType = 0;
    //TODO: name length
    print("prvInitialiseNewTask 1111");
    pxNewTCB.write().pcTaskName = pcName.to_string();
    pxNewTCB.write().priority = priority;
    //TODO:auto init
    print("prvInitialiseNewTask 2222");
    list_item_set_owner(&pxNewTCB.write().xStateListItem, Arc::downgrade(&pxNewTCB));
    print("prvInitialiseNewTask 33333");
    //TODO: connect
    let s_ = format!("top of stack{:X}", pxTopOfStack);
    print(&s_);
    unsafe {
        pxNewTCB.write().pxTopOfStack =
            pxPortInitialiseStack(pxTopOfStack as *mut _, pxTaskCode, 0 as *mut _) as usize;
        pxNewTCB.write().uxCriticalNesting = 0;
    }
    let s_ = format!("top of stack{:X}", pxNewTCB.read().pxTopOfStack);
    print(&s_);
    print("prvInitialiseNewTask 4444");
    //TODO: return
    pxNewTCB
}

pub fn v_task_start_scheduler() {
    unsafe {
        X_SCHEDULER_RUNNING = pdTRUE!();
    }
    set_current_tcb(Some(&*TCB1_p.read()));
    if x_port_start_scheduler() != pdFALSE!() {
        panic!("error scheduler!!!!!!");
    }
}

// pub fn x_task_create_static() {}
fn prvInitialiseTaskLists() {
    //initial in list impl
}

pub fn v_task_enter_critical() {
    port_disable_interrupts!();
    unsafe {
        if X_SCHEDULER_RUNNING != pdFALSE!() {
            (*(pxCurrentTCB_.unwrap() as *mut tskTaskControlBlock)).uxCriticalNesting += 1;
            if (*(pxCurrentTCB_.unwrap())).uxCriticalNesting == 1 {
                // TODO: portASSERT_IF_IN_ISR
            }
        } else {
            mt_coverage_test_marker!();
        }
    }
}

pub fn v_task_exit_critical() {
    unsafe {
        let cur_tcb = pxCurrentTCB_.unwrap();
        if X_SCHEDULER_RUNNING != pdFALSE!() {
            if (*cur_tcb).uxCriticalNesting > 0 {
                (*(cur_tcb as *mut tskTaskControlBlock)).uxCriticalNesting -= 1;
                if (*(cur_tcb)).uxCriticalNesting == 0 {
                    port_enable_interrupts!();
                } else {
                    mt_coverage_test_marker!();
                }
            } else {
                mt_coverage_test_marker!();
            }
        } else {
            mt_coverage_test_marker!();
        }
    }
}

pub fn taskSELECT_HIGHEST_PRIORITY_TASK() {
    //TODO: uxTopReadyPriority全局变量设置和更新
    //TODO: 函数规范化
    let max_prio = taskSELECT_HIGHEST_PRIORITY();
    // let target:ListItemWeakLink=list_get_head_entry(&READY_TASK_LISTS[max_prio]);
    // let owner:ListItemOwnerWeakLink=list_item_get_owner(&target);
    let owner: ListItemOwnerWeakLink = list_get_owner_of_next_entry(&READY_TASK_LISTS[max_prio]);
    unsafe {
        set_current_tcb(Some(&*(*owner.into_raw()).read()));
        auto_set_currentTcb();
    }
}

pub fn taskSELECT_HIGHEST_PRIORITY() -> usize {
    for i in 1..16 {
        let j = 16 - i;
        if !list_is_empty(&READY_TASK_LISTS[j]) {
            return j;
        }
    }
    return 0;
}

pub fn taskYield() {
    port_yield!();
}

pub fn prvAddCurrentTaskToDelayedList() {}

pub fn vTaskDelay(xTicksToDelay: UBaseType) {
    v_task_enter_critical();
    unsafe {
        if xTicksToDelay < xNextTaskUnblockTime {
            xNextTaskUnblockTime = xTicksToDelay + xTickCount;
        }
        let cur_tcb = pxCurrentTCB_.unwrap();
        let list_item = &(*cur_tcb).xStateListItem;
        list_item_set_value(&Arc::downgrade(&list_item), xTicksToDelay + xTickCount);
        ux_list_remove(Arc::downgrade(&list_item));
        v_list_insert(&DELAYED_TASK_LIST, list_item.clone());
    }
    v_task_exit_critical();
    taskYield();
}

#[no_mangle]
pub extern "C" fn xTaskIncrementTick() {
    //todo
    unsafe {
        xTickCount += 1;
        if xTickCount >= xNextTaskUnblockTime {
            loop {
                if list_is_empty(&DELAYED_TASK_LIST) {
                    xNextTaskUnblockTime = PORT_MAX_DELAY;
                    break;
                } else {
                    let head: ListItemLink =
                        list_get_head_entry(&DELAYED_TASK_LIST).upgrade().unwrap();
                    if head.read().x_item_value <= xTickCount {
                        ux_list_remove(Arc::downgrade(&head));
                        let owner_: ListItemOwnerWeakLink =
                            list_item_get_owner(&Arc::downgrade(&head));
                        let prio: UBaseType = owner_.upgrade().unwrap().read().priority;
                        v_list_insert_end(&READY_TASK_LISTS[prio as usize], head);
                    } else {
                        xNextTaskUnblockTime = head.read().x_item_value;
                        break;
                    }
                }
            }
        }
    }
}

pub fn xPortSysTickHandler() {
    v_task_enter_critical();
    xTaskIncrementTick();
    v_task_exit_critical();
}
