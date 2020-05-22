use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait NoneEmptyAlphabet: Eq + Hash + Clone + Display + Debug {}
pub type Alphabet<T> = Option<T>;

/// Any automaton state has an identifier, which will also be used as its label. The identifier has
/// to implement `StateIdentifier` trait. This trait is implemented for `usize`, `i8`, `i32`, `i64`,
/// `i128`, `char`, `&str` and `String` by default.
pub trait StateIdentifier: Clone + Eq + Hash + Display + Debug {}

macro_rules! impl_alphabet {
    // The pattern for a single `eval`
    ($type_name:ty) => {
        impl NoneEmptyAlphabet for $type_name {}
        impl StateIdentifier for $type_name {}
    };

    ($type_name:ty, $($type_names:ty),+) => {
        impl_alphabet! { $type_name }
        impl_alphabet! { $($type_names),+ }
    };
}

impl_alphabet!(usize, u8, i8, u32, i32, u64, i64, u128, i128, char, &str, String);
