use crate::config::*;
use crate::linked_list::*;
use crate::alloc::string::ToString;
use crate::portable::*;
use spin::RwLock;
pub type StackType_t_link = usize;
pub type Param_link = usize;
pub type TCB_t_link = Arc<RwLock<TCB_t>>;
pub type TaskFunction_t = dyn Fn(usize);
// use std::cell::RefCell;
use crate::alloc::sync::{Arc, Weak};
use alloc::string::String;
use crate::riscv_virt::*;

#[no_mangle]
extern "C"{
    pub fn pxPortInitialiseStack()->StackType_t_link;
}

#[derive(Debug)]
pub struct tskTaskControlBlock {
    pxTopOfStack: StackType_t_link,
    pxStack: StackType_t_link,
    pcTaskName: String,
    pub xStateListItem: ListItemLink,
}
impl Default for tskTaskControlBlock {
    fn default() -> Self {
        tskTaskControlBlock {
            pxStack: 0,
            pxTopOfStack: 0,
            pcTaskName: String::new(),
            xStateListItem: Default::default(),
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
pub fn xTaskCreateStatic(
    pxTaskCode: &TaskFunction_t,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    puxStackBuffer: Option<StackType_t_link>,
    pxTaskBuffer: Option<TCB_t_link>,
) -> Option<TaskHandle_t> {
    let xReturn = Arc::new(RwLock::new(tskTaskControlBlock::default()));
    // let xxReturn=
    // if (pxTaskBuffer!=None && puxStackBuffer!=None){

    //     None
    // }else{
    //     None
    // }
    vSendString("xTaskCreateStatic 1111");
    //TODO:assert if =true
    let pxNewTCB: TCB_t_link = pxTaskBuffer.unwrap().clone();
    TCB_set_pxStack(&pxNewTCB, puxStackBuffer.unwrap());
    vSendString("xTaskCreateStatic 2222"); 
    let xReturn = prvInitialiseNewTask(
        pxTaskCode,
        pcName,
        ulStackDepth,
        pvParameters,
        &xReturn,
        pxNewTCB,
    );
    vSendString("xTaskCreateStatic 3333"); 
    Some(xReturn)
}
pub fn prvInitialiseNewTask(
    pxTaskCode: &TaskFunction_t,
    pcName: &str,
    ulStackDepth: u32,
    pvParameters: Option<Param_link>,
    pxCreatedTask: &TaskHandle_t,
    pxNewTCB: TCB_t_link,
) -> TaskHandle_t {
    let mut pxTopOfStack: StackType_t_link =
        pxNewTCB.read().pxStack + (ulStackDepth as usize - 1);
    pxTopOfStack = pxTopOfStack & (!(0x0007usize));
    let mut x: UBaseType = 0;
    //TODO: name length
    vSendString("prvInitialiseNewTask 1111");
    pxNewTCB.write().pcTaskName = pcName.to_string();
    //TODO:auto init
    vSendString("prvInitialiseNewTask 2222");
    list_item_set_owner(
        &pxNewTCB.write().xStateListItem,
        Arc::downgrade(&pxNewTCB),
    );
    vSendString("prvInitialiseNewTask 33333");
    //TODO: connect
    unsafe{
        pxNewTCB.write().pxTopOfStack = pxPortInitialiseStack();
    }
    vSendString("prvInitialiseNewTask 4444");
    //TODO: return
    pxNewTCB
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        println!("test world!");
    }
}

// pub fn x_task_create_static() {}
fn prvInitialiseTaskLists() {
    //initial in list impl
}