use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, CommunicativeHashSet, DFATransitionMap, FiniteAutomaton, StateIdentifier};
use maplit::hashset;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::mem::swap;

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

    pub fn minimized(&self) -> DFA<T, CommunicativeHashSet<U>> {
        let non_final_states: HashSet<U> = HashSet::from_iter(
            self.states
                .difference(self.accepted_states())
                .map(|ele| ele.clone()),
        );
        let mut last_equivalence = HashSet::new();
        let mut cur_equivalence = hashset! {
            CommunicativeHashSet::from(non_final_states),
            CommunicativeHashSet::from(self.accepted_states.clone()),
        };
        while last_equivalence != cur_equivalence {
            swap(&mut last_equivalence, &mut cur_equivalence);
            cur_equivalence.clear();
            for cur_last_equiv_set in last_equivalence.iter() {
                let mut cur_new_sets = vec![];
                if cur_last_equiv_set.len() <= 1 {
                    cur_equivalence.insert(cur_last_equiv_set.clone());
                    continue;
                }
                for s1 in cur_last_equiv_set.hashset.iter() {
                    if cur_new_sets.len() <= 0 {
                        cur_new_sets.push(CommunicativeHashSet::from(hashset! {s1.clone()}));
                        continue;
                    }
                    let mut did_insert = false;
                    for i in 0..cur_new_sets.len() {
                        let working_set = cur_new_sets.get_mut(i).unwrap();
                        let s2: U = working_set.hashset.iter().next().unwrap().clone();
                        let same_set = is_same_equivalent_set(self, s1, &s2, &last_equivalence);
                        if same_set {
                            working_set.insert(s1.clone());
                            did_insert = true;
                        }
                    }
                    if !did_insert {
                        cur_new_sets.push(CommunicativeHashSet::from(hashset! {s1.clone()}));
                    }
                }
                cur_equivalence.extend(cur_new_sets);
            }
        }
        last_equivalence.clear();
        let mut old_state_to_new_state = HashMap::new();
        for new_state in cur_equivalence.iter() {
            for old_state in new_state.hashset.iter() {
                if !old_state_to_new_state.contains_key(old_state) {
                    old_state_to_new_state.insert(old_state, new_state);
                }
            }
        }

        let mut new_start_state = None;
        let mut new_accepted_states = HashSet::new();
        for new_state in cur_equivalence.iter() {
            if new_state.contains(&self.start_state) {
                new_start_state = Some(new_state.clone());
            }
            for cur in self.accepted_states.iter() {
                if new_state.contains(cur) {
                    new_accepted_states.insert(new_state.clone());
                    break;
                }
            }
        }
        let mut new_transition_map = HashMap::new();
        for state in cur_equivalence.iter() {
            let mut res_map = HashMap::new();
            let any_old_state = state.hashset.iter().next().unwrap();
            for alphabet in self.alphabets.iter() {
                let next_old_state = self
                    .transition_map
                    .get(any_old_state)
                    .unwrap()
                    .get(alphabet)
                    .unwrap();
                res_map.insert(
                    alphabet.clone(),
                    (*old_state_to_new_state.get(next_old_state).unwrap()).clone(),
                );
            }
            new_transition_map.insert(state.clone(), res_map);
        }
        DFA::from_formal(
            cur_equivalence,
            self.alphabets.clone(),
            new_start_state.unwrap(),
            new_accepted_states,
            new_transition_map,
        )
    }
}

fn is_same_equivalent_set<T, U>(
    dfa: &DFA<T, U>,
    s1: &U,
    s2: &U,
    last_equivalence: &HashSet<CommunicativeHashSet<U>>,
) -> bool
where
    T: NoneEmptyAlphabet,
    U: StateIdentifier,
{
    let mut same_set = true;
    for alphabet in dfa.alphabets.iter() {
        let dst_1 = dfa.transition_map.get(s1).unwrap().get(alphabet).unwrap();
        let dst_2 = dfa.transition_map.get(s2).unwrap().get(alphabet).unwrap();
        for ele in last_equivalence.iter() {
            if !ele.contains(dst_1) && ele.contains(dst_2)
                || ele.contains(dst_1) && !ele.contains(dst_2)
            {
                same_set = false;
                break;
            }
        }
        if !same_set {
            break;
        }
    }
    same_set
}
