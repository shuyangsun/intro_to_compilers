use crate::Alphabet;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::Iterator;

pub trait StateIdentifier: Clone + Eq + Hash + Display {}

pub trait FiniteAutomaton<T, U>
where
    T: Alphabet,
    U: StateIdentifier,
{
    fn states(&self) -> &HashSet<U>;
    fn alphabets(&self) -> &HashSet<T>;
    fn transition_func(&self) -> &fn(U, T) -> HashSet<U>;
    fn start_state(&self) -> U;
    fn accepted_states(&self) -> &HashSet<U>;

    fn transition(&self, state: U, alphabet: T) -> HashSet<U> {
        self.transition_func()(state, alphabet)
    }

    fn epsilon_closure_states(&self, state: U) -> HashSet<U> {
        let mut result = HashSet::<U>::new();
        result.insert(state.clone());
        let mut stack = vec![state.clone()];
        while !stack.is_empty() {
            let cur_state = stack.pop().unwrap();
            for neighbor in self.transition(cur_state.clone(), T::empty()) {
                if result.contains(&neighbor) {
                    continue;
                }
                result.insert(neighbor.clone());
                stack.push(neighbor);
            }
        }
        result
    }

    fn epsilon_closure_transition(&self, state: U, alphabet: T) -> HashSet<U> {
        let mut start_states = self.epsilon_closure_states(state.clone());
        start_states.extend(self.transition(state, alphabet));
        let mut neighbors = HashSet::<U>::new();
        for cur_state in start_states.iter() {
            neighbors.extend(self.epsilon_closure_states(cur_state.clone()));
        }
        start_states.extend(neighbors);
        start_states
    }

    fn accept<S>(&self, content: S) -> bool
    where
        S: Iterator<Item = T>,
    {
        let mut cur_states = self.epsilon_closure_states(self.start_state());
        for alphabet in content.into_iter() {
            let mut next_states = HashSet::<U>::new();
            for state in cur_states.iter() {
                next_states
                    .extend(self.epsilon_closure_transition(state.clone(), alphabet.clone()));
            }
            cur_states = next_states;
        }
        cur_states
            .intersection(self.accepted_states())
            .collect::<Vec<&U>>()
            .len()
            > 0
    }
}
