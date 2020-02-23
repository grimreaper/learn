use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link: Link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}


#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn can_be_constructed() {
        let _ = List::new;
    }

    #[test]
    fn push_value_pops_value() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn push_pop_multiple_values() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn push_pop_and_repushes() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        list.push(2);
        assert_eq!(list.pop(), Some(2));
    }


    #[test]
    fn no_push_gets_none() {
        let mut list = List::new();
        assert_eq!(list.pop(), None)
    }

    #[test]
    fn push_pop_gets_value_and_none_after() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None)
    }


}
