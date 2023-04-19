//! External specifications for `alloc::vec::Vec`.

use prusti_contracts::*;

#[extern_spec]
impl<T> Vec<T> {
    #[ensures(result.len() == 0)]
    fn new() -> Vec<T>;
}

#[extern_spec]
impl<T, A: std::alloc::Allocator> Vec<T, A> {
    #[pure]
    fn len(&self) -> usize;

    #[trusted]
    #[ensures(old(self.len()) == self.len() - 1)]
    fn push(&mut self, item: T);

    #[trusted]
    #[ensures(self.len() == 0)]
    fn clear(&mut self);

    #[trusted]
    #[ensures(old(self.len()) == 0 ==> result.is_none())]
    #[ensures(old(self.len()) > 0 ==> old(self.len()) - 1 == self.len() && result.is_some())]
    fn pop(&mut self) -> Option<T>;
}

/// Passed: construct an empty vector.
#[ensures(result.len() == 0)]
pub fn vec_must_be_empty<T>() -> Vec<T> {
    Vec::new()
}

#[ensures(vec.len() == 0)]
#[ensures(result.len() == old(vec.len()))]
pub fn pop_to_zero_correct<T>(vec: &mut Vec<T>) -> Vec<T> {
    let mut res = Vec::new();
    let mut i = 0usize;
    let len = vec.len();

    while i < len {
        body_invariant!(i < len);
        body_invariant!(res.len() == i);
        body_invariant!(vec.len() + i == len);

        let first = vec.pop();
        res.push(first.unwrap());

        i += 1;
    }

    res
}

#[ensures(result.len() == capacity)]
pub fn vec_with_capacity(capacity: usize) -> Vec<i64> {
    let mut vec = Vec::new();

    let mut i = 0usize;
    while i < capacity {
        body_invariant!(vec.len() == i);

        let item = 123;
        vec.push(item);
        i += 1;
    }

    vec
}
