type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Node<T>> {
        Box::new(Node { value, next: None })
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
        let mut new_node = Node::new(value);
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
            //map的时候，消耗的是引用，map是从option中解套的好法宝
            //返回的又是一个引用，因此不会引起所有权变动
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
            //self.next是一个引用，因此用map去消耗是没有问题
            self.next = node.next.as_ref().map(|node| {
                //因为node.next不是引用，因此map会转移所有权，这会导致后面乱套，因此需要先取引用然后再map就没有问题了
                &**node //返回一个引用不会消耗所有权
            });
            &node.value //返回一个引用不会消耗所有权
        })
    }
}


pub struct ListMutRefIterator<'a, T>{
    next : Option<&'a mut Node<T>>
}
impl<'a, T> Iterator for ListMutRefIterator<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{//这里不能像上面那样as_ref之后再map，因为不是共享引用；因此必须先take出来，而take出来是没有副作用的，因为后续self.next马上就要被赋值了
            self.next = node.next.as_mut().map(|node| {
                &mut**node
            });
            &mut node.value
        })
    }
}
