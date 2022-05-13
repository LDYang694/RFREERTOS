use crate::kernel::allocator::*;
use crate::kernel::config::*;
use crate::kernel::kernel::*;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::projdefs::pdFALSE;
use crate::kernel::projdefs::pdTRUE;
use crate::kernel::queue::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::tasks::*;
use crate::portENTER_CRITICAL;
use crate::portEXIT_CRITICAL;
use crate::prvLockQueue;
use crate::taskENTER_CRITICAL;
use crate::taskEXIT_CRITICAL;
use alloc::sync::{Arc, Weak};
use alloc::{fmt::format, format};
use core::ffi::c_void;
use core::mem::size_of;
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;

pub fn test_macro_xQueueCreate_oneItem_oneLength() {
    let xQueue: QueueHandle_t = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(
        1,
        size_of::<BaseType>() as UBaseType,
    )));
    assert!(xQueue.read().uxMessagesWaiting == 0);
    let mut sendval: BaseType = 0xff;
    assert!(xQueueSend(xQueue.clone(), &mut sendval as *mut BaseType as usize, 0) == pdTRUE);

    assert!(xQueue.read().uxMessagesWaiting == 1);
    let mut testval: BaseType = 0;
    assert!(xQueueReceive(xQueue.clone(), &mut testval as *mut BaseType as usize, 0) == pdTRUE);
    assert!(testval == sendval);
    assert!(xQueue.read().uxMessagesWaiting == 0);
}

pub fn test_xQueuePeek_fail_empty() {
    let xQueue: QueueHandle_t = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(
        1,
        size_of::<BaseType>() as UBaseType,
    )));

    let mut checkval: BaseType = 0xff;
    assert!(xQueuePeek(xQueue, &mut checkval as *mut BaseType as usize, 0) == pdFALSE);
}

pub fn test_xQueuePeek_zeroItemSize_full() {
    let xQueue: QueueHandle_t = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(1, 0)));
    xQueueSend(xQueue.clone(), 0, 0);
    assert!(xQueuePeek(xQueue.clone(), 0, 0) == pdTRUE);
    assert!(xQueue.read().uxMessagesWaiting == 1);
}

pub static mut testQueue: Option<QueueHandle_t> = None;

pub fn high_priority_send_task() {
    loop {
        vSendString("in high priority task");
        unsafe {
            xQueueSend(testQueue.clone().unwrap(), 0, 0);
        }
        //vTaskDelete(None);
        vTaskSuspend(testTask.clone());
        //vTaskDelay(1000000);
    }
}

pub fn test_xQueuePeek_xQueueReceive_waiting_higher_priority() {
    unsafe {
        testQueue = Some(Arc::new(RwLock::new(QueueDefinition::xQueueCreate(1, 0))));
    }
    let param1: Param_link = 0;
    unsafe {
        vTaskPrioritySet(testTask.clone(), 4);
    }
    assert!(unsafe { testQueue.clone().unwrap().read().uxMessagesWaiting } == 1);
    let checkval: BaseType = 0;
    assert!(unsafe {
        xQueuePeek(
            testQueue.clone().unwrap(),
            &checkval as *const BaseType as usize,
            0,
        ) == pdTRUE
    });
}

pub fn test_xQueueReceive_fail_empty() {
    let xQueue: QueueHandle_t = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(
        1,
        size_of::<BaseType>() as UBaseType,
    )));

    let mut checkval: BaseType = 0xff;
    assert!(xQueueReceive(xQueue, &mut checkval as *mut BaseType as usize, 0) == pdFALSE);
}

pub fn test_xQueueReceiveFromISR_locked() {
    let xQueue: QueueHandle_t = Arc::new(RwLock::new(QueueDefinition::xQueueCreate(
        1,
        size_of::<BaseType>() as UBaseType,
    )));
    let testval: BaseType = 0xab;
    xQueueSend(xQueue.clone(), &testval as *const BaseType as usize, 0);
    prvLockQueue!(xQueue);
    let checkval: BaseType = 0;
    assert!(xQueueReceive(xQueue.clone(), &checkval as *const BaseType as usize, 0) == pdTRUE);
    let s = format!("{}", xQueue.read().cRxLock);
    vSendString(&s);
    //assert!(xQueue.read().cRxLock == queueLOCKED_UNMODIFIED + 1);
    //assert!(xQueue.read().cTxLock == queueLOCKED_UNMODIFIED);
}

pub fn test_func_queue(t: *mut c_void) {
    vSendString("testing queue");
    test_macro_xQueueCreate_oneItem_oneLength();
    test_xQueuePeek_fail_empty();
    test_xQueuePeek_zeroItemSize_full();
    test_xQueuePeek_xQueueReceive_waiting_higher_priority();
    test_xQueueReceive_fail_empty();
    test_xQueueReceiveFromISR_locked();
    vSendString("test passed!");
    loop {}
}

lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
    pub static ref testTask: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}

pub fn test_main_queue() {
    let param1: Param_link = 0;
    unsafe {
        xTaskCreate(
            test_func_queue as u32,
            "test_func_queue",
            USER_STACK_SIZE as u32,
            Some(param1),
            3,
            Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
        );
        xTaskCreate(
            high_priority_send_task as u32,
            "high_priority_send_task",
            USER_STACK_SIZE as u32,
            Some(param1),
            2,
            Some(Arc::clone(&(testTask.as_ref().unwrap()))),
        );
    }
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
