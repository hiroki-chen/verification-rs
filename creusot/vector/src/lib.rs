#![cfg_attr(not(creusot), feature(proc_macro_hygiene))]
#![cfg_attr(not(creusot), feature(stmt_expr_attributes))]

use creusot_contracts::{logic::*, *};

#[cfg(creusot)]
#[predicate]
fn sorted_range<T: OrdLogic>(s: Seq<T>, l: Int, u: Int) -> bool {
    pearlite! {
        forall<i : Int, j : Int> l <= i && i < j && j < u ==> s[i] <= s[j]
    }
}

#[cfg(creusot)]
#[predicate]
fn sorted<T: OrdLogic>(s: Seq<T>) -> bool {
    sorted_range(s, 0, s.len())
}

#[cfg(creusot)]
#[predicate]
fn partition<T: OrdLogic>(v: Seq<T>, i: Int) -> bool {
    pearlite! { forall<k1 : Int, k2: Int> 0 <= k1 && k1 < i && i <= k2 && k2 < v.len() ==> v[k1] <= v[k2]}
}

#[requires(v.deep_model().sorted())]
#[ensures((^v).deep_model().sorted())]
pub fn insert_in_order<T>(v: &mut Vec<T>, item: T)
where
    T: Ord + DeepModel,
    T::DeepModelTy: OrdLogic,
{
    let mut insert_idx = 0;

    if v.last() <= Some(&item) {
        insert_idx = v.len();
    } else {
        #[invariant(prev_leq, forall<i: Int> 0 <= i && i < produced.len() ==>
            v.deep_model()[i] <= item.deep_model()
        )]
        for idx in 0..v.len() {
            if v[idx] > item {
                insert_idx = idx;
                break;
            }
        }
    }

    v.insert(insert_idx, item);
}

#[ensures(sorted((^v).deep_model()))]
#[ensures((^v)@.permutation_of(v@))]
pub fn selection_sort<T: Ord + DeepModel>(v: &mut Vec<T>)
where
    T::DeepModelTy: OrdLogic,
{
    #[cfg(creusot)]
    let old_v = ghost! { v };
    #[invariant(permutation, v@.permutation_of(old_v@))]
    #[invariant(sorted, sorted_range(v.deep_model(), 0, produced.len()))]
    #[invariant(partition, partition(v.deep_model(), produced.len()))]
    for i in 0..v.len() {
        let mut min = i;
        #[invariant(min_is_min, forall<k: Int> i@ <= k && k < produced.len() + i@ + 1 ==> v.deep_model()[min@] <= v.deep_model()[k])]
        #[invariant(min_bound, i@ <= min@ && min@ < produced.len() + i@ + 1)]
        for j in (i + 1)..v.len() {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
}

/// Loop invariants are not sufficient.
#[ensures(sorted((^v).deep_model()))]
#[ensures((^v)@.permutation_of(v@))]
pub fn bubble_sort<T>(v: &mut Vec<T>)
where
    T: Ord + DeepModel,
    T::DeepModelTy: OrdLogic,
{
    #[cfg(creusot)]
    let old_v = ghost! { v };
    let len = v.len();
    #[invariant(permutation, v@.permutation_of(old_v@))]
    #[invariant(sorted, sorted_range(v.deep_model(), len@ - produced.len(), len@))]
    for i in 0..v.len() {
        #[invariant(partition, forall<k1: Int, k2: Int> k1 <= 0 && k1 <= i@ && k1 < k2 && i@ < k2 && k2 < len@ ==>
            v.deep_model()[k1] <= v.deep_model()[k2]
        )]
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

#[cfg(not(creusot))]
#[test]
fn test() {
    let mut v = ::std::vec![6, 5, 4, 3, 2, 1];
    bubble_sort(&mut v);
    println!("{v:?}");
}
