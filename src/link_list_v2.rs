use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;//注意这里的意思是：Link是一个可能为空的堆指针，然而因为经过Option的包装，它并不是copy语义的

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Node<T> {
        Node { value, next }
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }
    pub fn tail(&self) -> List<T> {
        let next_node = self.head.as_ref().and_then(|head_ref|{
            head_ref.next.clone()
        });
        List {
            head:next_node
        }
    }

    pub fn prepend(&self, value: T) -> List<T> {
        let node = Node::new(value, self.head.as_ref().map(|node| Rc::clone(node)));
        List {
            head: Some(Rc::new(node)),
        }
    }

    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn iter(&self) -> ListRefIterator<T> {
        ListRefIterator {
            /*
                option<T>进行map的时候，如果T不是可copy的，则会导致move
                option<&T>进行map的时候，因&T是可copy的，则不会导致move
                因此可以知道为何下面实现Iterator的时候，可以直接map而无需先as_ref
            */
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

pub struct ListRefIterator<'a, T>{
    next : Option<&'a Node<T>>
}

impl<'a, T> Iterator for ListRefIterator<'a, T> {
    type Item = &'a T; //先设计这个的返回内容，然后就能明确ListRefIterator中的Option里应该放什么
    fn next(&mut self) -> Option<Self::Item> {
       self.next.map(|node|{
           self.next = node.next.as_deref();
           &node.value
       })
    }
}

