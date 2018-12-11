#![feature(box_into_raw_non_null)]
#![feature(alloc, raw_vec_internals)]

extern crate alloc;

pub trait Queue<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

pub mod bound;
pub mod unbound;
