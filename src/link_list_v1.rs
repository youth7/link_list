type Link<T> = Option<Box<Node<T>>>;//注意这里的意思是：Link是一个可能为空的堆指针，然而因为经过Option的包装，它并不是copy语义的

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
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
    pub fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn into_iter(self) -> ListIterator<T> {
        ListIterator(self)
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

    pub fn iter_mut(&mut self)->ListMutRefIterator<T>{
        ListMutRefIterator{
            next: self.head.as_mut().map(|node|{
                &mut**node
            })
        }
    }

}

pub struct ListIterator<T>(List<T>);
impl<T> Iterator for ListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct ListRefIterator<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for ListRefIterator<'a, T> {
    type Item = &'a T; //先设计这个的返回内容，然后就能明确ListRefIterator中的Option里应该放什么
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| {
                &**node //返回一个引用不需要所有权，只涉及声明周期
            });
            &node.value //返回一个引用不需要所有权，只涉及声明周期
        })
    }
}


pub struct ListMutRefIterator<'a, T>{
    next : Option<&'a mut Node<T>>
}
impl<'a, T> Iterator for ListMutRefIterator<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            //这里不能像上面那样as_ref之后再map，因为不是共享引用；因此必须先take出来，take了之后虽然改变了self.next中的内容，但是没关系，因为后续self.next马上就要被赋值了
            self.next = node.next.as_mut().map(|node| {
                &mut**node
            });
            &mut node.value
        })
    }
}