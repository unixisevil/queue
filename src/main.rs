extern crate queue;
use queue::*;

fn main(){
    let   q1  = bound::BoundQueue::<i32>::new(10);
    push_pop(q1, 432);
    let   q2  = unbound::UnboundQueue::<&str>::new();
    push_pop(q2, "hello");
}

fn  push_pop<Q,T>(mut q: Q, item :T) where Q:Queue<T>, T: std::fmt::Debug {
    q.push(item);
    println!("{:?}", q.pop())
}
