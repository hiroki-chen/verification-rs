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

/// An awkward solution to the external specifications for [`Vec<T>`] due to the complex behavior of std containers.
/// For example, Prusti is unable to encode the [`std::ops::Index`] trait into Viper, which will be invoked when one
/// uses the `[]` operator. Also, the implementation of `Index` requires another trait bound called [`std::slice::SliceIndex`],
/// which introduces more generics and complex function behaviors again. Thus, the simplest workaround would be to
/// wrap [`Vec<T>`] in a custom struct and impose constraints on this struct whose member functions are mared as `trusted`.
pub struct VecWrapper<T> {
    inner: Vec<T>,
}

impl<T> VecWrapper<T> {
    #[trusted]
    #[ensures(result.len() == 0)]
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[pure]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[trusted]
    #[ensures(old(self.len()) + 1 == self.len())]
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    #[trusted]
    #[requires(idx < self.len())]
    pub fn get(&self, idx: usize) -> &T {
        self.inner.get(idx).unwrap()
    }

    #[trusted]
    #[requires(idx < self.len())]
    #[after_expiry(self.len() == old(self.len()))]
    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        self.inner.get_mut(idx).unwrap()
    }
}

impl<T: PartialOrd + Copy> VecWrapper<T> {
    #[trusted]
    #[pure]
    #[requires(i < self.len() && j < self.len())]
    pub fn le(&self, i: usize, j: usize) -> bool {
        self.get(i) <= self.get(j)
    }
}

/// For generics, one must give specifications on each concrete implementation to allow Prusti to perform encoding.
/// For example, the concrete type of this function is [`i64`], then we must mark `PartialOrd<i64>` with an `extern_spec`.
/// 
/// Warning: Prusti has difficulty verifying this function.
#[allow(unused)]
#[ensures(arr.len() == old(arr.len()))]
#[ensures(forall(|i: usize, j: usize|
    i < j && j < arr.len() ==> arr.le(i, j) /* Prusti will get confused if we directly call <&T as PartialOrd>::le */
))]
fn selection_sort_vec_wrapper<T: PartialOrd + Copy>(arr: &mut VecWrapper<T>) {
    if arr.len() <= 1 {
        return;
    }

    let mut i = 0usize;
    let len = arr.len();

    while i < len {
        body_invariant!(i < len && len == arr.len());
        body_invariant!(arr.le(0, i));
        // 1. arr[..i] is properly sorted, if i >= 2.:
        body_invariant!(forall(|k1: usize, k2: usize|
            k1 < i && i <= k2 && k2 < len ==> arr.le(k1, k2)
        ));
        // 2. arr[..i] <= arr[i..]
        body_invariant!(forall(|k1: usize, k2: usize|
            k1 < k2 && k2 < i ==> arr.le(k1, k2)
        ));

        let mut j = i + 1;
        let mut min = i;

        while j < len {
            // Copied.
            body_invariant!(i < len && len == arr.len());
            body_invariant!(arr.le(0, i));
            body_invariant!(forall(|k1: usize, k2: usize|
                k1 < i && i <= k2 && k2 < len ==> arr.le(k1, k2)
            ));
            body_invariant!(forall(|k1: usize, k2: usize|
                k1 < k2 && k2 < i ==> arr.le(k1, k2)
            ));

            // 3. range is correct.
            body_invariant!(i < j && j < len);
            // 4. min is in [i,j]
            body_invariant!(i <= min && min <= j);
            // 5. arr[min] >= arr[..i].
            body_invariant!(forall(|k: usize|
                k < i ==> arr.le(k, min)
            ));
            // 6. arr[min] <= arr[i..]
            body_invariant!(forall(|k: usize|
                i <= k && k < j && k < len ==> arr.le(min, k)
            ));

            if arr.le(j, min) {
                min = j;
            }

            j += 1;
        }

        let arr_i = *arr.get(i);
        let arr_min = *arr.get(min);
        *arr.get_mut(i) = arr_min;
        *arr.get_mut(min) = arr_i;

        i += 1;
    }
}

#[allow(unused)]
fn test() {
    let mut arr = VecWrapper::new();
    arr.push(9i64);
    arr.push(8);
    arr.push(7);
    arr.push(6);
    arr.push(5);
    selection_sort_vec_wrapper(&mut arr);
}
