#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| {
          &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link: Option<Box<Node<T>>> = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[cfg(test)]
mod test {
    use crate::second::List;

    #[test]
    fn push_value_pops_value() {
        let mut list: List<i32> = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn push_pop_multiple_values() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn push_pop_and_repushes() {
        let mut list: List<i32> = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        list.push(2);
        assert_eq!(list.pop(), Some(2));
    }


    #[test]
    fn no_push_gets_none() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None)
    }

    #[test]
    fn push_pop_gets_value_and_none_after() {
        let mut list: List<i32> = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None)
    }

    #[test]
    fn test_push_and_peek_and_pops_then_none() {
        let mut list: List<i32> = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None)
    }

    #[test]
    fn test_peek_mut_allows_mutating() {
        let mut list: List<i32> = List::new();
        list.push(1);
        let peeked: Option<&mut i32> = list.peek_mut();
        assert_eq!(peeked, Some(&mut 1));
        peeked.map(|node| {
            *node = 2
        });
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
    }



}
