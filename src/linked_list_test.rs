use crate::linked_list::*;
// use core::cell::RwLock;\
use spin::RwLock;
use core::default::Default;
use alloc::sync::Arc;
use core::clone::Clone;
use alloc::format;
use crate::riscv_virt::vSendString;

pub fn ll_test() {
    // let list: List<u32> = List::new();

    let a: ListItemT = ListItemT::default();
    let mut l: ListT = ListT::default();
    let a_p = Arc::new(RwLock::new(a));
    let l_p = Arc::new(RwLock::new(l));
    let a_p2 = Arc::new(RwLock::new(ListItemT::new(2)));
    let a_p3 = Arc::new(RwLock::new(ListItemT::new(3)));
    let a_p5 = Arc::new(RwLock::new(ListItemT::new(5)));
    // v_list_insert_end(&l_p, a_p.clone());
    v_list_insert(&l_p, a_p2.clone());
    v_list_insert(&l_p, a_p3.clone());

    let a_p4 = Arc::new(RwLock::new(ListItemT::new(4)));
    v_list_insert(&l_p, a_p4.clone());
    ux_list_remove(Arc::downgrade(&a_p2.clone()));
    v_list_insert(&l_p, a_p5.clone());
    // l.v_list_insert_end(Arc::downgrade(&Arc::new(RwLock::new(a))));
    // println!("{:?}", a);
    // println!("{:?}", l);
    // println!(
    //     "a_p strong = {}, weak = {}",
    //     Arc::strong_count(&a_p),
    //     Arc::weak_count(&a_p),
    // );
    // println!("Hello, world!");
    let s = format!("a_p strong = {}, weak = {}",
    Arc::strong_count(&a_p),
    Arc::weak_count(&a_p));
    vSendString(&s);
}
