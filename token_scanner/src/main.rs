use std::borrow::{Borrow, BorrowMut};
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

trait Empty: PartialEq + Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool {
        *self == Self::empty()
    }
}
trait Alphabet: Eq + Hash + Copy + Empty {}

trait NondeterministicFiniteAutomatonState<T>: Eq + Hash + Clone
where
    T: Alphabet,
{

    fn is_final(&self) -> bool;
    fn label(&self) -> String;

    fn transition(&self, alphabet: T) -> HashSet<Self>
    where
        T: Alphabet;

    fn does_accept_empty_alphabet(&self) -> bool {
        !self.transition(T::empty()).is_empty()
    }
}

trait NondeterministicFiniteAutomaton<T, U>: Sized
where
    T: Alphabet,
    U: NondeterministicFiniteAutomatonState<T>,
{
    fn from(states: HashSet<U>) -> Self;
    fn states(&self) -> &HashSet<U>;
    fn states_mut(&mut self) -> &mut HashSet<U>;

    fn new() -> Self {
        Self::from(HashSet::<U>::new())
    }

    fn is_deterministic(&self) -> bool {
        unimplemented!()
    }
    fn add_state(&mut self, state: U) {
        self.states_mut().insert(state);
    }
    fn remove_state(&mut self, state_label: String) {
        let state_mut = self.states_mut();
        let mut state_to_remove: Option<U> = None;
        for state in state_mut.iter() {
            if state.label() == state_label {
                state_to_remove = Some(state.clone());
                break;
            }
        }
        match state_to_remove {
            Some(s) => state_mut.remove(&s),
            None => panic!("Cannot find state with label {}", state_label),
        };
    }
}

impl Empty for &str {
    fn empty() -> Self {
        ""
    }
}

impl Alphabet for &str {}

struct NFAState<T>
where
    T: Alphabet,
{
    is_final_state: bool,
    name: String,
    transition_map: HashMap<T, HashSet<Self>>,
}

// TODO: PartialEq, Eq and Hash could be implemented with specialization once the feature is stable.
impl<T> PartialEq for NFAState<T>
where
    T: Alphabet,
{
    fn eq(&self, other: &Self) -> bool {
        self.label() == other.label()
    }
}

impl<T> Eq for NFAState<T> where T: Alphabet {}

impl<T> Hash for NFAState<T>
where
    T: Alphabet,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label().hash(state)
    }
}

impl<T> Clone for NFAState<T>
where
    T: Alphabet,
{
    fn clone(&self) -> Self {
        Self {
            is_final_state: self.is_final_state,
            name: self.name.clone(),
            transition_map: self.transition_map.clone(),
        }
    }
}

impl<T> NondeterministicFiniteAutomatonState<T> for NFAState<T>
where
    T: Alphabet,
{
    fn is_final(&self) -> bool {
        self.is_final_state
    }

    fn label(&self) -> String {
        self.name.clone()
    }

    fn transition(&self, alphabet: T) -> HashSet<Self> {
        self.transition_map[&alphabet].clone()
    }
}

struct NFA<T> {
    states: HashSet<T>,
}

impl<T, U> NondeterministicFiniteAutomaton<T, U> for NFA<U>
where
    T: Alphabet,
    U: NondeterministicFiniteAutomatonState<T>,
{
    fn from(states: HashSet<U>) -> Self {
        Self { states }
    }

    fn states(&self) -> &HashSet<U> {
        self.states.borrow()
    }

    fn states_mut(&mut self) -> &mut HashSet<U> {
        self.states.borrow_mut()
    }
}

fn main() {
    let mut start_state = NFAState::<&str>::
    let s0 = NFAState::<&str> {
        is_final_state: false,
        name: String::from("S0"),
        transition_map: HashSet::<&str, NFAState<&str>>,
    };
    asdf.add_state(s0);
}
