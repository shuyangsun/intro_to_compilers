use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait Empty: PartialEq + Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool {
        *self == Self::empty()
    }
}
pub trait Alphabet: Eq + Hash + Clone + Empty + Display + Debug {}
pub trait StateIdentifier: Clone + Eq + Hash + Display + Debug {}

impl StateIdentifier for String {}
impl StateIdentifier for &str {}
impl StateIdentifier for char {}

impl Empty for char {
    fn empty() -> Self {
        '\n'
    }
}

impl Alphabet for char {}

impl Empty for &str {
    fn empty() -> Self {
        ""
    }
}

impl Alphabet for &str {}

impl Empty for String {
    fn empty() -> Self {
        String::from("")
    }
}

impl Alphabet for String {}

macro_rules! impl_empty {
    // The pattern for a single `eval`
    ($type_name:ty) => {
        impl Empty for $type_name {
            fn empty() -> Self {
                Self::max_value()
            }
        }

        impl Alphabet for $type_name {}
        impl StateIdentifier for $type_name {}
    };

    ($type_name:ty, $($type_names:ty),+) => {
        impl_empty! { $type_name }
        impl_empty! { $($type_names),+ }
    };
}

impl_empty!(usize, i8, i32, i64, i128);
