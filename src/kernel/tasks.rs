extern crate alloc;

use crate::kernel::kernel::TCB1_p;
use crate::kernel::kernel::DELAYED_TASK_LIST;
use crate::kernel::kernel::OVERFLOW_DELAYED_TASK_LIST;
use crate::kernel::kernel::READY_TASK_LISTS;
use crate::kernel::linked_list::*;
use crate::kernel::portable::*;
use crate::mtCOVERAGE_TEST_MARKER;
use crate::pdFALSE;
use crate::portDISABLE_INTERRUPTS;
use crate::portENABLE_INTERRUPTS;
use crate::portYIELD;
use crate::portmacro::*;
use alloc::format;
use alloc::string::ToString;
use alloc::sync::{Arc, Weak};
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
use crate::portENTER_CRITICAL;
use crate::portEXIT_CRITICAL;
use crate::riscv_virt::*;
use alloc::string::String;
use core::arch::asm;
use core::clone;
use core::ffi::c_void;

use super::config::USER_STACK_SIZE;
use super::kernel::IDLE_STACK;
use super::kernel::IDLE_p;
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
macro_rules! taskENTER_CRITICAL {
    () => {
        portENTER_CRITICAL!();
    };
}
#[macro_export]
macro_rules! taskEXIT_CRITICAL {
    () => {
        portEXIT_CRITICAL!();
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

#[derive(Debug, Clone)]
pub struct tskTaskControlBlock {
    pub pxTopOfStack: StackType_t_link,
    pxStack: StackType_t_link,
    pcTaskName: String,
    pub xStateListItem: ListItemLink,
    pub uxCriticalNesting: UBaseType_t,
    pub uxPriority: UBaseType,
}
impl Default for tskTaskControlBlock {
    fn default() -> Self {
        tskTaskControlBlock {
            pxStack: 0,
            pxTopOfStack: 0,
            pcTaskName: String::new(),
            xStateListItem: Default::default(),
            uxCriticalNesting: 0,
            uxPriority: 0,
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
#[cfg(feature = "configSUPPORT_STATIC_ALLOCATION")]
pub fn xTaskCreateStatic(
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    puxStackBuffer: Option<StackType_t_link>,
    pxTaskBuffer: Option<TCB_t_link>,
    uxPriority: UBaseType,
) -> Option<TaskHandle_t> {
    print("xTaskCreateStatic 1111");
    assert!(puxStackBuffer.is_some());
    assert!(pxTaskBuffer.is_some());
    //TODO: C:
    //     #if ( configASSERT_DEFINED == 1 )
    //     {
    //         /* Sanity check that the size of the structure used to declare a
    //          * variable of type StaticTask_t equals the size of the real task
    //          * structure. */
    //         volatile size_t xSize = sizeof( StaticTask_t );
    //         configASSERT( xSize == sizeof( TCB_t ) );
    //         ( void ) xSize; /* Prevent lint warning when configASSERT() is not used. */
    //     }
    // #endif /* configASSERT_DEFINED */
    //TODO: fix xReturn
    let xReturn = Arc::new(RwLock::new(tskTaskControlBlock::default()));

    let pxNewTCB: TCB_t_link = pxTaskBuffer.unwrap().clone();
    TCB_set_pxStack(&pxNewTCB, puxStackBuffer.unwrap());
    //TODO: C:
    //     #if ( tskSTATIC_AND_DYNAMIC_ALLOCATION_POSSIBLE != 0 ) /*lint !e731 !e9029 Macro has been consolidated for readability reasons. */
    //     {
    //         /* Tasks can be created statically or dynamically, so note this
    //          * task was created statically in case the task is later deleted. */
    //         pxNewTCB->ucStaticallyAllocated = tskSTATICALLY_ALLOCATED_STACK_AND_TCB;
    //     }
    // #endif /* tskSTATIC_AND_DYNAMIC_ALLOCATION_POSSIBLE */
    prvInitialiseNewTask(
        pxTaskCode,
        pcName,
        ulStackDepth,
        pvParameters,
        &xReturn,
        uxPriority,
        pxNewTCB.clone(),
    );
    print("xTaskCreateStatic 3333");
    prvAddNewTaskToReadyList(pxNewTCB.clone());
    Some(xReturn)
}
pub fn prvAddNewTaskToReadyList(pxNewTCB: TCB_t_link) {
    // taskENTER_CRITICAL!();
    {
        //TODO:
        prvAddTaskToReadyList(pxNewTCB);
    }
    // taskEXIT_CRITICAL!();
}
pub fn prvAddTaskToReadyList(pxNewTCB: TCB_t_link) {
    let uxPriority = pxNewTCB.read().uxPriority;
    let s_ = format!("uxPriority{:X}", uxPriority);
    print(&s_);
    taskRECORD_READY_PRIORITY(uxPriority);
    v_list_insert_end(
        &READY_TASK_LISTS[uxPriority as usize],
        (pxNewTCB.read().xStateListItem).clone(),
    );
}
pub fn taskRECORD_READY_PRIORITY(uxPriority: UBaseType) {
    //TODO: set max uxTopReadyPriority
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
    pxNewTCB.write().uxPriority = priority;
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
    *pxCreatedTask.write() = (*(pxNewTCB.write())).clone();
    pxNewTCB
}
pub fn prvIdleTask(t: *mut c_void) {
    vSendString("idle gogogogo");
    loop {
        vSendString("idle gogogogo!!!(in loop)");
    }
}
pub fn vTaskStartScheduler() {
    unsafe {
        X_SCHEDULER_RUNNING = pdTRUE!();
    }
    if cfg!(feature = "configSUPPORT_STATIC_ALLOCATION") {
        let param: Param_link = 0;
        let stack2ptr: StackType_t_link =
        &*IDLE_STACK as *const [u32; USER_STACK_SIZE] as *const u32 as usize + USER_STACK_SIZE * 4
            - 4;
        xTaskCreateStatic(
            prvIdleTask as u32,
            "idle",
            USER_STACK_SIZE as u32,
            Some(param),
            Some(stack2ptr),
            Some(IDLE_p.clone()),
            0,
        );
    }
    set_current_tcb(Some(Arc::downgrade(&IDLE_p)));
    print("set tcb success");
    if x_port_start_scheduler() != pdFALSE!() {
        panic!("error scheduler!!!!!!");
    }
}

fn prvInitialiseTaskLists() {
    //initial in list impl
}

pub fn vTaskEnterCritical() {
    portDISABLE_INTERRUPTS!();
    unsafe {
        if X_SCHEDULER_RUNNING != pdFALSE!() {
            get_current_tcb().unwrap().uxCriticalNesting += 1;
            if get_current_tcb().unwrap().uxCriticalNesting == 1 {
                // TODO: portASSERT_IF_IN_ISR
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
}

pub fn vTaskExitCritical() {
    unsafe {
        if X_SCHEDULER_RUNNING != pdFALSE!() {
            if get_current_tcb().unwrap().uxCriticalNesting > 0 {
                get_current_tcb().unwrap().uxCriticalNesting -= 1;
                if get_current_tcb().unwrap().uxCriticalNesting == 0 {
                    portENABLE_INTERRUPTS!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
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
        set_current_tcb(Some(owner));
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
    portYIELD!();
}

pub fn prvAddCurrentTaskToDelayedList() {}

pub fn vTaskDelay(xTicksToDelay: UBaseType) {
    vTaskEnterCritical();
    unsafe {
        let xTimeToWake = xTicksToDelay + xTickCount;
        let list_item = &get_current_tcb().unwrap().xStateListItem;
        list_item_set_value(&Arc::downgrade(&list_item), xTimeToWake);
        ux_list_remove(Arc::downgrade(&list_item));
        if xTimeToWake > xTickCount {
            v_list_insert(&DELAYED_TASK_LIST, list_item.clone());
            if xTicksToDelay < xNextTaskUnblockTime {
                xNextTaskUnblockTime = xTimeToWake;
            }
        } else {
            v_list_insert(&OVERFLOW_DELAYED_TASK_LIST, list_item.clone());
        }
    }
    vTaskExitCritical();
    taskYield();
}

fn prvResetNextTaskUnblockTime() {}

fn taskSWITCH_DELAYED_LISTS() {
    let mut delayed = DELAYED_TASK_LIST.write();
    let mut overflowed = OVERFLOW_DELAYED_TASK_LIST.write();
    let tmp = (*delayed).clone();
    *delayed = (*overflowed).clone();
    *overflowed = tmp;
}

#[no_mangle]
pub extern "C" fn xTaskIncrementTick() {
    //todo
    unsafe {
        xTickCount += 1;
        if xTickCount == 0 {
            taskSWITCH_DELAYED_LISTS();
        }

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
                        let prio: UBaseType = owner_.upgrade().unwrap().read().uxPriority;
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
    vTaskEnterCritical();
    xTaskIncrementTick();
    vTaskExitCritical();
}
#[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]

pub fn xTaskCreate(
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,

    uxPriority: UBaseType,
    pxCreatedTask: Option<TaskHandle_t>,
) -> BaseType {
    let xReturn: BaseType = 0;
    let mut pxStack: StackType_t_link = 0;
    // let stack:[u32;ulStackDepth]= [0;ulStackDepth];
    print("xTaskCreate 11111111");
    use alloc::alloc::Layout;

    use alloc::vec::Vec;
    let layout = Layout::from_size_align(ulStackDepth as usize * 4, 4)
        .ok()
        .unwrap();
    let stack_ptr: *mut u8;
    unsafe {
        stack_ptr = alloc::alloc::alloc(layout);
    }
    pxStack = stack_ptr as usize + ulStackDepth as usize * 4 - 4;
    // let stack: Vec<usize> = Vec::with_capacity(ulStackDepth as usize);

    let pxNewTCB: TCB_t_link = Arc::new(RwLock::new(tskTaskControlBlock::default()));
    TCB_set_pxStack(&pxNewTCB, pxStack);
    let s_ = format!("top of stack{:X}", pxNewTCB.read().pxTopOfStack);
    print(&s_);
    prvInitialiseNewTask(
        pxTaskCode,
        pcName,
        ulStackDepth,
        pvParameters,
        &pxCreatedTask.unwrap(),
        uxPriority,
        pxNewTCB.clone(),
    );
    prvAddNewTaskToReadyList(pxNewTCB.clone());
    1
}
// macro_rules! taskENTER_CRITICAL_FROM_ISR {
//     () => {
//         portSET_INTERRUPT_MASK_FROM_ISR();
//     };
// }
