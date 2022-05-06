use crate::kernel::queue::*;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;

pub type SemaphoreHandle_t=QueueHandle_t;
pub static semSEMAPHORE_QUEUE_ITEM_LENGTH:UBaseType = 0;
pub static semGIVE_BLOCK_TIME:TickType = 0;

#[macro_export]
macro_rules!  xSemaphoreCreateBinary{
    () => {
        QueueDefinition::xQueueGenericCreate(1, semSEMAPHORE_QUEUE_ITEM_LENGTH, queueQUEUE_TYPE_BINARY_SEMAPHORE);
    };
}

pub fn xSemaphoreCreateCounting(uxMaxCount:UBaseType, uxInitialCount:UBaseType)->QueueDefinition{
    let mut handle=
    QueueDefinition::xQueueGenericCreate(uxMaxCount, semSEMAPHORE_QUEUE_ITEM_LENGTH, queueQUEUE_TYPE_COUNTING_SEMAPHORE);
    
    //todo:asserts
    handle.uxMessagesWaiting=uxInitialCount;

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
        xQueueGenericSend($xSemaphore,0,semGIVE_BLOCK_TIME,queueSEND_TO_BACK);
    };
}

#[macro_export]
macro_rules! xSemaphoreTake {
    ($xSemaphore:expr,$xBlockTime:expr) => {
        xQueueReceive($xSemaphore,0,$xBlockTime)
    };
}

pub fn  xQueueCreateMutex(ucQueueType:u8)->QueueDefinition{
    let mut queue=QueueDefinition::xQueueGenericCreate(1,0,ucQueueType);
    prvInitialiseMutex(&mut queue);
    queue
}

pub fn prvInitialiseMutex(pxNewQueue:&mut QueueDefinition){
    pxNewQueue.xMutexHolder=None;
    pxNewQueue.uxRecursiveCallCount=0;
    xQueueGenericSend(pxNewQueue,0,0,queueSEND_TO_BACK);
}

