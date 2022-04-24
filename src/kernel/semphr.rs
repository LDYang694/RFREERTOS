use crate::kernel::queue::*;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;

pub type SemaphoreHandle_t=QueueHandle_t;
pub static semSEMAPHORE_QUEUE_ITEM_LENGTH:UBaseType = 0;
pub static semGIVE_BLOCK_TIME:TickType = 0;

#[macro_export]
macro_rules!  xSemaphoreCreateBinary{
    () => {
        xQueueGenericCreate(1, semSEMAPHORE_QUEUE_ITEM_LENGTH, queueQUEUE_TYPE_BINARY_SEMAPHORE);
    };
}

pub fn xSemaphoreCreateCounting(uxMaxCount:UBaseType, uxInitialCount:UBaseType)->SemaphoreHandle_t{
    let handle=
        xQueueGenericCreate(uxMaxCount, semSEMAPHORE_QUEUE_ITEM_LENGTH, queueQUEUE_TYPE_COUNTING_SEMAPHORE);
    
    //todo:asserts
    handle.write().uxMessagesWaiting=uxInitialCount;

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
        xQueueGenericSend(&mut $xSemaphore.write(),0,semGIVE_BLOCK_TIME,queueSEND_TO_BACK);
    };
}

#[macro_export]
macro_rules! xSemaphoreTake {
    ($xSemaphore:expr,$xBlockTime:expr) => {
        xQueueReceive($xSemaphore,0,$xBlockTime)
    };
}