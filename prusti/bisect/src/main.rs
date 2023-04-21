use prusti_contracts::*;

predicate! {
    fn sorted(s: &[i64]) -> bool {
        forall(|i: usize, j: usize| (i < j && j < s.len()) ==> s[i] <= s[j])
    }
}

/// A monotonically increasing discrete function, with domain [0, domain_size)
trait BisectFunction {
    #[pure]
    fn domain_size(&self) -> usize;

    #[pure]
    #[requires(x <= self.domain_size())]
    fn eval(&self, x: usize) -> i32;
}

/// Find the `x` s.t. `f(x) == target`
#[allow(unused)]
#[ensures(if let Some(x) = result {
    f.eval(x) == target
} else {
    // Not found.
    true
})]
fn bisect<T: BisectFunction>(f: &T, target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = f.domain_size();
    while low < high {
        // Add a body invariant that limits the index of `mid` by contraining `low` and `high`.
        body_invariant!(low < high && high <= f.domain_size());

        let mid = low + (high - low) / 2;
        let mid_val = f.eval(mid);
        if mid_val < target {
            low = mid + 1;
        } else if mid_val > target {
            high = mid;
        } else {
            return Some(mid);
        }
    }
    None
}

#[allow(unused)]
#[requires(sorted(arr))]
#[ensures(
    if let Some(res) = result {
        arr[res] == target
    } else {
        true
    }
)]
fn binary_search(arr: &[i64], target: i64) -> Option<usize> {
    let mut lhs = 0;
    let mut rhs = arr.len();

    while lhs < rhs {
        body_invariant!(lhs < rhs && rhs <= arr.len());

        let mid = lhs + (rhs - lhs) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] > target {
            rhs = mid;
        } else {
            lhs = mid + 1;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 8, 9];
    // Should be `None`.
    let res = binary_search(&arr, 10);
    if let Some(target) = res {
        assert!(arr[target] == 10);
    }
}
