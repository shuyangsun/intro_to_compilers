use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, FiniteAutomaton, StateIdentifier};
use maplit::hashset;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

type TransitionMap<T, U> = HashMap<U, HashMap<Alphabet<T>, HashSet<U>>>;

pub struct NFA<T, U>
where
    T: NoneEmptyAlphabet,
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
    T: NoneEmptyAlphabet,
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

    fn transition(&self, state: U, alphabet: Alphabet<T>) -> HashSet<U> {
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
    T: NoneEmptyAlphabet,
    U: StateIdentifier,
{
    pub fn from(
        states: HashSet<U>,
        alphabets: HashSet<T>,
        start_state: U,
        accepted_states: HashSet<U>,
        transition_map: TransitionMap<T, U>,
    ) -> Self {
        if !accepted_states.is_subset(&states) {
            panic!(
                "Cannot initialize NFA with accepted states {:?} not in all states {:?}.",
                accepted_states.difference(&states),
                states
            )
        }
        if !states.contains(&start_state) {
            panic!(
                "Cannot initialize NFA with start state {} not in all states {:?}.",
                start_state, states
            )
        }
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
        transition_map: TransitionMap<T, U>,
    ) -> Self {
        let mut states = hashset! {start_state.clone()};
        let mut alphabets = HashSet::new();
        for (state, alphabet_map) in transition_map.iter() {
            states.insert(state.clone());
            for (alphabet, dst_states) in alphabet_map.iter() {
                match alphabet {
                    Alphabet::<T>::Epsilon => false,
                    Alphabet::<T>::Content(val) => alphabets.insert(val.clone()),
                };
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
