mod link_list_v1;
mod test;
use link_list_v1::List;

fn main() {
    let mut list = List::new();
    for i in 1..=5 {
        list.push(i);
    }
    
    // assert_eq!(list.pop(), Some(5));
    // assert_eq!(list.pop(), Some(4));
    // assert_eq!(list.pop(), Some(3));
    // assert_eq!(list.pop(), Some(2));
    // assert_eq!(list.pop(), Some(1));
    // assert_eq!(list.pop(), None);
    // assert_eq!(list.pop(), None);

    for i in list.iter_mut(){
        *i = *i - 66;
        println!("mut iterator {:?}", i)
    }

    for i in list.iter(){
        println!("ref iterator {:?}", i)
    }



    // println!("{:?}", list);


    for i in list.into_iter(){
        println!("non ref iterator {:?}", i)
    }

    
}
