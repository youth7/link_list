use std::{cell::{RefCell, RefMut}, rc::Rc};

mod link_list_v1;
mod link_list_v2;
mod link_list_v3;
// use std::rc::Rc;

// use link_list_v1::List;

fn main() {
    // test_v1();
    // test_v2();
    test_v3();
}

fn test_v1() {
    let mut list = link_list_v1::List::new();
    for i in 1..=5 {
        list.push(i);
    }

    for (i, v) in list.iter_mut().enumerate() {
        *v = *v - 66;
        assert_eq!(*v, 5 - (i as i32) - 66);
    }

    for (i, v) in list.iter().enumerate() {
        assert_eq!(*v, 5 - (i as i32) - 66);
    }

    println!("{:?}", list.get_head());

    for (i, v) in list.into_iter().enumerate() {
        assert_eq!(v, 5 - (i as i32) - 66);
    }
}

fn test_v2() {
    let mut list = link_list_v2::List::new();
    list = list.prepend(1).prepend(2).prepend(3);
    println!("{:?}", list);

    for i in list.iter() {
        println!("{:?}", i);
    }

    list = list.tail();
    println!("{:?}", list);
    list = list.tail();
    println!("{:?}", list);
    list = list.tail();
    println!("{:?}", list);
    list = list.tail();
    println!("{:?}", list);
}

fn test_v3() {
    let mut list = link_list_v3::List::<i32>::new();
    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    list.push_front(4);
    list.push_front(5);
    list.push_back(6);
    list.push_back(7);
    list.push_back(8);
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());
    println!("{:?}", list.pop_back());
    println!("peek {:?}", list.peek_back());

    list.push_back(9);
    list.push_front(10);
    println!("peek {:?} last", list.peek_back());

}
