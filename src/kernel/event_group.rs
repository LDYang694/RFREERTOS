use super::portmacro::*;
use super::projdefs::pdFALSE;
use crate::kernel::linked_list::*;
use crate::kernel::tasks::*;
use alloc::sync::{Arc, Weak};
use alloc::boxed::Box;
use core::default;
use core::{alloc::Layout, mem};
use crate::*;
use spin::RwLock;

pub type EventBits=TickType;
pub type EventGroupHandle=Arc<RwLock<EventGroupDefinition>>;

pub const eventUNBLOCKED_DUE_TO_BIT_SET:TickType=02000000;

#[derive(Clone)]
pub struct EventGroupDefinition{
    uxEventBits:EventBits,
    pub xTasksWaitingForBits:ListRealLink,
    uxEventGroupNumber:UBaseType,
    ucStaticallyAllocated:u8
}

impl Default for EventGroupDefinition{
    fn default() -> Self {
        EventGroupDefinition { 
            uxEventBits: 0, 
            xTasksWaitingForBits:Default::default(), 
            uxEventGroupNumber: 0, 
            ucStaticallyAllocated: pdFALSE as u8
        }
    }
}

impl EventGroupDefinition {
    pub fn xEventGroupCreate()->Self{
        let mut pxEventBits: EventGroupDefinition=Default::default();


        pxEventBits.uxEventBits=0;
        //pxEventBits.xTasksWaitingForBits=Arc::new(RwLock::new(xTasksWaitingForBits));
        
        pxEventBits
    }
    
    
}

pub fn vEventGroupDelete(xEventGroup:EventGroupHandle){
    
    //vTaskSuspendAll();
    {
        while !list_is_empty(&xEventGroup.read().xTasksWaitingForBits){
            let head=list_get_head_entry(&xEventGroup.read().xTasksWaitingForBits);
            vTaskRemoveFromUnorderedEventList(&Weak::upgrade(&head).unwrap(),eventUNBLOCKED_DUE_TO_BIT_SET);
        }
    }
    //vTaskResumeAll();
}