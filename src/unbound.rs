use super::Queue;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    data: T,
}

pub struct UnboundQueue<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            next: None,
            data: data,
        }
    }
}

impl<T> UnboundQueue<T> {
    pub fn new() -> Self {
        UnboundQueue {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }
}

impl<T> Queue<T> for UnboundQueue<T> {
    fn push(&mut self, item: T) {
        let boxnode = Box::new(Node::new(item));
        self.push_node(boxnode);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| node.data)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> UnboundQueue<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        let node = Some(Box::into_raw_non_null(node));
        unsafe {
            match self.tail {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().next = node,
            }
            self.tail = node;
            self.len += 1;
        }
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            if let None = self.head {
                self.tail = None;
            }
            self.len -= 1;
            node
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            pos: self.head.as_ref().map(|node| unsafe { node.as_ref() }),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            pos: self.head.as_mut().map(|node| unsafe { node.as_mut() }),
        }
    }
}

impl<T> Drop for UnboundQueue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_node() {}
    }
}

pub struct Iter<'a, T: 'a> {
    pos: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    pos: Option<&'a mut Node<T>>,
}

pub struct IntoIter<T>(UnboundQueue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos.map(|node| {
            self.pos = node.next.as_ref().map(|node| unsafe { node.as_ref() });
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos.take().map(|node| {
            self.pos = node.next.as_mut().map(|node| unsafe { node.as_mut() });
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut q = UnboundQueue::<i32>::new();
        for i in 1..=10 {
            q.push(i);
            assert_eq!(q.pop(), Some(i));
        }
        for i in 1..=10 {
            q.push(i);
        }
        for i in 1..=10 {
            assert_eq!(q.pop(), Some(i));
        }
    }

    #[test]
    fn into_iter() {
        let mut q = UnboundQueue::<i32>::new();
        q.push(1);
        q.push(2);
        q.push(3);

        let mut iter = q.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut q = UnboundQueue::<String>::new();
        q.push(1.to_string());
        q.push(2.to_string());
        q.push(3.to_string());

        let mut iter = q.iter();
        assert_eq!(iter.next(), Some(&"1".to_string()));
        assert_eq!(iter.next(), Some(&"2".to_string()));
        assert_eq!(iter.next(), Some(&"3".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut q = UnboundQueue::<i32>::new();
        q.push(1);
        q.push(2);
        q.push(3);

        let mut iter = q.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
