use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait NoneEmptyAlphabet: Eq + Hash + Clone + Display + Debug {}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Alphabet<T>
where
    T: NoneEmptyAlphabet,
{
    Epsilon,
    Content(T),
}

#[macro_export]
macro_rules! eps {
    () => {{
        Alphabet::Epsilon
    }};
}

#[macro_export]
macro_rules! alp {
    ($val: expr) => {{
        Alphabet::Content($val)
    }};
}

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

impl_alphabet!(usize, i8, i32, i64, i128, char, &str, String);
