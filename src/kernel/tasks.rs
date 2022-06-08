//! task control api

extern crate alloc;
use crate::configMAX_PRIORITIES;
use crate::kernel::kernel::*;
use crate::kernel::linked_list::*;
use crate::kernel::portable::*;
use crate::kernel::projdefs::*;
use crate::mtCOVERAGE_TEST_MARKER;
use crate::portDISABLE_INTERRUPTS;
use crate::portENABLE_INTERRUPTS;
use crate::portYIELD;
use crate::portmacro::*;
#[cfg(feature = "configUSE_IDLE_HOOK")]
use crate::vApplicationIdleHook;

use alloc::format;
use alloc::string::ToString;
use alloc::sync::{Arc, Weak};
use core::cmp::max;
use core::mem::size_of;
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
use crate::portYIELD_WITHIN_API;
use crate::riscv_virt::*;
use alloc::string::String;
use core::arch::asm;
use core::clone;
use core::ffi::c_void;
use core::ptr::NonNull;

use super::config::USER_STACK_SIZE;
use super::kernel::IDLE_p;
use super::kernel::IDLE_STACK;
pub static tskIDLE_PRIORITY: UBaseType = 0;
pub static mut XSCHEDULERRUNNING: BaseType = pdFALSE;
pub static mut xTickCount: UBaseType = 0;
pub static mut xNumOfOverflows: BaseType = 0;
pub static mut xNextTaskUnblockTime: UBaseType = PORT_MAX_DELAY;
pub static mut uxCurrentNumberOfTasks: UBaseType = 0;
pub static mut uxSchedulerSuspended: UBaseType = 0;
pub static mut xPendedTicks: UBaseType = 0;
pub static mut xYieldPending: bool = false;
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
macro_rules! vTaskMissedYield {
    () => {
        unsafe {
            xYieldPending = true;
        }
    };
}

pub const taskEVENT_LIST_ITEM_VALUE_IN_USE: UBaseType = 0x8000;

extern "C" {
    /// initialise task stack space ( Extern C )
    pub fn pxPortInitialiseStack(
        pxTopOfStack: *mut StackType_t,
        pxCode: usize,
        pvParameters: *mut c_void,
    ) -> *mut StackType_t;
}

#[derive(Default)]
pub struct TimeOut {
    pub xOverflowCount: BaseType,
    pub xTimeOnEntering: TickType,
}
///
/// tskTaskControlBlock
#[derive(Debug, Clone)]
pub struct tskTaskControlBlock {
    ///Stack top pointer
    pub pxTopOfStack: StackType_t_link,
    ///Stack bottom pointer
    pxStack: StackType_t_link,
    ///Task name
    pcTaskName: String,
    ///Task status list pointer
    pub xStateListItem: ListItemLink,
    ///Task evnet list pointer
    pub xEventListItem: ListItemLink,
    ///
    pub uxCriticalNesting: UBaseType_t,
    /// Task priority
    pub uxPriority: UBaseType,
    pub uxMutexesHeld: UBaseType,
    pub uxBasePriority: UBaseType,
    /// mark for ffi
    pub build_from_c: bool,
}
impl Default for tskTaskControlBlock {
    fn default() -> Self {
        tskTaskControlBlock {
            pxStack: 0,
            pxTopOfStack: 0,
            pcTaskName: String::new(),
            xStateListItem: Default::default(),
            xEventListItem: Default::default(),
            uxCriticalNesting: 0,
            uxPriority: 0,
            uxBasePriority: 0,
            uxMutexesHeld: 0,
            build_from_c: false,
        }
    }
}

/// set pxStack of target tcb
pub fn TCB_set_pxStack(tcb: &TCB_t_link, item: StackType_t_link) {
    //TODO: item owner
    tcb.write().pxStack = item;
}

pub type tskTCB = tskTaskControlBlock;
pub type TCB_t = tskTCB;
//TaskHandle_t=tskTaskControlBlock*
pub type TaskHandle_t = Arc<RwLock<tskTaskControlBlock>>;

/// set priority of target task
pub fn vTaskPrioritySet(pxTask: Option<TaskHandle_t>, uxNewPriority: UBaseType) {
    vTaskEnterCritical();
    match pxTask {
        Some(x) => {
            ux_list_remove(Arc::downgrade(&x.read().xStateListItem));
            v_list_insert_end(
                &READY_TASK_LISTS[uxNewPriority as usize],
                Arc::clone(&x.read().xStateListItem),
            );
            x.write().uxPriority = uxNewPriority;
            list_item_set_value(
                &x.write().xEventListItem,
                configMAX_PRIORITIES - uxNewPriority,
            );
        }
        None => unsafe {
            match get_current_tcb() {
                Some(x) => {
                    ux_list_remove(Arc::downgrade(&(*x).xStateListItem));
                    v_list_insert_end(
                        &READY_TASK_LISTS[uxNewPriority as usize],
                        Arc::clone(&(*x).xStateListItem),
                    );
                    x.uxPriority = uxNewPriority;
                    list_item_set_value(&x.xEventListItem, configMAX_PRIORITIES - uxNewPriority);
                }
                None => {}
            }
        },
    }
    vTaskExitCritical();
}

/// get priority of target task
pub fn uxTaskPriorityGet(pxTask: Option<TaskHandle_t>) -> UBaseType {
    unsafe {
        match get_current_tcb() {
            Some(x) => unsafe {
                return (*x).uxPriority;
            },
            None => {
                return 0;
            }
        }
    }
}

/// Create tasks static
///
///
#[cfg(feature = "configSUPPORT_STATIC_ALLOCATION")]
pub fn xTaskCreateStatic(
    pxTaskCode: usize,
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
    let s_ = format!("top of stack{:X}", pxNewTCB.read().pxTopOfStack);
    print(&s_);
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

/// add new task to ready list
pub fn prvAddNewTaskToReadyList(pxNewTCB: TCB_t_link) {
    // taskENTER_CRITICAL!();
    {
        //TODO:
        prvAddTaskToReadyList(pxNewTCB);
    }
    // taskEXIT_CRITICAL!();
}

/// add task to ready list
pub fn prvAddTaskToReadyList(pxNewTCB: TCB_t_link) {
    let uxPriority = pxNewTCB.read().uxPriority;
    let s = format!(
        "add to readylist{:X}",
        pxNewTCB.read().xStateListItem.read().x_item_value
    );
    print(&s);

    taskRECORD_READY_PRIORITY(uxPriority);
    v_list_insert_end(
        &READY_TASK_LISTS[uxPriority as usize],
        (pxNewTCB.read().xStateListItem).clone(),
    );
}
///set max uxTopReadyPriority
pub fn taskRECORD_READY_PRIORITY(uxPriority: UBaseType) {
    //TODO: set max uxTopReadyPriority
}

/// initialise new task
pub fn prvInitialiseNewTask(
    pxTaskCode: usize,
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
    if cfg!(feature = "configUSE_MUTEXES") {
        pxNewTCB.write().uxBasePriority = priority;
        pxNewTCB.write().uxMutexesHeld = 0;
    }
    //TODO:auto init
    print("prvInitialiseNewTask 2222");
    list_item_set_owner(&pxNewTCB.write().xStateListItem, Arc::downgrade(&pxNewTCB));
    list_item_set_owner(&pxNewTCB.write().xEventListItem, Arc::downgrade(&pxNewTCB));
    list_item_set_value(
        &pxNewTCB.write().xEventListItem,
        configMAX_PRIORITIES - priority,
    );
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

/// idle task function
pub fn prvIdleTask(t: *mut c_void) {
    vSendString("idle gogogogo");
    loop {
        #[cfg(feature = "configUSE_IDLE_HOOK")]
        {
            /* Call the user defined function from within the idle task.  This
             * allows the application designer to add background functionality
             * without the overhead of a separate task.
             * NOTE: vApplicationIdleHook() MUST NOT, UNDER ANY CIRCUMSTANCES,
             * CALL A FUNCTION THAT MIGHT BLOCK. */
            vApplicationIdleHook();
        } /* configUSE_IDLE_HOOK */
        vSendString("idle gogogogo!!!(in loop)");
    }
}

/// start scheduler
#[no_mangle]
pub extern "C" fn vTaskStartScheduler() {
    unsafe {
        XSCHEDULERRUNNING = pdTRUE;
    }
    if cfg!(feature = "configSUPPORT_STATIC_ALLOCATION") {
        let param: Param_link = 0;
        let size_:usize=size_of::<usize>();
        let stack2ptr: StackType_t_link = &*IDLE_STACK as *const [usize; USER_STACK_SIZE]
            as *const usize as usize
            + USER_STACK_SIZE * size_
            - size_;
        xTaskCreateStatic(
            prvIdleTask as usize,
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
    if x_port_start_scheduler() != pdFALSE {
        panic!("error scheduler!!!!!!");
    }
}

// fn prvInitialiseTaskLists() {
//     //initial in list impl
// }
/// Enter task critical area
pub fn vTaskEnterCritical() {
    portDISABLE_INTERRUPTS!();
    unsafe {
        if XSCHEDULERRUNNING != pdFALSE {
            match get_current_tcb() {
                Some(x) => {
                    x.uxCriticalNesting += 1;
                    if x.uxCriticalNesting == 1 {
                        // TODO: portASSERT_IF_IN_ISR
                    }
                }
                None => (),
            }
            // get_current_tcb().unwrap().uxCriticalNesting += 1;
            // if get_current_tcb().unwrap().uxCriticalNesting == 1 {
            //     // TODO: portASSERT_IF_IN_ISR
            // }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
}
/// Exit task critical area
pub fn vTaskExitCritical() {
    unsafe {
        if XSCHEDULERRUNNING != pdFALSE {
            match get_current_tcb() {
                Some(x) => {
                    if x.uxCriticalNesting > 0 {
                        x.uxCriticalNesting -= 1;
                        if x.uxCriticalNesting == 0 {
                            portENABLE_INTERRUPTS!();
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
                None => (),
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
}

/// set current tcb to task with highest priority
/// when want to start highest priority task call taskSELECT_HIGHEST_PRIORITY_TASK
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

/// find highest priority with valid task
pub fn taskSELECT_HIGHEST_PRIORITY() -> usize {
    for i in 1..16 {
        let j = 16 - i;
        if !list_is_empty(&READY_TASK_LISTS[j]) {
            return j;
        }
    }
    return 0;
}

/// yield in task
pub fn taskYield() {
    portYIELD!();
}

/// add current task to delayed list
pub fn prvAddCurrentTaskToDelayedList(xTicksToWait: TickType, xCanBlockIndefinitely: bool) {
    //todo
    //vTaskEnterCritical();
    let mut xTimeToWake: TickType;
    let mut xConstTickCount: TickType;
    unsafe {
        xTimeToWake = xTicksToWait + xTickCount;
        xConstTickCount = xTickCount;
    }
    let list_item = &get_current_tcb().unwrap().xStateListItem;
    list_item_set_value(&list_item, xTimeToWake);
    ux_list_remove(Arc::downgrade(&list_item));
    if xTimeToWake > xConstTickCount {
        v_list_insert(&DELAYED_TASK_LIST, list_item.clone());
        unsafe {
            if xTimeToWake < xNextTaskUnblockTime {
                xNextTaskUnblockTime = xTimeToWake;
            }
        }
    } else {
        v_list_insert(&OVERFLOW_DELAYED_TASK_LIST, list_item.clone());
    }
    //vTaskExitCritical();
}

#[no_mangle]
/// Delay Task xTicksToDelay Relativly
/// used in task function
/// # Examples
/// ```
/// fn task_example(t: *mut c_void){
///     loop{
///         vTaskDelay(1000);
///     }
/// }
/// ```
pub extern "C" fn vTaskDelay(xTicksToDelay: TickType) {
    vTaskEnterCritical();
    //todo
    prvAddCurrentTaskToDelayedList(xTicksToDelay, true);

    vTaskExitCritical();
    taskYield();
}

#[no_mangle]
//TODO: exampke
/// Delay task until pxPreviousWakeTime+pxPreviousWakeTime
/// used in task function
/// # Examples
/// ```
/// fn task_example(t: *mut c_void){
///     let mut PreviousWakeTime=0;
///     let TimeIncrement=100;
///     loop{
///         xTaskDelayUntil(&PreviousWakeTime,TimeIncrement);
///     }
/// }
/// ```
pub extern "C" fn xTaskDelayUntil(pxPreviousWakeTime: &mut TickType, xTimeIncrement: TickType) {
    let mut xShouldDelay: bool = false;

    vTaskSuspendAll();
    {
        let mut xConstTickCount: TickType;
        unsafe {
            xConstTickCount = xTickCount;
        }

        let xTimeToWake: TickType = *pxPreviousWakeTime + xTimeIncrement;
        //let s=format!("xConstTickCount:{} pxPreviousWakeTime:{} xTimeToWake:{}",xConstTickCount,*pxPreviousWakeTime,xTimeToWake);
        //vSendString(&s);
        if xConstTickCount < *pxPreviousWakeTime {
            if (xTimeToWake < *pxPreviousWakeTime) && (xTimeToWake > xConstTickCount) {
                xShouldDelay = true;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            if (xTimeToWake < *pxPreviousWakeTime) || (xTimeToWake > xConstTickCount) {
                xShouldDelay = true;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }

        *pxPreviousWakeTime = xTimeToWake;

        if xShouldDelay == true {
            prvAddCurrentTaskToDelayedList(xTimeToWake - xConstTickCount, true);
        } else {
            //vSendString("no delay!");
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    let xAlreadyYielded: bool = vTaskResumeAll();
    if xAlreadyYielded == false {
        portYIELD_WITHIN_API!();
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }

    xShouldDelay;
}

/// in case of mtime overflow, swap delayed list and overflow list
fn taskSWITCH_DELAYED_LISTS() {
    let mut delayed = DELAYED_TASK_LIST.write();
    let mut overflowed = OVERFLOW_DELAYED_TASK_LIST.write();
    let tmp = (*delayed).clone();
    *delayed = (*overflowed).clone();
    *overflowed = tmp;
    unsafe {
        xNumOfOverflows += 1;
    }
}

/// tick increment, free delayed task
#[no_mangle]
pub extern "C" fn xTaskIncrementTick() {
    //todo
    unsafe {
        if uxSchedulerSuspended == 0 {
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
                            let owner: TaskHandle_t;
                            let temp = list_item_get_c_owner(&Arc::downgrade(&head));
                            let from_c: bool;
                            match temp {
                                Some(x) => {
                                    owner = x;
                                    from_c = true;
                                }
                                None => {
                                    owner = list_item_get_owner(&Arc::downgrade(&head))
                                        .upgrade()
                                        .unwrap();
                                    from_c = false;
                                }
                            }
                            let prio: UBaseType = owner.read().uxPriority;
                            v_list_insert_end(&READY_TASK_LISTS[prio as usize], head);
                            if from_c {
                                let temp_ = Arc::into_raw(owner);
                            }
                        } else {
                            xNextTaskUnblockTime = head.read().x_item_value;
                            break;
                        }
                    }
                }
            }
        } else {
            xPendedTicks += 1;
        }
    }
}

pub fn xPortSysTickHandler() {
    vTaskEnterCritical();
    xTaskIncrementTick();
    vTaskExitCritical();
}
#[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]

/// create task dynamic
///
/// # Examples
///
/// ```
/// lazy_static! {
/// pub static ref task1handler: Option<TaskHandle_t> =
///     Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
/// }
/// fn task_example(t: *mut c_void){
///     loop{
///         //do somthing
///     }
/// }
/// fn main(){
///     let params=0;
///     xTaskCreate(
///     task_example as u32,    //task define ,should be same as example defination
///     "task_example",         //task name
///     USER_STACK_SIZE as u32, //task stack depth
///     Some(params),           //task params
///     3,                      //task priority
///     Some(Arc::clone(&(task1handler.as_ref().unwrap()))), //task handler
///     );
///
/// }
/// ```
///
pub fn xTaskCreate(
    pxTaskCode: usize,
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
    use core::mem;

    use alloc::alloc::Layout;

    use alloc::vec::Vec;
    let size_:usize=size_of::<usize>();
    let layout = Layout::from_size_align(ulStackDepth as usize * size_, size_)
        .ok()
        .unwrap();
    let stack_ptr: *mut u8;
    unsafe {
        stack_ptr = alloc::alloc::alloc(layout);
    }
    pxStack = stack_ptr as usize + ulStackDepth as usize * size_ - size_;
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
    mem::forget(pxNewTCB);
    1
}
// macro_rules! taskENTER_CRITICAL_FROM_ISR {
//     () => {
//         portSET_INTERRUPT_MASK_FROM_ISR();
//     };
// }

pub enum eTaskState {
    eRunning = 0,
    eReady = 1,
    eBlocked = 2,
    eSuspended = 3,
    eDeleted = 4,
    eInvalid = 5,
}
#[macro_export]
macro_rules! get_scheduler_running {
    () => {
        unsafe { crate::xSchedulerRunning }
    };
}
#[macro_export]
macro_rules! taskYIELD_IF_USING_PREEMPTION {
    () => {
        portYIELD_WITHIN_API!();
    };
}
#[macro_export]
macro_rules! get_uxCurrentNumberOfTasks {
    () => {
        unsafe { crate::uxCurrentNumberOfTasks }
    };
}

/// suspend task until resumed
/// assert params is not Option::None
#[cfg(feature = "INCLUDE_vTaskSuspend")]
pub fn vTaskSuspend(xTaskToSuspend_: Option<TaskHandle_t>) {
    /*
    默认传入有效handle or curtcb
     */
    let xTaskToSuspend = prvGetTCBFromHandle(xTaskToSuspend_).unwrap();
    use crate::kernel::kernel::SUSPENDED_TASK_LIST;
    taskENTER_CRITICAL!();
    {
        //let pxTCB = xTaskToSuspend;
        // let pxTCB = prvGetTCBFromHandle(xTaskToSuspend);
        /* 从就绪/阻塞列表中删除任务并放入挂起列表中。 */

        if ux_list_remove(Arc::downgrade(&xTaskToSuspend.xStateListItem)) == 0 {
            // taskRESET_READY_PRIORITY( pxTCB->uxPriority );
            //TODO:
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
        ux_list_remove(Arc::downgrade(&xTaskToSuspend.xEventListItem));
        v_list_insert_end(&SUSPENDED_TASK_LIST, xTaskToSuspend.xStateListItem.clone());
    }
    taskEXIT_CRITICAL!();

    if (get_scheduler_running!() != false) {
        taskENTER_CRITICAL!();
        {
            prvResetNextTaskUnblockTime(); //TODO:
        }
        taskEXIT_CRITICAL!();
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
    // if ( pxTCB == pxCurrentTCB ){//TODO: pxCurrentTCB

    if is_current_tcb_raw(xTaskToSuspend) {
        if get_scheduler_running!() {
            /* The current task has just been suspended. */
            // assert!(get_scheduler_suspended!() == 0);
            // portYIELD_WITHIN_API!();
            print("R_FreeRTOS paniced! portYIELD_WITHIN_API");
        } else {
            //             if ( listCURRENT_LIST_LENGTH( &xSuspendedTaskList )
            // 71 == uxCurrentNumberOfTasks ) { (10)
            // 72 /* 没有其他任务准备就绪，因此将 pxCurrentTCB 设置回 NULL，
            // 73 以便在创建下一个任务时 pxCurrentTCB 将被设置为指向它，
            // 74 实际上并不会执行到这里 */
            // 75
            // 76 pxCurrentTCB = NULL; (11)
            // 77 } else {
            // 78 /* 有其他任务，则切换到其他任务 */
            // 79
            // 80 vTaskSwitchContext(); (12)
            // 81 }
            // 82
            if list_current_list_length(&SUSPENDED_TASK_LIST) != get_uxCurrentNumberOfTasks!() {
                /* 没有其他任务准备就绪，因此将 pxCurrentTCB 设置回 NULL，
                以便在创建下一个任务时 pxCurrentTCB 将被设置为指向它，
                实际上并不会执行到这里 */
                // pxCurrentTCB = NULL
            } else {
                /* 有其他任务，则切换到其他任务 */
                vTaskSwitchContext();
            }
        }
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}
pub static mut xSchedulerRunning: bool = false;

// pub fn prvGetTCBFromHandle(xTaskToSuspend: TaskHandle_t) -> RwLock<TCB_t> {
//     let x=xTaskToSuspend.read();
//     x
// }

/// resume target task
#[cfg(feature = "INCLUDE_vTaskSuspend")]
pub fn vTaskResume(xTaskToResume_: Option<TaskHandle_t>) {
    //TODO: 检查要恢复的任务是否被挂起
    //TODO：assert is not None &&pxTCB != pxCurrentTCB
    let xTaskToResume = xTaskToResume_.unwrap();
    let mut pxTCB = xTaskToResume.read();
    if is_current_tcb(Arc::downgrade(&xTaskToResume)) == false {
        taskENTER_CRITICAL!();
        {
            if prvTaskIsTaskSuspended(&xTaskToResume) != false {
                ux_list_remove(Arc::downgrade(&pxTCB.xStateListItem));
                prvAddNewTaskToReadyList(xTaskToResume.clone());
                if (pxTCB.uxPriority >= get_current_tcb().unwrap().uxPriority) {
                    /* 因为恢复的任务在当前情况下的优先级最高
                    36 调用 taskYIELD_IF_USING_PREEMPTION()进行一次任务切换*/
                    // 37 taskYIELD_IF_USING_PREEMPTION();
                    taskYIELD_IF_USING_PREEMPTION!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        taskEXIT_CRITICAL!();
    }
}

pub fn prvTaskIsTaskSuspended(xTaskToResume: &TaskHandle_t) -> bool {
    true
}

/// get tcb from handle, return current tcb if handle is None
pub fn prvGetTCBFromHandle(
    handle: Option<TaskHandle_t>,
) -> Option<&'static mut tskTaskControlBlock> {
    match handle {
        Some(x) => unsafe {
            let temp = &*(*Arc::into_raw(x)).read() as *const tskTaskControlBlock;
            Some(&mut *(temp as *mut tskTaskControlBlock))
        },
        None => get_current_tcb(),
    }
}

/// delete target task
pub fn vTaskDelete(xTaskToDelete: Option<TaskHandle_t>) {
    taskENTER_CRITICAL!();
    let pxTCB = prvGetTCBFromHandle(xTaskToDelete.clone());
    ux_list_remove(Arc::downgrade(&pxTCB.unwrap().xStateListItem));
    //todo：事件相关处理
    //todo：任务和tcb内存释放
    //todo：钩子函数
    taskEXIT_CRITICAL!();
    let need_yield = match xTaskToDelete {
        Some(x) => is_current_tcb(Arc::downgrade(&x)),
        None => true,
    };
    if need_yield {
        portYIELD!();
    }
}

/// reset NextTaskUnblockTime
fn prvResetNextTaskUnblockTime() {
    unsafe {
        if list_is_empty(&DELAYED_TASK_LIST) {
            xNextTaskUnblockTime = PORT_MAX_DELAY;
        } else {
            let head = list_get_head_entry(&DELAYED_TASK_LIST).upgrade().unwrap();
            xNextTaskUnblockTime = head.read().x_item_value;
        }
    }
}

#[no_mangle]
/// suspend scheduler
pub extern "C" fn vTaskSuspendAll() {
    unsafe {
        uxSchedulerSuspended += 1;
    }
}

#[no_mangle]
/// resume scheduler
pub extern "C" fn vTaskResumeAll() -> bool {
    let mut xAlreadyYielded = false;
    let mut moved = false;
    unsafe {
        uxSchedulerSuspended -= 1;
        taskENTER_CRITICAL!();
        if uxSchedulerSuspended == 0 {
            if get_uxCurrentNumberOfTasks!() > 0 {
                while !list_is_empty(&PENDING_READY_LIST) {
                    let pxTCB = list_get_owner_of_next_entry(&PENDING_READY_LIST)
                        .upgrade()
                        .unwrap();
                    ux_list_remove(Arc::downgrade(&pxTCB.read().xEventListItem));
                    ux_list_remove(Arc::downgrade(&pxTCB.read().xStateListItem));

                    if pxTCB.read().uxPriority >= get_current_tcb().unwrap().uxPriority {
                        xYieldPending = true;
                    }
                    prvAddNewTaskToReadyList(pxTCB);

                    moved = true;
                }
                if moved {
                    prvResetNextTaskUnblockTime();
                }
                let mut xPendedTicks_ = xPendedTicks;
                if xPendedTicks_ > 0 {
                    while xPendedTicks_ > 0 {
                        xTaskIncrementTick(); //todo return value

                        xPendedTicks_ -= 1;
                    }
                    xYieldPending = true;
                    xPendedTicks = 0;
                }
                if xYieldPending {
                    if cfg!(feature = "configUSE_PREEMPTION") {
                        xAlreadyYielded = true;
                    }
                    portYIELD_WITHIN_API!();
                }
            }
        }
        taskEXIT_CRITICAL!();
    }
    xAlreadyYielded
}

/// remove first task from event list, and insert the task to ready list
pub fn xTaskRemoveFromEventList(pxEventList: &ListRealLink) -> bool {
    let pxUnblockedTCB: TaskHandle_t;
    let from_c: bool;
    let test = list_get_c_owner_of_head_entry(pxEventList);
    match test {
        Some(x) => {
            pxUnblockedTCB = x;
            from_c = true;
        }
        None => {
            pxUnblockedTCB = list_get_owner_of_head_entry(pxEventList).upgrade().unwrap();
            from_c = false;
        }
    }

    let xReturn: bool;
    let uxSchedulerSuspended_: UBaseType;
    unsafe {
        uxSchedulerSuspended_ = uxSchedulerSuspended;
    }
    ux_list_remove(Arc::downgrade(&pxUnblockedTCB.read().xEventListItem));
    if uxSchedulerSuspended_ == 0 {
        //vSendString("no suspend");
        ux_list_remove(Arc::downgrade(&pxUnblockedTCB.read().xStateListItem));
        prvAddTaskToReadyList(pxUnblockedTCB.clone());
        if cfg!(feature = "configUSE_TICKLESS_IDLE") {
            prvResetNextTaskUnblockTime();
        }
    } else {
        //vSendString("suspend");
        v_list_insert_end(
            &PENDING_READY_LIST,
            pxUnblockedTCB.read().xEventListItem.clone(),
        );
    }
    if pxUnblockedTCB.read().uxPriority > get_current_tcb().unwrap().uxPriority {
        xReturn = true;
        unsafe {
            xYieldPending = true;
        }
    } else {
        xReturn = false;
    }
    if from_c {
        let temp = Arc::into_raw(pxUnblockedTCB);
    }
    xReturn
}

/// set pxTimeOut to current time (in kernel)
pub fn vTaskInternalSetTimeOutState(pxTimeOut: &mut TimeOut) {
    unsafe {
        pxTimeOut.xOverflowCount = xNumOfOverflows;
        pxTimeOut.xTimeOnEntering = xTickCount;
    }
}

/// set pxTimeOut to current time (in task)
pub fn vTaskSetTimeOutState(pxTimeOut: &mut TimeOut) {
    taskENTER_CRITICAL!();
    unsafe {
        pxTimeOut.xOverflowCount = xNumOfOverflows;
        pxTimeOut.xTimeOnEntering = xTickCount;
    }
    taskEXIT_CRITICAL!();
}

/// return if timeout has been reached
pub fn xTaskCheckForTimeOut(pxTimeOut: &mut TimeOut, pxTicksToWait: &mut TickType) -> BaseType {
    let xReturn: BaseType;
    taskENTER_CRITICAL!();
    {
        let xConstTickCount: TickType;
        unsafe {
            xConstTickCount = xTickCount;
        }
        let xElapsedTime = xConstTickCount - pxTimeOut.xTimeOnEntering;
        if cfg!(feature = "INCLUDE_xTaskAbortDelay") {
            //todo
        }

        if cfg!(feature = "INCLUDE_vTaskSuspend") {
            if *pxTicksToWait == PORT_MAX_DELAY {
                taskEXIT_CRITICAL!();
                return pdFALSE;
            }
        }
        let xNumOfOverflows_: BaseType;
        unsafe {
            xNumOfOverflows_ = xNumOfOverflows;
        }
        if xNumOfOverflows_ != pxTimeOut.xOverflowCount
            && xConstTickCount >= pxTimeOut.xTimeOnEntering
        {
            xReturn = pdTRUE;
            *pxTicksToWait = 0;
        } else if xElapsedTime < *pxTicksToWait {
            *pxTicksToWait -= xElapsedTime;
            xReturn = pdFALSE;
        } else {
            xReturn = pdTRUE;
            *pxTicksToWait = 0;
        }
    }
    taskEXIT_CRITICAL!();
    xReturn
}

/// inherit mutex holder task's priority to current task's priority <br>
/// return if the inherit was successful
pub fn xTaskPriorityInherit(pxMutexHolder: Option<TaskHandle_t>) -> BaseType {
    let mut xReturn: BaseType = pdFALSE;
    match pxMutexHolder {
        Some(pxMutexHolder_) => {
            let pxMutexHolderTCB: &mut tskTaskControlBlock = &mut pxMutexHolder_.write();
            if pxMutexHolderTCB.uxPriority < get_current_tcb().unwrap().uxPriority {
                if list_item_get_value(&pxMutexHolderTCB.xEventListItem)
                    & taskEVENT_LIST_ITEM_VALUE_IN_USE
                    == 0
                {
                    list_item_set_value(
                        &pxMutexHolderTCB.xEventListItem,
                        configMAX_PRIORITIES - pxMutexHolderTCB.uxPriority,
                    );
                }

                if list_is_contained_within(
                    &READY_TASK_LISTS[pxMutexHolderTCB.uxPriority as usize],
                    &pxMutexHolderTCB.xStateListItem,
                ) == true
                {
                    ux_list_remove(Arc::downgrade(&pxMutexHolderTCB.xStateListItem));
                    pxMutexHolderTCB.uxPriority = get_current_tcb().unwrap().uxPriority;
                    prvAddTaskToReadyList(pxMutexHolder_.clone());
                } else {
                    pxMutexHolderTCB.uxPriority = get_current_tcb().unwrap().uxPriority;
                }
                xReturn = pdTRUE;
            } else if pxMutexHolderTCB.uxPriority > get_current_tcb().unwrap().uxPriority {
                xReturn = pdTRUE;
            }
        }
        None => {
            mtCOVERAGE_TEST_MARKER!();
        }
    }

    xReturn
}

/// disinherit and recover original priority for mutex holder task <br>
/// disinherit priority only when no other mutex are held <br>
/// do not change mutex held number
/// return if disinherit was successful
pub fn xTaskPriorityDisinherit(pxMutexHolder: Option<TaskHandle_t>) -> BaseType {
    let mut xReturn: BaseType = pdFALSE;
    match pxMutexHolder {
        Some(pxMutexHolder_) => {
            let pxMutexHolderTCB: &mut tskTaskControlBlock = &mut pxMutexHolder_.write();
            pxMutexHolderTCB.uxMutexesHeld -= 1;
            if pxMutexHolderTCB.uxBasePriority != pxMutexHolderTCB.uxPriority {
                if pxMutexHolderTCB.uxMutexesHeld == 0 {
                    ux_list_remove(Arc::downgrade(&pxMutexHolderTCB.xStateListItem));
                    pxMutexHolderTCB.uxPriority = pxMutexHolderTCB.uxBasePriority;
                    list_item_set_value(
                        &pxMutexHolderTCB.xEventListItem,
                        configMAX_PRIORITIES - pxMutexHolderTCB.uxPriority,
                    );
                    prvAddTaskToReadyList(pxMutexHolder_.clone());
                    xReturn = pdTRUE;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        None => {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    xReturn
}

/// increase current task's mutex count <br>
/// return handle of current task
pub fn pvTaskIncrementMutexHeldCount() -> Option<TaskHandle_t> {
    match &*CURRENT_TCB.write() {
        Some(x) => {
            get_current_tcb().unwrap().uxMutexesHeld += 1;
            Some(x.clone())
        }
        None => None,
    }
}

/// disinherit and recover original priority for mutex holder task after timeout<br>
/// disinherit priority only when no other mutex are held <br>
/// do not change mutex held number
pub fn vTaskPriorityDisinheritAfterTimeout(
    pxMutexHolder: Option<TaskHandle_t>,
    uxHighestPriorityWaitingTask: UBaseType,
) {
    let uxPriorityToUse: UBaseType;
    let uxPriorityUsedOnEntry: UBaseType;
    match pxMutexHolder {
        Some(pxMutexHolder_) => {
            let pxMutexHolderTCB: &mut tskTaskControlBlock = &mut pxMutexHolder_.write();
            uxPriorityToUse = max(
                pxMutexHolderTCB.uxBasePriority,
                uxHighestPriorityWaitingTask,
            );
            if pxMutexHolderTCB.uxPriority != uxPriorityToUse {
                if pxMutexHolderTCB.uxMutexesHeld == 1 {
                    uxPriorityUsedOnEntry = pxMutexHolderTCB.uxPriority;
                    pxMutexHolderTCB.uxPriority = uxPriorityToUse;
                    list_item_set_value(
                        &pxMutexHolderTCB.xEventListItem,
                        configMAX_PRIORITIES - uxPriorityToUse,
                    );

                    if list_is_contained_within(
                        &READY_TASK_LISTS[uxPriorityUsedOnEntry as usize],
                        &pxMutexHolderTCB.xStateListItem,
                    ) {
                        ux_list_remove(Arc::downgrade(&pxMutexHolderTCB.xStateListItem));
                        prvAddTaskToReadyList(pxMutexHolder_.clone());
                    }
                }
            }
        }
        None => {}
    }
}

///place current task on event list and delay it
pub fn vTaskPlaceOnEventList(pxEventList: &ListRealLink, xTicksToWait: TickType) {
    v_list_insert(
        pxEventList,
        get_current_tcb().unwrap().xEventListItem.clone(),
    );
    prvAddCurrentTaskToDelayedList(xTicksToWait, true);
}

/// remove target task from unordered event list
/// used in event groups
pub fn vTaskRemoveFromUnorderedEventList(pxEventListItem: &ListItemLink, xItemValue: TickType) {
    list_item_set_value(
        pxEventListItem,
        xItemValue | taskEVENT_LIST_ITEM_VALUE_IN_USE,
    );
    ux_list_remove(Arc::downgrade(pxEventListItem));
    let pxUnblockedTCB: TaskHandle_t =
        Weak::upgrade(&list_item_get_owner(&Arc::downgrade(pxEventListItem))).unwrap();
    ux_list_remove(Arc::downgrade(&pxUnblockedTCB.read().xStateListItem));
    prvAddTaskToReadyList(pxUnblockedTCB.clone());
    if pxUnblockedTCB.read().uxPriority > get_current_tcb().unwrap().uxPriority {
        unsafe {
            xYieldPending = true;
        }
    }
}

/// place target task on unordered event list
/// used in event groups
pub fn vTaskPlaceOnUnorderedEventList(
    pxEventList: &ListRealLink,
    xItemValue: TickType,
    xTicksToWait: TickType,
) {
    taskENTER_CRITICAL!();
    list_item_set_value(
        &get_current_tcb().unwrap().xEventListItem,
        xItemValue | taskEVENT_LIST_ITEM_VALUE_IN_USE,
    );
    v_list_insert_end(
        pxEventList,
        get_current_tcb().unwrap().xEventListItem.clone(),
    );
    prvAddCurrentTaskToDelayedList(xTicksToWait, true);
    taskEXIT_CRITICAL!();
}

/// reset event item value
/// return original item value
pub fn uxTaskResetEventItemValue() -> TickType {
    let uxReturn: TickType = list_item_get_value(&get_current_tcb().unwrap().xEventListItem);
    list_item_set_value(
        &get_current_tcb().unwrap().xEventListItem,
        configMAX_PRIORITIES - &get_current_tcb().unwrap().uxPriority,
    );
    uxReturn
}
