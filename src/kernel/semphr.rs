use crate::kernel::queue::*;
use crate::portable::portmacro::*;
use alloc::sync::Arc;
use spin::RwLock;

pub type SemaphoreHandle_t = QueueHandle_t;
pub static semSEMAPHORE_QUEUE_ITEM_LENGTH: UBaseType = 0;
pub static semGIVE_BLOCK_TIME: TickType = 0;

#[macro_export]
macro_rules! xSemaphoreCreateBinary {
    () => {
        QueueDefinition::xQueueGenericCreate(
            1,
            semSEMAPHORE_QUEUE_ITEM_LENGTH,
            queueQUEUE_TYPE_BINARY_SEMAPHORE,
        )
    };
}

/// Create counting Semaphore.
pub fn xSemaphoreCreateCounting(
    uxMaxCount: UBaseType,
    uxInitialCount: UBaseType,
) -> QueueDefinition {
    let mut handle = QueueDefinition::xQueueGenericCreate(
        uxMaxCount,
        semSEMAPHORE_QUEUE_ITEM_LENGTH,
        queueQUEUE_TYPE_COUNTING_SEMAPHORE,
    );

    //todo:asserts
    handle.uxMessagesWaiting = uxInitialCount;

    handle
}

#[macro_export]
macro_rules! vSemaphoreDelete {
    ($xSemaphore:expr) => {
        vQueueDelete($xSemaphore);
    };
}

#[macro_export]
macro_rules! xSemaphoreGive {
    ($xSemaphore:expr) => {
        xQueueGenericSend($xSemaphore, 0, semGIVE_BLOCK_TIME, queueSEND_TO_BACK);
    };
}

#[macro_export]
macro_rules! xSemaphoreTake {
    ($xSemaphore:expr,$xBlockTime:expr) => {
        xQueueReceive($xSemaphore, 0, $xBlockTime)
    };
}

/// Create mutex.
pub fn xQueueCreateMutex(ucQueueType: u8) -> QueueHandle_t {
    let queue = Arc::new(RwLock::new(QueueDefinition::xQueueGenericCreate(
        1,
        0,
        ucQueueType,
    )));
    prvInitialiseMutex(&queue);
    queue
}

/// Initialise mutex.
pub fn prvInitialiseMutex(pxNewQueue: &QueueHandle_t) {
    pxNewQueue.write().xMutexHolder = None;
    pxNewQueue.write().uxRecursiveCallCount = 0;
    xQueueGenericSend(pxNewQueue, 0, 0, queueSEND_TO_BACK);
}
