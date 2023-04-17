use prusti_contracts::*;

#[allow(unused)]
#[ensures(old(a.len()) == a.len())]
#[ensures(a.len() > 1 ==>
    forall(|i: usize|
        1 <= i && i < a.len() ==> a[i - 1] <= a[i]
    )
)]
fn sort(a: &mut [i32]) {
    if a.len() == 0 {
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

fn main() {}
