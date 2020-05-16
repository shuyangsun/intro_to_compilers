pub mod automaton;
pub mod custom_traits;

pub use automaton::nfa::NFA;
pub use custom_traits::alphabet::{Alphabet, StateIdentifier};
pub use custom_traits::finite_automaton::FiniteAutomaton;
