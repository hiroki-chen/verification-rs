use prusti_contracts::*;

predicate! {
    fn sorted(arr: &[i64]) -> bool {
        forall(|k1: usize, k2: usize|
            k1 < k2 && k2 < arr.len() ==> arr[k1] <= arr[k2]
        )
    }
}

#[allow(unused)]
#[ensures(old(a.len()) == a.len())]
#[ensures(sorted(a))]
fn bubble_sort(a: &mut [i64]) {
    if a.len() <= 1 {
        return;
    }
    let mut i = 0;
    while i < a.len() - 1 {
        body_invariant!(true);
        body_invariant!(a.len() >= 1);

        let mut j = 0;
        while j < a.len() - i - 1 {
            body_invariant!(true);

            if a[j] > a[j + 1] {
                let tmp = a[j];
                a[j] = a[j + 1];
                a[j + 1] = tmp;
            }
            j += 1;
        }
    }

    i += 1;
}

#[allow(unused)]
#[ensures(old(arr.len()) == arr.len())]
#[ensures(sorted(arr))]
fn selection_sort(arr: &mut [i64; 10]) {
    if arr.len() <= 1 {
        return;
    }

    let mut i = 0usize;
    let len = arr.len();

    while i < len {
        body_invariant!(i < len);
        // Necessary: edge condition.??
        body_invariant!(arr[0] <= arr[i]);
        // 1. arr[..i] is properly sorted, if i >= 2.
        body_invariant!(forall(|k1: usize, k2: usize|
            k1 < i && i <= k2 && k2 < len ==> arr[k1] <= arr[k2]
        ));
        // 2. arr[..i] <= arr[i..]
        body_invariant!(forall(|k1: usize, k2: usize|
            k1 < k2 && k2 < i ==> arr[k1] <= arr[k2]
        ));

        let mut j = i + 1;
        let mut min = i;

        while j < len {
            // Copied.
            body_invariant!(i < len);
            body_invariant!(arr[0] <= arr[i]);
            body_invariant!(forall(|k1: usize, k2: usize|
                k1 < i && i <= k2 && k2 < len ==> arr[k1] <= arr[k2]
            ));
            body_invariant!(forall(|k1: usize, k2: usize|
                k1 < k2 && k2 < i ==> arr[k1] <= arr[k2]
            ));

            // 3. range is correct.
            body_invariant!(i < j && j < len);
            // 4. min is in [i,j]
            body_invariant!(i <= min && min <= j);
            // 5. arr[min] >= arr[..i].
            body_invariant!(forall(|k: usize|
                k < i ==> arr[k] <= arr[min]
            ));
            // 6. arr[min] <= arr[i..]
            body_invariant!(forall(|k: usize|
                i <= k && k < j && k < len ==> arr[min] <= arr[k]
            ));

            if arr[j] < arr[min] {
                min = j;
            }

            j += 1;
        }

        let arr_i = arr[i];
        let arr_min = arr[min];
        arr[i] = arr_min;
        arr[min] = arr_i;

        i += 1;
    }
}

/// Timed out?
#[allow(unused)]
#[ensures(old(arr.len()) == arr.len())]
#[ensures(sorted(arr))]
fn insertion_sort(arr: &mut [i64]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();
    let mut i = 1usize;
    while i < len {
        // 1. Loop condition.
        body_invariant!(
            1 <= i && i < len && len == arr.len() /* Strange condition: len == arr.len() */
        );
        // 2. arr[0..i] is sorted.
        body_invariant!(forall(|k1: usize, k2: usize|
            k1 < k2 && k2 < i - 1 ==> arr[k1] <= arr[k2]
        ));
        // 3. arr[0..i] <= arr[i..]
        body_invariant!(forall(|k1: usize, k2: usize|
            k1 < i - 1 && i - 1 <= k2 && k2 < len ==> arr[k1] <= arr[k2]
        ));

        let mut j = i;
        let key = arr[i];
        // To allow Prusti to encode the loop condition into Viper.
        let mut loop_condition = j > 0 && arr[j - 1] > key;
        while loop_condition {
            // 1. Loop condition.
            body_invariant!(i < len && j > 0 && arr[j - 1] > key && len == arr.len());
            // 2. arr[0..i] is sorted.
            body_invariant!(forall(|k1: usize, k2: usize|
                k1 < k2 && k2 < i - 1 ==> arr[k1] <= arr[k2]
            ));
            // 3. arr[0..i] <= arr[i..]
            body_invariant!(forall(|k1: usize, k2: usize|
                k1 < i - 1 && i - 1 <= k2 && k2 < len ==> arr[k1] <= arr[k2]
            ));
            // 4. j is bounded.
            body_invariant!(j <= i);

            arr[j] = arr[j - 1];
            j -= 1;
            loop_condition = j > 0 && arr[j - 1] > key;
        }

        arr[j] = key;
        i += 1;
    }
}

fn main() {
    #[cfg(not(prusti))]
    {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        println!("{:?}", arr);
    }
}
