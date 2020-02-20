#![allow(dead_code)]

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            data: elem,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut head) => {
                self.head = head.next.take();
                Some(head.data)
            }
        }
    }

    pub fn pop_fun(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            self.head = head.next.take();
            head.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_stack() {
        let mut s = Stack::<u32>::new();

        match &s.pop() {
            Some(_) => panic!("should not pop value!"),
            None => {}
        }
    }

    #[test]
    fn push_and_pop() {
        let mut s = Stack::<u32>::new();

        &s.push(33);

        match &s.pop() {
            Some(v) => assert_eq!(v, &33),
            None => panic!("should pop value here!"),
        }

        match &s.pop() {
            Some(_) => panic!("should not pop value!"),
            None => {}
        }
    }

    #[test]
    fn push_and_pop_fun() {
        let mut s = Stack::<u32>::new();

        &s.push(33);

        match &s.pop_fun() {
            Some(v) => assert_eq!(v, &33),
            None => panic!("should pop value here!"),
        }

        match &s.pop_fun() {
            Some(_) => panic!("should not pop value!"),
            None => {}
        }
    }
}
