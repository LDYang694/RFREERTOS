#![allow(non_snake_case)]
//! Bidirectional linked list Definition and API
extern crate alloc;
use crate::kernel::riscv_virt::*;
use crate::portable::portmacro::*;
use crate::portable::*;
use crate::tasks::*;
use alloc::format;
use alloc::sync::{Arc, Weak};
use core::clone::Clone;
use core::default::Default;
use spin::RwLock;
pub type ListItemWeakLink = Weak<RwLock<XListItem>>;
pub type ListWeakLink = Weak<RwLock<XList>>;
pub type ListRealLink = Arc<RwLock<XList>>;
pub type ListItemLink = Arc<RwLock<XListItem>>;
pub type ListItemOwnerWeakLink = Weak<RwLock<TCB_t>>;

use alloc::string;
use core::option::Option;

#[derive(Debug)]
pub struct XListItem {
    /// Used to help arrange nodes in order.
    pub x_item_value: TickType,
    /// Point to the next linked list item.
    pub px_next: ListItemWeakLink,
    /// Point to the previous linked list item.
    pub px_previous: ListItemWeakLink,
    /// Point to the linked list where the node is located, and point to the kernel object that owns the node.
    pub pv_owner: ListItemOwnerWeakLink,
    /// Point to the linked list where the node is located.
    pub px_container: ListWeakLink,
    pub pv_owner_c: usize,
}
pub type ListItemT = XListItem;
impl XListItem {
    pub fn new(value: TickType) -> Self {
        ListItemT {
            x_item_value: value,
            px_next: Default::default(),
            px_previous: Default::default(),
            pv_owner: Default::default(),
            px_container: Default::default(),
            pv_owner_c: Default::default(),
        }
    }
}
//链表节点初始化
impl Default for ListItemT {
    fn default() -> Self {
        ListItemT {
            x_item_value: 0, //TODO: set
            px_next: Default::default(),
            px_previous: Default::default(),
            pv_owner: Default::default(),
            px_container: Default::default(),
            pv_owner_c: Default::default(),
        }
    }
}
//#[derive(Debug)]
#[derive(Clone, Debug)]
pub struct XList {
    pub ux_number_of_items: UBaseType,
    px_index: ListItemWeakLink,
    x_list_end: Arc<RwLock<ListItemT>>,
}
pub type ListT = XList;
//链表根节点初始化
impl Default for ListT {
    fn default() -> Self {
        //得到一个list_end 然后设置其辅助排序值 并将其next和pre指向自身
        let x_list_end = Arc::new(RwLock::new(XListItem::default()));
        (x_list_end).write().x_item_value = PORT_MAX_DELAY;
        (x_list_end).write().px_next = Arc::downgrade(&x_list_end);
        (x_list_end).write().px_previous = Arc::downgrade(&x_list_end);
        ListT {
            ux_number_of_items: 0,
            px_index: Arc::downgrade(&x_list_end),
            x_list_end: x_list_end,
        }
    }
}

/// Set previous item of target item.
pub fn list_item_set_pre(item: &ListItemWeakLink, pre: ListItemWeakLink) {
    item.upgrade().unwrap().write().px_previous = pre;
}

/// Set next item of target item.
pub fn list_item_set_next(item: &ListItemWeakLink, next: ListItemWeakLink) {
    (item.upgrade().unwrap()).write().px_next = next;
}

/// Get previous item of target item.
pub fn list_item_get_pre(item: &ListItemWeakLink) -> ListItemWeakLink {
    let pre = Weak::clone(&(item.upgrade().unwrap()).read().px_previous);
    pre
}

/// Get next item of target item.
pub fn list_item_get_next(item: &ListItemWeakLink) -> ListItemWeakLink {
    let next = Weak::clone(&(item.upgrade().unwrap()).read().px_next);
    next
}

/// Set container of item.
pub fn list_item_set_container(item: &ListItemWeakLink, container: ListWeakLink) {
    (item.upgrade().unwrap()).write().px_container = container;
}

/// Get value of item. <br>
/// Insert() place item in order of values.
pub fn list_item_get_value(item: &ListItemLink) -> TickType {
    let value = (item).read().x_item_value;
    value
}

/// Set value of item. <br>
/// Insert() place item in order of values.
pub fn list_item_set_value(item: &ListItemLink, x_value: TickType) {
    (item).write().x_item_value = x_value;
}

/// Get head entry of list. <br>
/// Head entry is the first valid item in the list, unless the list is empty.<br>
/// In that case, the end marker will be returned.
pub fn list_get_head_entry(px_list: &ListRealLink) -> ListItemWeakLink {
    let entry = Weak::clone(&((px_list).read().x_list_end).read().px_next);
    entry
}

/// Get end marker of list. <br>
/// The end marker is not a valid item.
pub fn list_get_end_marker(px_list: &ListRealLink) -> ListItemWeakLink {
    let entry = Arc::downgrade(&(px_list).read().x_list_end);
    entry
}

/// Get container of item.
pub fn list_item_get_container(item: &ListItemWeakLink) -> ListWeakLink {
    let container = Weak::clone(&(item.upgrade().unwrap()).read().px_container);
    container
}

/// Set container of item.
pub fn list_item_set_owner(item: &ListItemLink, owner: ListItemOwnerWeakLink) {
    (item).write().pv_owner = Weak::clone(&owner);
}

/// Get owner of item. <br>
/// Owner is a tskTaskControlBlock object.
pub fn list_item_get_owner(item: &ListItemWeakLink) -> ListItemOwnerWeakLink {
    let owner = Weak::clone(&(item.upgrade().unwrap()).read().pv_owner);
    owner
}

/// Get owner of item. <br>
/// Owner is saved as C ptr.
pub fn list_item_get_c_owner(item: &ListItemWeakLink) -> Option<TaskHandle_t> {
    let owner = item.upgrade().unwrap().read().pv_owner_c;
    if owner == 0 {
        return None;
    } else {
        return Some(unsafe { Arc::from_raw(owner as *const RwLock<tskTaskControlBlock>) });
    }
}

/// Get num of item in list.
pub fn list_get_num_items(px_list: &ListWeakLink) -> UBaseType {
    let num = (px_list.upgrade().unwrap()).read().ux_number_of_items;
    num
}

/// Get current index of list.
pub fn list_get_pxindex(px_list: &ListWeakLink) -> ListItemWeakLink {
    let px_index = Weak::clone(&(px_list.upgrade().unwrap()).read().px_index);
    px_index
}

/// Set current index of list.
pub fn list_set_pxindex(px_list: &ListWeakLink, item: ListItemWeakLink) {
    (px_list.upgrade().unwrap()).write().px_index = item;
}

/// Return if the list is empty.
pub fn list_is_empty(px_list: &ListRealLink) -> bool {
    (px_list).read().ux_number_of_items == 0
}

/// Get num of item in list.
pub fn list_current_list_length(px_list: &ListRealLink) -> UBaseType {
    (px_list).read().ux_number_of_items
}

/// Get owner of next entry in list. <br>
/// Move current index to next item.
pub fn list_get_owner_of_next_entry(px_list: &ListRealLink) -> ListItemOwnerWeakLink {
    //add index and return owner
    let owner = px_list.write().get_owner_of_next_entry();
    owner
}

/// Get owner of head entry in list. <br>
/// Does not alter current index.
pub fn list_get_owner_of_head_entry(px_list: &ListRealLink) -> ListItemOwnerWeakLink {
    let owner = px_list.write().get_owner_of_head_entry();
    owner
}

/// Get owner of head entry in list. <br>
/// Owner is saved as C ptr. <br>
/// Does not alter current index.
pub fn list_get_c_owner_of_head_entry(px_list: &ListRealLink) -> Option<TaskHandle_t> {
    let ret = px_list.write().get_c_owner_of_head_entry();
    ret
}

impl ListT {
    /// Insert target item into end of list.
    pub fn insert_end(&mut self, px_new_list_item: ListItemWeakLink) {
        let px_index_pre = list_item_get_pre(&self.px_index);
        list_item_set_next(&px_new_list_item, Weak::clone(&self.px_index));
        list_item_set_pre(&px_new_list_item, Weak::clone(&px_index_pre));
        list_item_set_next(&px_index_pre, Weak::clone(&px_new_list_item));
        list_item_set_pre(&self.px_index, Weak::clone(&px_new_list_item));
        self.ux_number_of_items += 1;
    }

    /// Insert target item into list in ascending order of value. <br>
    /// If target item has value==PORT_MAX_DELAY, insert to list end. <br>
    /// If list is not already in order, insert position is not guaranteed.
    pub fn insert(&mut self, px_new_list_item: ListItemWeakLink) {
        let x_value_of_insertion = list_item_get_value(&Weak::upgrade(&px_new_list_item).unwrap());

        let px_iterator = if x_value_of_insertion == PORT_MAX_DELAY {
            list_item_get_pre(&(Arc::downgrade(&self.x_list_end)))
        } else {
            let mut iterator = Arc::downgrade(&self.x_list_end);
            loop {
                iterator = list_item_get_next(&iterator);
                let value = list_item_get_value(&Weak::upgrade(&iterator).unwrap());

                if value >= x_value_of_insertion {
                    break;
                }
            }
            iterator
        };

        list_item_set_next(&px_new_list_item, Weak::clone(&px_iterator));
        list_item_set_pre(
            &px_new_list_item,
            Weak::clone(&list_item_get_pre(&px_iterator)),
        );
        list_item_set_next(
            &list_item_get_pre(&px_iterator),
            Weak::clone(&px_new_list_item),
        );
        list_item_set_pre(&px_iterator, Weak::clone(&px_new_list_item));
        self.ux_number_of_items += 1;
    }

    /// Get owner of next entry in list. <br>
    /// Move current index to next item.
    pub fn get_owner_of_next_entry(&mut self) -> ListItemOwnerWeakLink {
        self.px_index = list_item_get_next(&self.px_index);
        if Weak::ptr_eq(&self.px_index, &Arc::downgrade(&self.x_list_end)) {
            self.px_index = list_item_get_next(&self.px_index);
        }

        let owner = Weak::clone(&self.px_index.upgrade().unwrap().read().pv_owner);
        owner
    }

    /// Get owner of head entry in list.
    pub fn get_owner_of_head_entry(&mut self) -> ListItemOwnerWeakLink {
        let end = self.x_list_end.read();
        let target: &ListItemWeakLink = &(end.px_next);

        list_item_get_owner(target)
    }

    /// Get owner stored in C standard of head entry in list.
    pub fn get_c_owner_of_head_entry(&mut self) -> Option<TaskHandle_t> {
        let end = self.x_list_end.read();
        let target: ListItemLink = end.px_next.upgrade().unwrap();
        let owner = target.read().pv_owner_c;
        if owner == 0 {
            return None;
        } else {
            return Some(unsafe { Arc::from_raw(owner as *const RwLock<tskTaskControlBlock>) });
        }
    }
}

//=====================对外接口=====================

/// Rnsert target item into end of list.
pub fn v_list_insert_end(px_list: &ListRealLink, px_new_list_item: &ListItemLink) {
    px_list.write().insert_end(Arc::downgrade(px_new_list_item));

    px_new_list_item.write().px_container = Arc::downgrade(&px_list);
}

/// Insert target item into list in ascending order of value. <br>
/// If target item has value==PORT_MAX_DELAY, insert to list end. <br>
/// If list is not already in order, insert position is not guaranteed.
pub fn v_list_insert(px_list: &ListRealLink, px_new_list_item: &ListItemLink) {
    px_list.write().insert(Arc::downgrade(px_new_list_item));

    px_new_list_item.write().px_container = Arc::downgrade(&px_list);
}

/// Remove target item from its container.<br>
/// Return number of items after remove.
pub fn ux_list_remove(px_item_to_remove: ListItemWeakLink) -> UBaseType {
    let px_list = list_item_get_container(&px_item_to_remove);
    match px_list.upgrade() {
        Some(x) => {}
        None => {
            return 0;
        }
    }
    list_item_set_pre(
        &list_item_get_next(&px_item_to_remove),
        list_item_get_pre(&px_item_to_remove),
    );
    list_item_set_next(
        &list_item_get_pre(&px_item_to_remove),
        list_item_get_next(&px_item_to_remove),
    );
    if Weak::ptr_eq(&px_item_to_remove, &list_get_pxindex(&px_list)) {
        list_set_pxindex(
            &px_list,
            Weak::clone(&list_item_get_pre(&px_item_to_remove)),
        );
    }

    (px_list.upgrade().unwrap()).write().ux_number_of_items -= 1;
    list_item_set_container(&px_item_to_remove, Default::default());
    list_get_num_items(&px_list)
}

/// Return if the list item is contained within target list.
pub fn list_is_contained_within(px_list: &ListRealLink, px_new_list_item: &ListItemLink) -> bool {
    let temp = Arc::downgrade(px_list);
    temp.ptr_eq(&px_new_list_item.read().px_container)
}

/// Return the value of list's head entry.
pub fn list_get_value_of_head_entry(px_list: &ListRealLink) -> UBaseType {
    list_item_get_value(&Weak::upgrade(&list_get_head_entry(px_list)).unwrap())
}
