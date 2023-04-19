//! External specifications for `std::option::Option`.

use prusti_contracts::*;

#[extern_spec]
impl<T> Option<T> {
    #[pure]
    #[ensures(if let Some(_) = self { false } else { true })]
    fn is_none(&self) -> bool;

    #[pure]
    #[ensures(result == !self.is_none())]
    fn is_some(&self) -> bool;
}