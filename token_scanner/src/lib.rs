pub mod automaton;
pub mod custom_traits;

pub use automaton::nfa::NFA;
pub use custom_traits::alphabet::Alphabet;
pub use custom_traits::alphabet::Empty;
pub use custom_traits::alphabet::StateIdentifier;
pub use custom_traits::finite_automaton::FiniteAutomaton;
