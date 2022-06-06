//! task control api

extern crate alloc;
use super::config::{configMAX_PRIORITIES, CONFIG_ISR_STACK_SIZE_WORDS, USER_STACK_SIZE};
use super::kernel::{IDLE_p, IDLE_STACK};
use crate::kernel::kernel::*;
use crate::kernel::linked_list::*;
use crate::kernel::projdefs::*;
use crate::portable::portable::*;
use crate::portable::portmacro::*;
use crate::riscv_virt::*;
#[cfg(feature = "configUSE_IDLE_HOOK")]
use crate::vApplicationIdleHook;
use crate::{
    mtCOVERAGE_TEST_MARKER, portDISABLE_INTERRUPTS, portENABLE_INTERRUPTS, portENTER_CRITICAL,
    portEXIT_CRITICAL, portYIELD, portYIELD_WITHIN_API,
};
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::{Arc, Weak};
use core::arch::asm;
use core::clone;
use core::cmp::max;
use core::ffi::c_void;
use core::ptr::NonNull;
use spin::RwLock;

pub type StackType_t = usize;
pub type StackType_t_link = usize;
pub type Param_link = usize;
pub type TCB_t_link = Arc<RwLock<TCB_t>>;
pub type UBaseType_t = usize;
pub type TaskFunction_t = *mut fn(*mut c_void);

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
        pxCode: u32,
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

/// Set pxStack of target tcb.
pub fn TCB_set_pxStack(tcb: &TCB_t_link, item: StackType_t_link) {
    //TODO: item owner
    tcb.write().pxStack = item;
}

pub type tskTCB = tskTaskControlBlock;
pub type TCB_t = tskTCB;
//TaskHandle_t=tskTaskControlBlock*
pub type TaskHandle_t = Arc<RwLock<tskTaskControlBlock>>;

/// Set priority of target task.
pub fn vTaskPrioritySet(pxTask: Option<&TaskHandle_t>, uxNewPriority: UBaseType) {
    vTaskEnterCritical();
    match pxTask {
        Some(x) => {
            ux_list_remove(Arc::downgrade(&x.read().xStateListItem));
            v_list_insert_end(
                &READY_TASK_LISTS[uxNewPriority as usize],
                &x.read().xStateListItem,
            );
            x.write().uxPriority = uxNewPriority;
            list_item_set_value(
                &x.write().xEventListItem,
                *configMAX_PRIORITIES - uxNewPriority,
            );
        }
        None => match get_current_tcb() {
            Some(x) => {
                ux_list_remove(Arc::downgrade(&(*x).xStateListItem));
                v_list_insert_end(&READY_TASK_LISTS[uxNewPriority as usize], &x.xStateListItem);
                x.uxPriority = uxNewPriority;
                list_item_set_value(&x.xEventListItem, *configMAX_PRIORITIES - uxNewPriority);
            }
            None => {}
        },
    }
    vTaskExitCritical();
}

/// Get priority of target task.
pub fn uxTaskPriorityGet(pxTask: Option<TaskHandle_t>) -> UBaseType {
    match get_current_tcb() {
        Some(x) => unsafe {
            return (*x).uxPriority;
        },
        None => {
            return 0;
        }
    }
}

/// Create task (static).
#[cfg(feature = "configSUPPORT_STATIC_ALLOCATION")]
pub fn xTaskCreateStatic(
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    puxStackBuffer: Option<StackType_t_link>,
    pxTaskBuffer: Option<&TCB_t_link>,
    uxPriority: UBaseType,
) -> Option<TaskHandle_t> {
    assert!(puxStackBuffer.is_some());
    assert!(pxTaskBuffer.is_some());

    let xReturn = Arc::new(RwLock::new(tskTaskControlBlock::default()));

    let pxNewTCB: &TCB_t_link = pxTaskBuffer.unwrap();
    TCB_set_pxStack(pxNewTCB, puxStackBuffer.unwrap());

    prvInitialiseNewTask(
        pxTaskCode,
        pcName,
        ulStackDepth,
        pvParameters,
        &xReturn,
        uxPriority,
        pxNewTCB,
    );

    prvAddNewTaskToReadyList(pxNewTCB);

    Some(xReturn)
}

/// Add new task to ready list.
pub fn prvAddNewTaskToReadyList(pxNewTCB: &TCB_t_link) {
    // taskENTER_CRITICAL!();
    {
        //TODO:
        prvAddTaskToReadyList(&pxNewTCB);
    }
    // taskEXIT_CRITICAL!();
}

/// Add target task to ready list.
pub fn prvAddTaskToReadyList(pxNewTCB: &TCB_t_link) {
    let uxPriority = pxNewTCB.read().uxPriority;

    taskRECORD_READY_PRIORITY(uxPriority);
    v_list_insert_end(
        &READY_TASK_LISTS[uxPriority as usize],
        &pxNewTCB.read().xStateListItem,
    );
}
/// Set max uxTopReadyPriority.<br>
/// Deprecated in current implement.
pub fn taskRECORD_READY_PRIORITY(uxPriority: UBaseType) {
    //TODO: set max uxTopReadyPriority
}

/// Initialise new task.
pub fn prvInitialiseNewTask<'a>(
    pxTaskCode: u32,
    pcName: &'a str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    pxCreatedTask: &'a TaskHandle_t,
    priority: UBaseType,
    pxNewTCB: &'a TCB_t_link,
) -> &'a TaskHandle_t {
    let mut pxTopOfStack: StackType_t_link = pxNewTCB.read().pxStack;
    pxTopOfStack = pxTopOfStack & (!(0x0007usize));

    let x: UBaseType = 0;
    //TODO: name length

    pxNewTCB.write().pcTaskName = pcName.to_string();
    pxNewTCB.write().uxPriority = priority;
    if cfg!(feature = "configUSE_MUTEXES") {
        pxNewTCB.write().uxBasePriority = priority;
        pxNewTCB.write().uxMutexesHeld = 0;
    }
    //TODO:auto init

    list_item_set_owner(&pxNewTCB.write().xStateListItem, Arc::downgrade(&pxNewTCB));
    list_item_set_owner(&pxNewTCB.write().xEventListItem, Arc::downgrade(&pxNewTCB));
    list_item_set_value(
        &pxNewTCB.write().xEventListItem,
        *configMAX_PRIORITIES - priority,
    );

    unsafe {
        pxNewTCB.write().pxTopOfStack =
            pxPortInitialiseStack(pxTopOfStack as *mut _, pxTaskCode, 0 as *mut _) as usize;
        pxNewTCB.write().uxCriticalNesting = 0;
    }

    //TODO: return
    *pxCreatedTask.write() = (*(pxNewTCB.write())).clone();
    pxNewTCB
}

/// Idle task function.
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

/// Start scheduler.
#[no_mangle]
pub extern "C" fn vTaskStartScheduler() {
    X_ISRSTACK_
        .write()
        .resize(unsafe { CONFIG_ISR_STACK_SIZE_WORDS }, 0);
    unsafe {
        XSCHEDULERRUNNING = pdTRUE;
    }
    if cfg!(feature = "configSUPPORT_STATIC_ALLOCATION") {
        let param: Param_link = 0;
        let stack2ptr: StackType_t_link = &*IDLE_STACK as *const [u32; USER_STACK_SIZE]
            as *const u32 as usize
            + USER_STACK_SIZE * 4
            - 4;
        xTaskCreateStatic(
            prvIdleTask as u32,
            "idle",
            USER_STACK_SIZE as u32,
            Some(param),
            Some(stack2ptr),
            Some(&IDLE_p),
            0,
        );
    }
    set_current_tcb(Some(Arc::downgrade(&IDLE_p)));
    if x_port_start_scheduler() != pdFALSE {
        panic!("error scheduler!!!!!!");
    }
}

/// Enter task critical area.
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
/// Exit task critical area.
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

/// Set current tcb to task with highest priority.
pub fn taskSELECT_HIGHEST_PRIORITY_TASK() {
    //TODO: uxTopReadyPriority全局变量设置和更新
    //TODO: 函数规范化
    let max_prio = taskSELECT_HIGHEST_PRIORITY();
    // let target:ListItemWeakLink=list_get_head_entry(&READY_TASK_LISTS[max_prio]);
    // let owner:ListItemOwnerWeakLink=list_item_get_owner(&target);
    let owner: ListItemOwnerWeakLink = list_get_owner_of_next_entry(&READY_TASK_LISTS[max_prio]);
    set_current_tcb(Some(owner));
    auto_set_currentTcb();
}

/// Find highest priority with valid task.
pub fn taskSELECT_HIGHEST_PRIORITY() -> usize {
    for i in 1..16 {
        let j = 16 - i;
        if !list_is_empty(&READY_TASK_LISTS[j]) {
            return j;
        }
    }
    return 0;
}

/// Yield in task.
pub fn taskYield() {
    portYIELD!();
}

/// Add current task to delayed list.
pub fn prvAddCurrentTaskToDelayedList(xTicksToWait: TickType, xCanBlockIndefinitely: bool) {
    //todo
    //vTaskEnterCritical();

    let xTimeToWake: TickType;
    let xConstTickCount: TickType;

    unsafe {
        xTimeToWake = xTicksToWait + xTickCount;
        xConstTickCount = xTickCount;
    }

    let temp = get_current_tcb().unwrap();
    let list_item = &temp.xStateListItem;

    list_item_set_value(&list_item, xTimeToWake);
    ux_list_remove(Arc::downgrade(&list_item));
    if xTimeToWake > xConstTickCount {
        v_list_insert(&DELAYED_TASK_LIST, &list_item);
        unsafe {
            if xTimeToWake < xNextTaskUnblockTime {
                xNextTaskUnblockTime = xTimeToWake;
            }
        }
    } else {
        v_list_insert(&OVERFLOW_DELAYED_TASK_LIST, &list_item);
    }
    //vTaskExitCritical();
}

#[no_mangle]
/// Delay Task xTicksToDelay Relativly.
/// Used in task function.
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
/// Delay task until pxPreviousWakeTime+pxPreviousWakeTime.
/// Used in task function.
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
        let xConstTickCount: TickType;
        unsafe {
            xConstTickCount = xTickCount;
        }

        let xTimeToWake: TickType = *pxPreviousWakeTime + xTimeIncrement;
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
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    let xAlreadyYielded: bool = vTaskResumeAll();
    if xAlreadyYielded == false {
        portYIELD_WITHIN_API!();
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}

/// In case of mtime overflow, swap delayed list and overflow list.
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

/// Increase tick and free delayed task.
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
                            v_list_insert_end(&READY_TASK_LISTS[prio as usize], &head);
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

/// Manually force tick increment outside ISR.
pub fn xPortSysTickHandler() {
    vTaskEnterCritical();
    xTaskIncrementTick();
    vTaskExitCritical();
}
#[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]

/// Create task (dynamic).
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
    pxTaskCode: u32,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,

    uxPriority: UBaseType,
    pxCreatedTask: Option<TaskHandle_t>,
) -> BaseType {
    let xReturn: BaseType = 0;
    let mut pxStack: StackType_t_link = 0;

    use core::mem;

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

    prvInitialiseNewTask(
        pxTaskCode,
        pcName,
        ulStackDepth,
        pvParameters,
        &pxCreatedTask.unwrap(),
        uxPriority,
        &pxNewTCB,
    );
    prvAddNewTaskToReadyList(&pxNewTCB);
    mem::forget(pxNewTCB);
    1
}

/// Enumerate states of task.
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

/// Suspend task until resumed.
/// Assert params is not Option::None.
#[cfg(feature = "INCLUDE_vTaskSuspend")]
pub fn vTaskSuspend(xTaskToSuspend_: Option<TaskHandle_t>) {
    /*
    默认传入有效handle or curtcb
     */
    let xTaskToSuspend = prvGetTCBFromHandle(xTaskToSuspend_.as_ref()).unwrap();
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
        v_list_insert_end(&SUSPENDED_TASK_LIST, &xTaskToSuspend.xStateListItem);
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
            portYIELD_WITHIN_API!();
        } else {
            if list_current_list_length(&SUSPENDED_TASK_LIST) != get_uxCurrentNumberOfTasks!() {
            } else {
                vTaskSwitchContext();
            }
        }
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}
pub static mut xSchedulerRunning: bool = false;

/// Resume target task.
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
                prvAddNewTaskToReadyList(&xTaskToResume);
                if (pxTCB.uxPriority >= get_current_tcb().unwrap().uxPriority) {
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

/// Return if target task is suspended.
pub fn prvTaskIsTaskSuspended(xTaskToResume: &TaskHandle_t) -> bool {
    //todo!
    let mut xReturn: bool = false;
    if list_is_contained_within(&*SUSPENDED_TASK_LIST, &xTaskToResume.read().xStateListItem) {
        if !list_is_contained_within(&*PENDING_READY_LIST, &xTaskToResume.read().xEventListItem) {
            xReturn = true;
        }
    }
    xReturn
}

/// Get tcb from handle, return current tcb if handle is None.
pub fn prvGetTCBFromHandle(
    handle: Option<&TaskHandle_t>,
) -> Option<&'static mut tskTaskControlBlock> {
    match handle {
        Some(x) => unsafe {
            let temp = &*(x.read()) as *const tskTaskControlBlock;
            Some(&mut *(temp as *mut tskTaskControlBlock))
        },
        None => get_current_tcb(),
    }
}

/// Delete target task.
pub fn vTaskDelete(xTaskToDelete: Option<&TaskHandle_t>) {
    taskENTER_CRITICAL!();
    let pxTCB = prvGetTCBFromHandle(xTaskToDelete);
    ux_list_remove(Arc::downgrade(&pxTCB.unwrap().xStateListItem));
    //todo：事件相关处理
    //todo：任务和tcb内存释放
    //todo：钩子函数
    taskEXIT_CRITICAL!();
    let need_yield = match xTaskToDelete {
        Some(x) => is_current_tcb(Arc::downgrade(x)),
        None => true,
    };
    if need_yield {
        portYIELD!();
    }
}

/// Reset NextTaskUnblockTime.
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
/// Suspend scheduler.
pub extern "C" fn vTaskSuspendAll() {
    unsafe {
        uxSchedulerSuspended += 1;
    }
}

#[no_mangle]
/// Resume scheduler.
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
                    prvAddNewTaskToReadyList(&pxTCB);

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

/// Remove first task from event list, and insert the task to ready list.
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
        ux_list_remove(Arc::downgrade(&pxUnblockedTCB.read().xStateListItem));
        prvAddTaskToReadyList(&pxUnblockedTCB);
        if cfg!(feature = "configUSE_TICKLESS_IDLE") {
            prvResetNextTaskUnblockTime();
        }
    } else {
        v_list_insert_end(&PENDING_READY_LIST, &pxUnblockedTCB.read().xEventListItem);
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

/// Set pxTimeOut to current time in ISR.
pub fn vTaskInternalSetTimeOutState(pxTimeOut: &mut TimeOut) {
    unsafe {
        pxTimeOut.xOverflowCount = xNumOfOverflows;
        pxTimeOut.xTimeOnEntering = xTickCount;
    }
}

/// Set pxTimeOut to current time in task.
pub fn vTaskSetTimeOutState(pxTimeOut: &mut TimeOut) {
    taskENTER_CRITICAL!();
    unsafe {
        pxTimeOut.xOverflowCount = xNumOfOverflows;
        pxTimeOut.xTimeOnEntering = xTickCount;
    }
    taskEXIT_CRITICAL!();
}

/// Return if timeout has been reached.
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

/// Inherit mutex holder task's priority to current task's priority. <br>
/// Return if the inherit was successful.
pub fn xTaskPriorityInherit(pxMutexHolder: Option<&TaskHandle_t>) -> BaseType {
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
                        *configMAX_PRIORITIES - pxMutexHolderTCB.uxPriority,
                    );
                }

                if list_is_contained_within(
                    &READY_TASK_LISTS[pxMutexHolderTCB.uxPriority as usize],
                    &pxMutexHolderTCB.xStateListItem,
                ) == true
                {
                    ux_list_remove(Arc::downgrade(&pxMutexHolderTCB.xStateListItem));
                    pxMutexHolderTCB.uxPriority = get_current_tcb().unwrap().uxPriority;
                    prvAddTaskToReadyList(&pxMutexHolder_);
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

/// Disinherit and recover original priority for mutex holder task <br>
/// Disinherit priority only when no other mutex are held <br>
/// Does not change mutex held number.<br>
/// Return if disinherit was successful.
pub fn xTaskPriorityDisinherit(pxMutexHolder: Option<&TaskHandle_t>) -> BaseType {
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
                        *configMAX_PRIORITIES - pxMutexHolderTCB.uxPriority,
                    );
                    prvAddTaskToReadyList(&pxMutexHolder_);
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

/// Increase current task's mutex count. <br>
/// Return handle of current task.
pub fn pvTaskIncrementMutexHeldCount() -> Option<TaskHandle_t> {
    match &*CURRENT_TCB.write() {
        Some(x) => {
            get_current_tcb().unwrap().uxMutexesHeld += 1;
            Some(x.clone())
        }
        None => None,
    }
}

/// Disinherit and recover original priority for mutex holder task after timeout. <br>
/// Disinherit priority only when no other mutex are held. <br>
/// Doed not change mutex held number.
pub fn vTaskPriorityDisinheritAfterTimeout(
    pxMutexHolder: Option<&TaskHandle_t>,
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
                        *configMAX_PRIORITIES - uxPriorityToUse,
                    );

                    if list_is_contained_within(
                        &READY_TASK_LISTS[uxPriorityUsedOnEntry as usize],
                        &pxMutexHolderTCB.xStateListItem,
                    ) {
                        ux_list_remove(Arc::downgrade(&pxMutexHolderTCB.xStateListItem));
                        prvAddTaskToReadyList(&pxMutexHolder_);
                    }
                }
            }
        }
        None => {}
    }
}

/// Place current task on event list and delay it.
pub fn vTaskPlaceOnEventList(pxEventList: &ListRealLink, xTicksToWait: TickType) {
    v_list_insert(pxEventList, &get_current_tcb().unwrap().xEventListItem);
    prvAddCurrentTaskToDelayedList(xTicksToWait, true);
}

/// Remove target task from unordered event list.<br>
/// Used in event group.
pub fn vTaskRemoveFromUnorderedEventList(pxEventListItem: &ListItemLink, xItemValue: TickType) {
    list_item_set_value(
        pxEventListItem,
        xItemValue | taskEVENT_LIST_ITEM_VALUE_IN_USE,
    );
    ux_list_remove(Arc::downgrade(pxEventListItem));
    let pxUnblockedTCB: TaskHandle_t =
        Weak::upgrade(&list_item_get_owner(&Arc::downgrade(pxEventListItem))).unwrap();
    ux_list_remove(Arc::downgrade(&pxUnblockedTCB.read().xStateListItem));
    prvAddTaskToReadyList(&pxUnblockedTCB);
    if pxUnblockedTCB.read().uxPriority > get_current_tcb().unwrap().uxPriority {
        unsafe {
            xYieldPending = true;
        }
    }
}

/// Place target task on unordered event list.<br>
/// Used in event group.
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
    v_list_insert_end(pxEventList, &get_current_tcb().unwrap().xEventListItem);
    prvAddCurrentTaskToDelayedList(xTicksToWait, true);
    taskEXIT_CRITICAL!();
}

/// Reset event item value.
/// Return original item value.
pub fn uxTaskResetEventItemValue() -> TickType {
    let uxReturn: TickType = list_item_get_value(&get_current_tcb().unwrap().xEventListItem);
    list_item_set_value(
        &get_current_tcb().unwrap().xEventListItem,
        *configMAX_PRIORITIES - &get_current_tcb().unwrap().uxPriority,
    );
    uxReturn
}

/// Get name of target task.
pub fn pcTaskGetName(xTaskToQuery: Option<&TaskHandle_t>) -> &str {
    let temp = prvGetTCBFromHandle(xTaskToQuery).unwrap();
    &temp.pcTaskName
}
