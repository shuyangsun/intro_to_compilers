use crate::{Alphabet, FiniteAutomaton, StateIdentifier};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

type TransitionMap<T, U> = HashMap<U, HashMap<T, HashSet<U>>>;

pub struct NFA<T, U>
where
    T: Alphabet,
    U: StateIdentifier,
{
    states: HashSet<U>,
    alphabets: HashSet<T>,
    transition_map: TransitionMap<T, U>,
    start_state: U,
    accepted_states: HashSet<U>,
}

impl<T, U> FiniteAutomaton<T, U> for NFA<T, U>
where
    T: Alphabet,
    U: StateIdentifier,
{
    fn states(&self) -> &HashSet<U, RandomState> {
        &self.states
    }

    fn alphabets(&self) -> &HashSet<T, RandomState> {
        &self.alphabets
    }

    fn start_state(&self) -> U {
        self.start_state.clone()
    }
    fn accepted_states(&self) -> &HashSet<U, RandomState> {
        &self.accepted_states
    }

    fn transition(&self, state: U, alphabet: T) -> HashSet<U> {
        let alphabet_map = self.transition_map.get(&state);
        match alphabet_map {
            None => HashSet::new(),
            Some(next_states_map) => match next_states_map.get(&alphabet) {
                None => HashSet::new(),
                Some(val) => val.clone(),
            },
        }
    }
}

impl<T, U> NFA<T, U>
where
    T: Alphabet,
    U: StateIdentifier,
{
    pub fn from(
        states: HashSet<U>,
        alphabets: HashSet<T>,
        start_state: U,
        accepted_states: HashSet<U>,
        transition_map: TransitionMap<T, U>,
    ) -> Self {
        Self {
            states,
            alphabets,
            start_state,
            accepted_states,
            transition_map,
        }
    }

    pub fn from_map(
        start_state: U,
        accepted_states: HashSet<U>,
        transition_map: HashMap<U, HashMap<T, HashSet<U>>>,
    ) -> Self {
        let mut states = HashSet::new();
        let mut alphabets = HashSet::new();
        for (state, alphabet_map) in transition_map.iter() {
            states.insert(state.clone());
            for (alphabet, dst_states) in alphabet_map.iter() {
                alphabets.insert(alphabet.clone());
                states.extend(dst_states.clone());
            }
        }
        Self::from(
            states,
            alphabets,
            start_state,
            accepted_states,
            transition_map,
        )
    }
}
