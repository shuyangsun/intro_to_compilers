use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, StateIdentifier};
use std::collections::HashSet;
use std::iter::Iterator;

pub trait FiniteAutomaton<T, U>
where
    T: NoneEmptyAlphabet,
    U: StateIdentifier,
{
    fn states(&self) -> &HashSet<U>;
    fn alphabets(&self) -> &HashSet<T>;
    fn start_state(&self) -> U;
    fn accepted_states(&self) -> &HashSet<U>;
    fn transition(&self, state: U, alphabet: Alphabet<T>) -> HashSet<U>;

    fn epsilon_closure_states(&self, state: U) -> HashSet<U> {
        let mut result = HashSet::new();
        result.insert(state.clone());
        let mut stack = vec![state];
        while !stack.is_empty() {
            let cur_state = stack.pop().unwrap();
            for neighbor in self.transition(cur_state, Alphabet::Epsilon) {
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
        let start_states = self.epsilon_closure_states(state.clone());
        let mut neighbors = HashSet::new();
        for start in start_states.iter() {
            let cur_states = self.transition(start.clone(), Alphabet::Content(alphabet.clone()));
            for cur in cur_states {
                neighbors.extend(self.epsilon_closure_states(cur));
            }
        }
        neighbors
    }

    fn accept<S>(&self, content: S) -> bool
    where
        S: Iterator<Item = T>,
    {
        let mut cur_states = self.epsilon_closure_states(self.start_state());
        for alphabet in content.into_iter() {
            let mut next_states = HashSet::new();
            for state in cur_states.iter() {
                next_states
                    .extend(self.epsilon_closure_transition(state.clone(), alphabet.clone()));
            }
            cur_states = next_states;
        }
        for state in self.accepted_states() {
            if cur_states.contains(state) {
                return true;
            }
        }
        false
    }
}
