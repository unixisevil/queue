use super::Queue;
use alloc::raw_vec::RawVec;
use std::ptr;
use std::slice;

pub struct BoundQueue<T> {
    data: RawVec<T>,
    head: usize,
    tail: usize,
}

impl<T> BoundQueue<T> {
    pub fn new(size: usize) -> Self {
        let mut buf = RawVec::new();
        buf.reserve(0, size + 1);

        BoundQueue {
            head: 0,
            tail: 0,
            data: buf,
        }
    }

    pub fn cap(&self) -> usize {
        self.data.cap()
    }

    pub fn is_full(&self) -> bool {
        return self.tail + 1 == self.head;
    }
}

impl<T> Queue<T> for BoundQueue<T> {
    fn push(&mut self, item: T) {
        let mut next = self.tail + 1;
        if next >= self.cap() {
            next = 0
        }
        if next == self.head {
            return;
        }
        let tail = self.tail;
        unsafe {
            self.write(tail, item);
        }
        self.tail = next;
    }

    fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }
        let mut next = self.head + 1;
        if next >= self.cap() {
            next = 0
        }
        let head = self.head;
        let v = unsafe { self.read(head) };
        self.head = next;
        return Some(v);
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}

impl<T> BoundQueue<T> {
    unsafe fn as_slice(&self) -> &[T] {
        slice::from_raw_parts(self.data.ptr(), self.data.cap())
    }

    unsafe fn as_slice_mut(&self) -> &mut [T] {
        slice::from_raw_parts_mut(self.data.ptr(), self.data.cap())
    }

    unsafe fn read(&mut self, off: usize) -> T {
        ptr::read(self.data.ptr().add(off))
    }

    unsafe fn write(&mut self, off: usize, item: T) {
        ptr::write(self.data.ptr().add(off), item);
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            pos: self.head,
            tail: self.tail,
            data: unsafe { self.as_slice() },
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            pos: self.head,
            tail: self.tail,
            data: unsafe { self.as_slice_mut() },
        }
    }
}

pub struct Iter<'a, T: 'a> {
    pos: usize,
    tail: usize,
    data: &'a [T],
}

pub struct IterMut<'a, T: 'a> {
    pos: usize,
    tail: usize,
    data: &'a mut [T],
}

pub struct IntoIter<T>(BoundQueue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for BoundQueue<T> {
    fn drop(&mut self) {
        for e in self.iter_mut() {
            unsafe {
                ptr::drop_in_place(e as *mut _);
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.pos;
        if c == self.tail {
            return None;
        }
        if c == self.data.len() - 1 {
            self.pos = 0;
        } else {
            self.pos += 1;
        }
        unsafe { Some(self.data.get_unchecked(c)) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.pos;
        if c == self.tail {
            return None;
        }
        if c == self.data.len() - 1 {
            self.pos = 0;
        } else {
            self.pos += 1;
        }
        unsafe {
            let item = self.data.get_unchecked_mut(c);
            Some(&mut *(item as *mut _))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let q = BoundQueue::<i32>::new(10);
        assert_eq!(q.is_empty(), true);
    }

    #[test]
    fn test_push_pop() {
        let mut q = BoundQueue::<i32>::new(10);
        for i in 1..=10 {
            q.push(i);
            assert_eq!(q.pop(), Some(i));
        }
    }
    #[test]
    fn into_iter() {
        let mut q = BoundQueue::<i32>::new(10);
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
        let mut q = BoundQueue::<String>::new(10);
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
        let mut q = BoundQueue::<i32>::new(10);
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
