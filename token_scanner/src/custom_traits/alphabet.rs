use std::hash::Hash;

pub trait Empty: PartialEq + Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool {
        *self == Self::empty()
    }
}

pub trait Alphabet: Eq + Hash + Clone + Empty {}

impl Empty for &str {
    fn empty() -> Self {
        ""
    }
}

impl Empty for String {
    fn empty() -> Self {
        String::from("")
    }
}

macro_rules! impl_empty {
    // The pattern for a single `eval`
    ($type_name:ty) => {
        impl Empty for $type_name {
            fn empty() -> Self {
                Self::max_value()
            }
        }
    };

    ($type_name:ty, $($type_names:ty),+) => {
        impl_empty! { $type_name }
        impl_empty! { $($type_names),+ }
    };
}

impl_empty!(usize, i8, i32, i64, i128);
