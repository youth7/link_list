use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    value: T,
    pre: Link<T>,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            pre: None,
            next: None,
        }
    }
}


impl<T> List<T> {


    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }
    pub fn push_front(&mut self, value: T) {
        let node = Node::new(value);
        let new_node = Rc::new(RefCell::new(node));
        match self.head.take() {
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
                // println!("push count {}", Rc::strong_count(&new_node));
            }
            Some(old_head) => {
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().pre = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
                // println!("push count {}", Rc::strong_count(&new_node));
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // println!("pop count {}", Rc::strong_count(&self.head.));
        self.head.take().map(|old_head| {
            // println!("pop count {}", Rc::strong_count(&old_head));
            match old_head.borrow_mut().next.take() {
                None => {
                    self.tail = None;
                    // println!("pop count {}", Rc::strong_count(&old_head));
                }
                Some(new_head) => {
                    new_head.borrow_mut().pre = None;
                    self.head = Some(new_head);
                }
            }
            // println!("pop count {}", Rc::strong_count(&old_head));
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.tail.take() {
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().pre = Some(Rc::clone(&old_tail));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            let new_tail = old_tail.borrow_mut().pre.take();
            match new_tail {
                None => {
                    self.head = None;
                }
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(Rc::clone(&new_tail));
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }
}
