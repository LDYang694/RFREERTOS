// use std::cell::RefCell;
use spin::RwLock;
// use std::collections::LinkedList;
// use std::rc::Rc;
use crate::tasks::*;
// use std::rc::Weak;
//use std::boxed::Box;
use crate::alloc::sync::{Arc, Weak};
use crate::portable::*;
use crate::portmacro::*;
use core::clone::Clone;
use core::default::Default;
// type Link<T> = Option<Box<Node<T>>>;
pub type ListItemWeakLink = Weak<RwLock<XListItem>>;
pub type ListWeakLink = Weak<RwLock<XList>>;
pub type ListRealLink = Arc<RwLock<XList>>;
pub type ListItemLink = Arc<RwLock<XListItem>>;
pub type ListItemOwnerWeakLink = Weak<RwLock<TCB_t>>;
const portMAX_DELAY: TickType = 0xffffffff;
//TODO: tmp define tcv_t
// pub type TCB = u32;
use alloc::string;
use core::option::Option;
//define list types here
#[derive(Debug)]
pub struct XListItem {
    pub x_item_value: TickType, /* 辅助值，用于帮助节点做顺序排列 */
    pub px_next: ListItemWeakLink,
    pub px_previous: ListItemWeakLink,
    pub pv_owner: ListItemOwnerWeakLink, /* 指向拥有该节点的内核对象，通常是 TCB */
    pub px_container: ListWeakLink,      /* 指向该节点所在的链表 */
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
        }
    }
}
#[derive(Debug)]
// #[derive(Clone, Debug)]
pub struct XList {
    ux_number_of_items: UBaseType,
    px_index: ListItemWeakLink,
    x_list_end: Arc<RwLock<ListItemT>>,
}
pub type ListT = XList;
//链表根节点初始化
impl Default for ListT {
    fn default() -> Self {
        //得到一个list_end 然后设置其辅助排序值 并将其next和pre指向自身
        let x_list_end = Arc::new(RwLock::new(XListItem::default()));
        (*x_list_end).write().x_item_value = portMAX_DELAY;
        (*x_list_end).write().px_next = Arc::downgrade(&x_list_end);
        (*x_list_end).write().px_previous = Arc::downgrade(&x_list_end);
        ListT {
            ux_number_of_items: 0,
            px_index: Arc::downgrade(&x_list_end),
            x_list_end: x_list_end,
        }
    }
}
pub fn list_item_set_pre(item: &ListItemWeakLink, pre: ListItemWeakLink) {
    (*(item.upgrade().unwrap())).write().px_previous = pre;
}
pub fn list_item_set_next(item: &ListItemWeakLink, next: ListItemWeakLink) {
    (*(item.upgrade().unwrap())).write().px_next = next;
}
pub fn list_item_get_pre(item: &ListItemWeakLink) -> ListItemWeakLink {
    let pre = Weak::clone(&(*(item.upgrade().unwrap())).read().px_previous);
    pre
}
pub fn list_item_get_next(item: &ListItemWeakLink) -> ListItemWeakLink {
    let next = Weak::clone(&(*(item.upgrade().unwrap())).read().px_next);
    next
}

pub fn list_item_set_container(item: &ListItemWeakLink, container: ListWeakLink) {
    (*(item.upgrade().unwrap())).write().px_container = container;
}
pub fn list_item_get_value(item: &ListItemWeakLink) -> TickType {
    let value = (*(item.upgrade().unwrap())).read().x_item_value;
    value
}
pub fn list_item_set_value(item: &ListItemWeakLink, x_value: TickType) {
    (*(item.upgrade().unwrap())).write().x_item_value = x_value;
}
//TODO:/* 初始化节点的拥有者 */
// 2 #define listSET_LIST_ITEM_OWNER( pxListItem, pxOwner )\
// 3 ( ( pxListItem )->pvOwner = ( void * ) ( pxOwner ) )??
/* 获取节点拥有者 */
// 6 #define listGET_LIST_ITEM_OWNER( pxListItem )\
// 7 ( ( pxListItem )->pvOwner )
pub fn list_get_head_entry(px_list: &ListRealLink) -> ListItemWeakLink {
    let entry = Weak::clone(&((*(px_list)).read().x_list_end).read().px_next);
    entry
}

pub fn list_get_end_marker(px_list: &ListRealLink) -> ListItemWeakLink {
    let entry = Arc::downgrade(&(*(px_list)).read().x_list_end);
    entry
}
pub fn list_item_get_container(item: &ListItemWeakLink) -> ListWeakLink {
    let container = Weak::clone(&(*(item.upgrade().unwrap())).read().px_container);
    container
}
pub fn list_item_set_owner(item: &ListItemLink, owner: ListItemOwnerWeakLink) {
    (*(item)).write().pv_owner = Weak::clone(&owner);
}

pub fn list_item_get_owner(item: &ListItemWeakLink) -> ListItemOwnerWeakLink {
    let owner = Weak::clone(&(*(item.upgrade().unwrap())).read().pv_owner);
    owner
}

pub fn list_get_num_items(px_list: &ListWeakLink) -> UBaseType {
    let num = (*(px_list.upgrade().unwrap())).read().ux_number_of_items;
    num
}
pub fn list_get_pxindex(px_list: &ListWeakLink) -> ListItemWeakLink {
    let px_index = Weak::clone(&(*(px_list.upgrade().unwrap())).read().px_index);
    px_index
}
pub fn list_set_pxindex(px_list: &ListWeakLink, item: ListItemWeakLink) {
    (*(px_list.upgrade().unwrap())).write().px_index = item;
}
pub fn list_is_empty(px_list: &ListWeakLink) -> bool {
    (*(px_list.upgrade().unwrap())).read().ux_number_of_items == 0
}
pub fn list_current_list_length(px_list: &ListWeakLink) -> UBaseType {
    (*(px_list.upgrade().unwrap())).read().ux_number_of_items
}
pub fn list_get_owner_of_next_entry(px_list: &ListRealLink) -> ListItemOwnerWeakLink {
    //add index and return owner
    let owner=px_list.write().get_owner_of_next_entry();
    owner
}
impl ListT {
    pub fn insert_end(&mut self, px_new_list_item: ListItemWeakLink) {
        //插入到list末尾
        //pre就是end
        let px_index_pre = list_item_get_pre(&self.px_index);
        list_item_set_next(&px_new_list_item, Weak::clone(&self.px_index));
        list_item_set_pre(&px_new_list_item, Weak::clone(&px_index_pre));
        list_item_set_next(&px_index_pre, Weak::clone(&px_new_list_item));
        list_item_set_pre(&self.px_index, Weak::clone(&px_new_list_item));
        self.ux_number_of_items += 1;
    }

    pub fn insert(&mut self, px_new_list_item: ListItemWeakLink) {
        let x_value_of_insertion = list_item_get_value(&px_new_list_item);
        //println!("{}", x_value_of_insertion);
        let px_iterator = if x_value_of_insertion == portMAX_DELAY {
            list_item_get_pre(&(Arc::downgrade(&self.x_list_end)))
        } else {
            let mut iterator = Arc::downgrade(&self.x_list_end);
            loop {
                iterator = list_item_get_next(&iterator);
                let value = list_item_get_value(&iterator);
                //println!(" insert find value {}", value);
                if value > x_value_of_insertion {
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

    pub fn get_owner_of_next_entry(&mut self) -> ListItemOwnerWeakLink {
        self.px_index = list_item_get_next(&self.px_index);
        if Weak::ptr_eq(&self.px_index, &Arc::downgrade(&self.x_list_end)) {
            self.px_index = list_item_get_next(&self.px_index);
        }

        let owner = Weak::clone(&self.px_index.upgrade().unwrap().read().pv_owner);
        owner
    }
}

// fn main() {
//     // let list: List<u32> = List::new();

//     let a: ListItemT = ListItemT::default();
//     let mut l: ListT = ListT::default();
//     let a_p = Arc::new(RefCell::new(a));
//     let l_p = Arc::new(RefCell::new(l));
//     let a_p2 = Arc::new(RefCell::new(ListItemT::new(2)));
//     let a_p3 = Arc::new(RefCell::new(ListItemT::new(3)));
//     let a_p5 = Arc::new(RefCell::new(ListItemT::new(5)));
//     // v_list_insert_end(&l_p, a_p.clone());
//     v_list_insert(&l_p, a_p2.clone());
//     v_list_insert(&l_p, a_p3.clone());

//     let a_p4 = Arc::new(RefCell::new(ListItemT::new(4)));
//     v_list_insert(&l_p, a_p4.clone());
//     ux_list_remove(Arc::downgrade(&a_p2.clone()));
//     v_list_insert(&l_p, a_p5.clone());
//     // l.v_list_insert_end(Arc::downgrade(&Arc::new(RefCell::new(a))));
//     // println!("{:?}", a);
//     // println!("{:?}", l);
//     println!(
//         "a_p strong = {}, weak = {}",
//         Arc::strong_count(&a_p),
//         Arc::weak_count(&a_p),
//     );
//     println!("Hello, world!");
// }

//=====================对外接口=====================
pub fn v_list_insert_end(px_list: &ListRealLink, px_new_list_item: ListItemLink) {
    px_list
        .write()
        .insert_end(Arc::downgrade(&px_new_list_item));

    px_new_list_item.write().px_container = Arc::downgrade(&px_list);
}
pub fn v_list_insert(px_list: &ListRealLink, px_new_list_item: ListItemLink) {
    px_list.write().insert(Arc::downgrade(&px_new_list_item));

    px_new_list_item.write().px_container = Arc::downgrade(&px_list);
}

pub fn ux_list_remove(px_item_to_remove: ListItemWeakLink) -> UBaseType {
    let px_list = list_item_get_container(&px_item_to_remove);
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
    //TODO:pxItemToRemove->pvContainer = NULL;
    (*(px_list.upgrade().unwrap())).write().ux_number_of_items -= 1;
    list_get_num_items(&px_list)
}
