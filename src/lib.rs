#![feature(
    fn_traits,
    unboxed_closures,
    anonymous_lifetime_in_impl_trait,
    const_trait_impl,
    const_option,
    const_mut_refs,
    negative_impls,
    result_option_inspect,
    int_roundings
)]

pub mod ability;
pub mod creature;
pub mod dice;
pub mod identity;

pub use identity::Identity;
