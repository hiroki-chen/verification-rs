use prusti_contracts::*;

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

fn main() {}
