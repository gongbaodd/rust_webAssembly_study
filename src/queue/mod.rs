#![allow(dead_code)]

use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Pointer<T>,
}

struct Queue<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn size(&mut self) -> u32 {
        let mut current = self.head.take();
        let mut count = 0;
        while let Some(rc_node) = current {
            count += 1;

            if let Ok(node) = Rc::try_unwrap(rc_node) {
                current = node.borrow_mut().next.take();
            } else {
                break;
            }
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn enqueue(&mut self, elem: T) {
        let node = Node {
            data: elem,
            next: None,
        };
        let node = Rc::new(RefCell::new(node));

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    pub fn dequeue(&mut self) {
        let _ = self
            .head
            .take()
            .map(|old_head| match old_head.borrow_mut().next.take() {
                None => self.tail = None,
                Some(new_head) => {
                    self.head = Some(new_head);
                }
            });
    }

    pub fn peek_head(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }

    pub fn peek_tail(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_queue() {
        let mut queue = Queue::<u32>::new();
        assert!(queue.peek_head().is_none());
        assert!(queue.peek_tail().is_none());
        assert_eq!(&queue.is_empty(), &true);
        assert_eq!(&queue.size(), &0);
    }

    #[test]
    fn enqueue() {
        let mut queue = Queue::<u32>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(&*queue.peek_head().unwrap(), &1);
        assert_eq!(&*queue.peek_tail().unwrap(), &3);

        assert_eq!(&queue.is_empty(), &false);
        assert_eq!(&queue.size(), &3);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::<u32>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        queue.dequeue();

        assert_eq!(&*queue.peek_head().unwrap(), &2);
        assert_eq!(&*queue.peek_tail().unwrap(), &3);

        assert_eq!(&queue.is_empty(), &false);
        assert_eq!(&queue.size(), &2);
    }
}
