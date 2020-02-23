use std::mem;

#[derive(Debug)]
pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None)
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => {
                None
            }
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link: Option<Box<Node>> = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}


#[derive(Debug)]
struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

#[cfg(test)]
mod test {
    use crate::first::List;

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
