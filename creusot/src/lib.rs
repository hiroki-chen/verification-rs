use creusot_contracts::*;

#[ensures((^vec).deep_model().sorted())]
pub fn sort<T>(vec: &mut Vec<T>)
where
    T: Ord + DeepModel,
    T::DeepModelTy: OrdLogic,
{
}
