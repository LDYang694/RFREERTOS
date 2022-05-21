use crate::kernel::config::*;
use crate::kernel::linked_list::*;
use crate::kernel::portable::*;
use crate::kernel::portmacro::*;
use crate::kernel::riscv_virt::*;
use crate::kernel::tasks::*;
use alloc::sync::{Arc, Weak};
use core::ffi::c_void;
use lazy_static::{__Deref, lazy_static};
use spin::RwLock;

pub fn validate_empty_list(pxList: ListRealLink) {
    assert!(list_current_list_length(&pxList) == 0);
    assert!(list_get_pxindex(&Arc::downgrade(&pxList)).ptr_eq(&list_get_end_marker(&pxList)));
    assert!(list_item_get_next(&list_get_end_marker(&pxList)).ptr_eq(&list_get_end_marker(&pxList)));
    assert!(list_item_get_pre(&list_get_end_marker(&pxList)).ptr_eq(&list_get_end_marker(&pxList)));
}

pub fn test_vListInsertEnd_Success_1_item() {
    let xList: ListT = Default::default();
    let pxList: ListRealLink = Arc::new(RwLock::new(xList));
    let pxNewListItem: ListItemLink = Default::default();
    v_list_insert_end(&pxList, pxNewListItem.clone());
    assert!(list_current_list_length(&pxList) == 1);
    assert!(list_get_pxindex(&Arc::downgrade(&pxList)).ptr_eq(&list_get_end_marker(&pxList)));
    assert!(list_item_get_next(&Arc::downgrade(&pxNewListItem.clone()))
        .ptr_eq(&list_get_end_marker(&pxList)));
    assert!(list_item_get_pre(&Arc::downgrade(&pxNewListItem.clone()))
        .ptr_eq(&list_get_end_marker(&pxList)));
    assert!(
        list_item_get_container(&Arc::downgrade(&pxNewListItem.clone()))
            .ptr_eq(&Arc::downgrade(&pxList))
    );
}

pub fn test_vListInsert_success_1_item() {
    let xList: ListT = Default::default();
    let pxList: ListRealLink = Arc::new(RwLock::new(xList));
    let pxNewListItem: ListItemLink = Default::default();
    v_list_insert(&pxList, pxNewListItem.clone());
    assert!(list_current_list_length(&pxList) == 1);
    assert!(list_get_pxindex(&Arc::downgrade(&pxList)).ptr_eq(&list_get_end_marker(&pxList)));
    assert!(list_item_get_next(&Arc::downgrade(&pxNewListItem.clone()))
        .ptr_eq(&list_get_end_marker(&pxList)));
    assert!(list_item_get_pre(&Arc::downgrade(&pxNewListItem.clone()))
        .ptr_eq(&list_get_end_marker(&pxList)));
    assert!(
        list_item_get_container(&Arc::downgrade(&pxNewListItem.clone()))
            .ptr_eq(&Arc::downgrade(&pxList))
    );
}

pub fn test_uxListRemove_success() {
    let xList: ListT = Default::default();
    let pxList: ListRealLink = Arc::new(RwLock::new(xList));
    let pxNewListItem: ListItemLink = Default::default();
    v_list_insert(&pxList, pxNewListItem.clone());
    ux_list_remove(Arc::downgrade(&pxNewListItem.clone()));
    validate_empty_list(pxList);
}

pub fn test_macro_listSET_GET_LIST_ITEM_VALUE() {
    let pxNewListItem: ListItemLink = Default::default();
    let initial_value: TickType = 10;
    list_item_set_value(&pxNewListItem, initial_value);
    assert!(list_item_get_value(&pxNewListItem) == initial_value);
}

pub fn test_macros_list_SET_GET_LIST_ITEM_OWNER() {
    let owner: TaskHandle_t = Default::default();
    let pxNewListItem: ListItemLink = Default::default();
    list_item_set_owner(&pxNewListItem, Arc::downgrade(&owner));
    assert!(list_item_get_owner(&Arc::downgrade(&pxNewListItem)).ptr_eq(&Arc::downgrade(&owner)));
}

pub fn test_macro_listGET_NEXT() {
    let pxList: ListRealLink = Default::default();
    let pxNewListItems: [ListItemLink; 2] = Default::default();
    list_item_set_value(&pxNewListItems[0], 0);
    list_item_set_value(&pxNewListItems[1], 1);
    v_list_insert(&pxList, pxNewListItems[0].clone());
    v_list_insert(&pxList, pxNewListItems[1].clone());
    let pxNewListItem: ListItemWeakLink = list_item_get_next(&Arc::downgrade(&pxNewListItems[0]));
    assert!(pxNewListItem.ptr_eq(&Arc::downgrade(&pxNewListItems[1])));
}

pub fn test_macro_listGET_OWNER_OF_HEAD_ENTRY() {
    let pxList: ListRealLink = Default::default();
    let pxNewListItems: [ListItemLink; 2] = Default::default();
    let owners: [ListItemOwnerWeakLink; 2] = Default::default();
    list_item_set_value(&pxNewListItems[0], 0);
    list_item_set_value(&pxNewListItems[1], 1);
    list_item_set_owner(&pxNewListItems[0], owners[0].clone());
    list_item_set_owner(&pxNewListItems[1], owners[1].clone());
    v_list_insert(&pxList, pxNewListItems[0].clone());
    v_list_insert(&pxList, pxNewListItems[1].clone());
    let saved_owner: ListItemOwnerWeakLink = list_get_owner_of_head_entry(&pxList);
    assert!(saved_owner.ptr_eq(&owners[0]));
}

pub fn test_macro_listIS_CONTAINED_WITHIN() {
    let pxList: ListRealLink = Default::default();
    let pxNewListItems: [ListItemLink; 2] = Default::default();
    v_list_insert(&pxList, pxNewListItems[0].clone());
    assert!(!list_is_contained_within(&pxList, &pxNewListItems[1]));
    v_list_insert(&pxList, pxNewListItems[1].clone());
    assert!(list_is_contained_within(&pxList, &pxNewListItems[1]));
    ux_list_remove(Arc::downgrade(&pxNewListItems[1]));
    assert!(!list_is_contained_within(&pxList, &pxNewListItems[1]));
}

pub fn test_func_list(t: *mut c_void) {
    vSendString("testing list");
    let xList: ListT = Default::default();
    let pxList: ListRealLink = Arc::new(RwLock::new(xList));
    validate_empty_list(pxList);
    test_vListInsertEnd_Success_1_item();
    test_vListInsert_success_1_item();
    test_uxListRemove_success();
    test_macro_listSET_GET_LIST_ITEM_VALUE();
    test_macros_list_SET_GET_LIST_ITEM_OWNER();
    test_macro_listGET_NEXT();
    test_macro_listGET_OWNER_OF_HEAD_ENTRY();
    test_macro_listIS_CONTAINED_WITHIN();
    vSendString("test passed!");
    loop {}
}

lazy_static! {
    pub static ref task1handler: Option<TaskHandle_t> =
        Some(Arc::new(RwLock::new(tskTaskControlBlock::default())));
}

pub fn test_main_list() {
    let param1: Param_link = 0;
    unsafe {
        xTaskCreate(
            test_func_list as u32,
            "test_func",
            USER_STACK_SIZE as u32,
            Some(param1),
            3,
            Some(Arc::clone(&(task1handler.as_ref().unwrap()))),
        );
    }
    vTaskStartScheduler();
    loop {
        panic! {"error in loop!!!!!"};
    }
}
