use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, CommunicativeHashSet, DFATransitionMap, FiniteAutomaton, StateIdentifier};
use maplit::hashset;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct DFA<T, U>
where
    T: NoneEmptyAlphabet,
    U: StateIdentifier,
{
    states: HashSet<U>,
    alphabets: HashSet<T>,
    transition_map: DFATransitionMap<T, U>,
    start_state: U,
    accepted_states: HashSet<U>,
}

impl<T, U> FiniteAutomaton<T, U> for DFA<T, U>
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
        if alphabet.is_none() {
            HashSet::new()
        } else {
            hashset! {
            self.transition_map.get(&state).unwrap().get(&alphabet.unwrap()).unwrap().clone()
            }
        }
    }

    fn is_deterministic(&self) -> bool {
        true
    }

    fn to_dfa(&self) -> DFA<T, CommunicativeHashSet<U>> {
        let states: HashSet<CommunicativeHashSet<U>> = HashSet::from_iter(
            self.states
                .iter()
                .map(|ele| CommunicativeHashSet::from(hashset! {ele.clone()})),
        );
        let start_state = CommunicativeHashSet::from(hashset! {self.start_state.clone()});
        let accepted_states: HashSet<CommunicativeHashSet<U>> = HashSet::from_iter(
            self.accepted_states
                .iter()
                .map(|ele| CommunicativeHashSet::from(hashset! {ele.clone()})),
        );
        let transition_map: DFATransitionMap<T, CommunicativeHashSet<U>> =
            HashMap::from_iter(self.transition_map.iter().map(|(from_state, map)| {
                (
                    CommunicativeHashSet::from(hashset! {from_state.clone()}),
                    HashMap::from_iter(map.iter().map(|(alphabet, to_state)| {
                        (
                            alphabet.clone(),
                            CommunicativeHashSet::from(hashset! {to_state.clone()}),
                        )
                    })),
                )
            }));
        DFA::from_formal(
            states,
            self.alphabets.clone(),
            start_state,
            accepted_states,
            transition_map,
        )
    }
}

impl<T, U> DFA<T, U>
where
    T: NoneEmptyAlphabet,
    U: StateIdentifier,
{
    pub fn from_formal(
        states: HashSet<U>,
        alphabets: HashSet<T>,
        start_state: U,
        accepted_states: HashSet<U>,
        transition_map: DFATransitionMap<T, U>,
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
        if states.len() != transition_map.len() {
            panic!(
                "Missing transition in transition map for states {:?}.",
                states.difference(&HashSet::from_iter(
                    transition_map.keys().map(|ele| { ele.clone() })
                ))
            )
        }
        for (from_state, map) in transition_map.iter() {
            if map.len() != alphabets.len() {
                panic!(
                    "Missing transition in transition map for alphabets {:?} from state {}.",
                    alphabets
                        .difference(&HashSet::from_iter(map.keys().map(|ele| { ele.clone() }))),
                    from_state
                )
            }
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
        transition_map: DFATransitionMap<T, U>,
    ) -> Self {
        let mut states = hashset! {start_state.clone()};
        let mut alphabets = HashSet::new();
        for (state, alphabet_map) in transition_map.iter() {
            states.insert(state.clone());
            for (alphabet, dst_states) in alphabet_map.iter() {
                alphabets.insert(alphabet.clone());
                states.insert(dst_states.clone());
            }
        }
        Self::from_formal(
            states,
            alphabets,
            start_state,
            accepted_states,
            transition_map,
        )
    }
}
