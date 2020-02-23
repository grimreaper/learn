
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

#[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
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

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node)
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link: Link<T> = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[cfg(test)]
mod test {
    use crate::second::{List, Iter, IterMut};

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

    #[test]
    fn into_iter_pops_and_ends_in_none() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_pops_and_ends_in_none() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);

        let mut iter: Iter<i32> = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_pops_and_ends_with_none() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);

        let mut iter: IterMut<i32> = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None)
    }
}
