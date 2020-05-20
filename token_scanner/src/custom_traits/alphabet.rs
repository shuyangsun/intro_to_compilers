use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;

pub trait NoneEmptyAlphabet: Eq + Hash + Clone + Display + Debug {}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Alphabet<T>
where
    T: NoneEmptyAlphabet,
{
    Epsilon,
    Content(T),
}

fn alphabet_to_string<T>(alphabet: &Alphabet<T>) -> String
where
    T: NoneEmptyAlphabet,
{
    match alphabet {
        Alphabet::Epsilon => String::from("ϵ"),
        Alphabet::Content(val) => val.to_string(),
    }
}

impl<T> Display for Alphabet<T>
where
    T: NoneEmptyAlphabet,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", alphabet_to_string(self))
    }
}

impl<T> Debug for Alphabet<T>
where
    T: NoneEmptyAlphabet,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", alphabet_to_string(self))
    }
}

/// Use `eps!` macro to create empty alphabet. Since the Alphabet enum has to be associated with a
/// generic type, explicit type annoation is sometimes needed. However, in most usecased the type
/// can be inferred by the compiler.
/// ```
/// use maplit::hashset;
/// use token_scanner::{Alphabet, eps, alp};
/// let empty: Alphabet<char> = eps!();  // Explicit type annotation.
/// let alphabets = hashset!{alp!('a'), eps!()};  // Inferred by compiler.
/// ```
#[macro_export]
macro_rules! eps {
    () => {{
        Alphabet::Epsilon
    }};
}

/// Use `alp!` macro to create alphabets.
/// ```
/// use token_scanner::{alp, Alphabet};
/// let a = alp!('a');
/// ```
#[macro_export]
macro_rules! alp {
    ($val: expr) => {{
        Alphabet::Content($val)
    }};
}

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

impl_alphabet!(usize, i8, i32, i64, i128, char, &str, String);
