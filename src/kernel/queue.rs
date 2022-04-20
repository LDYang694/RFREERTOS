// typedef struct QueueDefinition /* The old naming convention is used to prevent breaking kernel aware debuggers. */
// {
//     int8_t * pcHead;           /*< Points to the beginning of the queue storage area. */
//     int8_t * pcWriteTo;        /*< Points to the free next place in the storage area. */
//     union
//     {
//         QueuePointers_t xQueue;     /*< Data required exclusively when this structure is used as a queue. */
//         SemaphoreData_t xSemaphore; /*< Data required exclusively when this structure is used as a semaphore. */
//     } u;

//     List_t xTasksWaitingToSend;             /*< List of tasks that are blocked waiting to post onto this queue.  Stored in priority order. */
//     List_t xTasksWaitingToReceive;          /*< List of tasks that are blocked waiting to read from this queue.  Stored in priority order. */
//     volatile UBaseType_t uxMessagesWaiting; /*< The number of items currently in the queue. */
//     UBaseType_t uxLength;                   /*< The length of the queue defined as the number of items it will hold, not the number of bytes. */
//     UBaseType_t uxItemSize;                 /*< The size of each items that the queue will hold. */
//     volatile int8_t cRxLock;                /*< Stores the number of items received from the queue (removed from the queue) while the queue was locked.  Set to queueUNLOCKED when the queue is not locked. */
//     volatile int8_t cTxLock;                /*< Stores the number of items transmitted to the queue (added to the queue) while the queue was locked.  Set to queueUNLOCKED when the queue is not locked. */
//     #if ( ( configSUPPORT_STATIC_ALLOCATION == 1 ) && ( configSUPPORT_DYNAMIC_ALLOCATION == 1 ) )
//         uint8_t ucStaticallyAllocated; /*< Set to pdTRUE if the memory used by the queue was statically allocated to ensure no attempt is made to free the memory. */
//     #endif

//     #if ( configUSE_QUEUE_SETS == 1 )
//         struct QueueDefinition * pxQueueSetContainer;
//     #endif

//     #if ( configUSE_TRACE_FACILITY == 1 )
//         UBaseType_t uxQueueNumber;
//         uint8_t ucQueueType;
//     #endif
// } xQUEUE;

// /* The old xQUEUE name is maintained above then typedefed to the new Queue_t
//  * name below to enable the use of older kernel aware debuggers. */
// typedef xQUEUE Queue_t;
extern crate alloc;
use super::{
    linked_list::ListRealLink,
    portmacro::{BaseType, UBaseType},
};
use crate::{
    kernel::{
        linked_list::*,
        tasks::{vTaskEnterCritical, vTaskExitCritical},
    },
    pdFALSE, taskENTER_CRITICAL, taskEXIT_CRITICAL,
};
use alloc::boxed::Box;
use alloc::sync::{Arc, Weak};
use core::{alloc::Layout, mem, intrinsics::size_of};
use spin::RwLock;
pub type QueueHandle_t = Arc<RwLock<QueueDefinition>>;
pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub type xQUEUE = QueueDefinition;
pub type Queue_t = xQUEUE;
pub struct QueueDefinition {
    pcHead: usize,
    pcTail: usize,
    pcWriteTo: usize,
    pcReadFrom: usize,
    xTasksWaitingToSend: ListRealLink,
    xTasksWaitingToReceive: ListRealLink,
    uxMessagesWaiting: UBaseType,
    uxLength: UBaseType,
    uxItemSize: UBaseType,
}
//TODO: xqueue default
pub fn xQueueCreate(uxQueueLength: UBaseType, uxItemSize: UBaseType) -> QueueHandle_t {
    xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
}
pub fn xQueueGenericCreate(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
    ucQueueType: u8,
) -> QueueHandle_t {
    assert!(uxQueueLength > 0);
    let mut xQueueSizeInBytes: isize;
    if uxItemSize == 0 {
        xQueueSizeInBytes = 0;
    } else {
        xQueueSizeInBytes = (uxQueueLength * uxItemSize) as isize;
    }
    let alloc_size = mem::size_of::<Queue_t>() + xQueueSizeInBytes as usize;
    let layout = Layout::from_size_align(alloc_size as usize, 4)
        .ok()
        .unwrap();
    let pxNewQueue_ptr: *mut u8;
    unsafe {
        pxNewQueue_ptr = alloc::alloc::alloc(layout);
    }
    //TODO:
    // if pxNewQueue_ptr!=NULL
    let pucQueueStorage: usize = pxNewQueue_ptr as usize + mem::size_of::<Queue_t>();
    // #if( configSUPPORT_STATIC_ALLOCATION == 1 )
    // 29 {
    // 30
    // 31 pxNewQueue->ucStaticallyAllocated = pdFALSE;
    // 32 }
    // 33 #endif
    prvInitialiseNewQueue(
        uxQueueLength,
        uxItemSize,
        pucQueueStorage,
        ucQueueType,
        pxNewQueue_ptr as usize,
    );
    let pxNewQueue = unsafe { Box::from_raw((pxNewQueue_ptr as *mut Queue_t)) };

    // unsafe {
    //     pxNewQueue = &*(pxNewQueue_ptr as *mut Queue_t );
    // }
    Arc::new(RwLock::new(Box::<QueueDefinition>::into_inner(pxNewQueue)))
}
pub fn prvInitialiseNewQueue(
    uxQueueLength: UBaseType,
    uxItemSize: UBaseType,
    pucQueueStorage: usize,
    ucQueueType: u8,
    pxNewQueue: usize,
) {
    let pxNewQueueBox = unsafe { Box::from_raw((pxNewQueue as *mut Queue_t)) };
    let mut pxNewQueue_ = Box::<QueueDefinition>::into_inner(pxNewQueueBox);
    if (uxItemSize == 0) {
        pxNewQueue_.pcHead = pxNewQueue;
    } else {
        pxNewQueue_.pcHead = pucQueueStorage;
    }
    pxNewQueue_.uxLength = uxQueueLength;
    pxNewQueue_.uxItemSize = uxItemSize;
    xQueueGenericReset(&mut pxNewQueue_, 1);
}

pub fn xQueueGenericReset(xQueue: &mut Queue_t, xNewQueue: BaseType) -> BaseType {
    // taskENTER_CRITICAL!();
    vTaskEnterCritical();
    {
        xQueue.pcTail = xQueue.pcHead + (xQueue.uxLength * xQueue.uxItemSize) as usize;
        xQueue.uxMessagesWaiting = 0;
        xQueue.pcWriteTo = xQueue.pcHead;
        //TODO: union
        xQueue.pcReadFrom = xQueue.pcHead + ((xQueue.uxLength - 1) * xQueue.uxItemSize) as usize;
        //TODO:lock
        if (xNewQueue == 0) {
            //TODO:
        } else {
            xQueue.xTasksWaitingToSend = Arc::new(RwLock::new(XList::default()));
            xQueue.xTasksWaitingToReceive = Arc::new(RwLock::new(XList::default()));
            mem::forget(&xQueue.xTasksWaitingToSend);
            mem::forget(&xQueue.xTasksWaitingToReceive);
        }
    }
    // taskEXIT_CRITICAL!();
    vTaskExitCritical();
    1
}

pub fn vQueueDelete(xQueue: QueueHandle_t) {
    let pxQueue = *xQueue.write();
    let alloc_size:usize=mem::size_of::<xQUEUE>()+(xQueue.read().uxLength*xQueue.read().uxItemSize) as usize;
    let layout = Layout::from_size_align(alloc_size as usize, 4)
        .ok()
        .unwrap();
    unsafe {
        alloc::alloc::dealloc(&pxQueue as *const Queue_t as *mut u8 , layout);
    }
}
