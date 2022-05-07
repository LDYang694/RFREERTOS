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



pub const eventCLEAR_EVENTS_ON_EXIT_BIT:TickType=0x01000000;
pub const eventUNBLOCKED_DUE_TO_BIT_SET:TickType=0x02000000;
pub const eventWAIT_FOR_ALL_BITS:TickType=0x04000000;
pub const eventEVENT_BITS_CONTROL_BYTES:TickType=0xff000000;

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
    
    vTaskSuspendAll();
    {
        while !list_is_empty(&xEventGroup.read().xTasksWaitingForBits){
            let head=list_get_head_entry(&xEventGroup.read().xTasksWaitingForBits);
            vTaskRemoveFromUnorderedEventList(&Weak::upgrade(&head).unwrap(),eventUNBLOCKED_DUE_TO_BIT_SET);
        }
        //todo:free
    }
    vTaskResumeAll();
}

pub fn xEventGroupSetBits(xEventGroup:&mut EventGroupDefinition,uxBitsToSet:EventBits)->EventBits{
    //todo:use handle
    vTaskSuspendAll();
    {
        xEventGroup.uxEventBits|=uxBitsToSet;
        let mut uxBitsToClear:TickType=0;
        let mut pxListItem:ListItemWeakLink=list_get_head_entry(&xEventGroup.xTasksWaitingForBits );
        let pxListEnd=list_get_end_marker(&xEventGroup.xTasksWaitingForBits);
        while !pxListItem.ptr_eq(&pxListEnd) {
            vSendString("matching");
            let mut xMatchFound:BaseType = pdFALSE;
            let mut uxBitsWaitedFor=list_item_get_value(&Weak::upgrade(&pxListItem).unwrap());
            let uxControlBits=uxBitsWaitedFor&eventEVENT_BITS_CONTROL_BYTES;
            uxBitsWaitedFor&= !eventEVENT_BITS_CONTROL_BYTES;
            if uxControlBits&eventWAIT_FOR_ALL_BITS==0{
                if uxBitsWaitedFor&xEventGroup.uxEventBits != 0{
                    xMatchFound=pdTRUE;
                }
            }
            else{
                if uxBitsWaitedFor&xEventGroup.uxEventBits == uxBitsWaitedFor{
                    xMatchFound=pdTRUE;
                }
            }
            if xMatchFound!=pdFALSE{
                vSendString("match found");
                if uxControlBits&eventCLEAR_EVENTS_ON_EXIT_BIT!=0{
                    uxBitsToClear|=uxBitsWaitedFor;
                }
                vTaskRemoveFromUnorderedEventList(&Weak::upgrade(&pxListItem).unwrap(), 
                    xEventGroup.uxEventBits|eventUNBLOCKED_DUE_TO_BIT_SET);
            }
            pxListItem=list_item_get_next(&pxListItem);
            
        }
        xEventGroup.uxEventBits&=!uxBitsToClear;

    }
    vTaskResumeAll();
    xEventGroup.uxEventBits
}

pub fn xEventGroupWaitBits(xEventGroup:&mut EventGroupDefinition,uxBitsToWaitFor:EventBits,
    xClearOnExit:BaseType,xWaitForAllBits:BaseType,mut xTicksToWait:TickType)->EventBits{
    //todo:use handle
    let mut uxReturn:EventBits;
    let mut xTimeoutOccurred:BaseType=pdFALSE;
    let uxCurrentEventBits=xEventGroup.uxEventBits;
    vTaskSuspendAll();
    tf1();
    {
        let xWaitConditionMet=prvTestWaitCondition(uxCurrentEventBits,uxBitsToWaitFor,xWaitForAllBits);
        if xWaitConditionMet!=pdFALSE{
            uxReturn=uxCurrentEventBits;
            xTicksToWait=0;
            if xClearOnExit!=pdFALSE{
                xEventGroup.uxEventBits&= !uxBitsToWaitFor;
            }
        }
        else if xTicksToWait==0{
            xTimeoutOccurred=pdTRUE;
            uxReturn=uxCurrentEventBits;
        }
        else{
            
            let mut uxControlBits:TickType=0;
            if xClearOnExit!=pdFALSE{
                uxControlBits|=eventCLEAR_EVENTS_ON_EXIT_BIT;
            }
            if xWaitForAllBits!=pdFALSE{
                uxControlBits|=eventWAIT_FOR_ALL_BITS;
            }
            tf2();
            let temp=&xEventGroup.xTasksWaitingForBits;
            tf3();
            vTaskPlaceOnUnorderedEventList(temp, 
                uxBitsToWaitFor|uxControlBits, xTicksToWait);
            
            uxReturn=0;
        }
    }
    tf4();
    vSendString("resuming");
    let xAlreadyYielded=vTaskResumeAll();
    tf5();
    if xTicksToWait!=0{
        if xAlreadyYielded==false{
            portYIELD_WITHIN_API!();
        }
        else{
            mtCOVERAGE_TEST_MARKER!();
        }
        uxReturn = uxTaskResetEventItemValue();
        if uxReturn&eventUNBLOCKED_DUE_TO_BIT_SET==0{
            taskENTER_CRITICAL!();
            {
                uxReturn=uxCurrentEventBits;
                if prvTestWaitCondition(uxCurrentEventBits,uxBitsToWaitFor,xWaitForAllBits)!=pdFALSE{
                    if xClearOnExit!=pdFALSE{
                        xEventGroup.uxEventBits&= !uxBitsToWaitFor;
                    }
                }
                xTimeoutOccurred=pdTRUE;
            }
            taskEXIT_CRITICAL!();
        }
        uxReturn&=!eventEVENT_BITS_CONTROL_BYTES;
    }
    tf6();
    uxReturn
}

pub fn prvTestWaitCondition(uxCurrentEventBits:EventBits,uxBitsToWaitFor:EventBits,xWaitForAllBits:BaseType)->BaseType{
    if xWaitForAllBits==pdFALSE{
        if uxCurrentEventBits&uxBitsToWaitFor!=0{
            return pdTRUE;
        }
    }
    else{
        if uxCurrentEventBits&uxBitsToWaitFor==uxBitsToWaitFor{
            return pdTRUE;
        }
    }
    pdFALSE
}