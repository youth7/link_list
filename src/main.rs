use std::rc::Rc;

mod link_list_v1;
mod link_list_v2;

// use std::rc::Rc;

// use link_list_v1::List;

fn main() {
    test_v1();
    test_v2();
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
