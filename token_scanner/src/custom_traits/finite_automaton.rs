use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, StateIdentifier, NFA};
use maplit::{hashmap, hashset};
use std::cmp::PartialEq;
use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, Iterator};
use std::vec::Vec;

pub type TransitionMap<T, U> = HashMap<U, HashMap<Alphabet<T>, HashSet<U>>>;

pub trait FiniteAutomaton<T, U>
where
    T: NoneEmptyAlphabet,
    U: StateIdentifier,
{
    fn from_formal(
        states: HashSet<U>,
        alphabets: HashSet<T>,
        start_state: U,
        accepted_states: HashSet<U>,
        transition_map: TransitionMap<T, U>,
    ) -> Self;
    fn states(&self) -> &HashSet<U>;
    fn alphabets(&self) -> &HashSet<T>;
    fn start_state(&self) -> U;
    fn accepted_states(&self) -> &HashSet<U>;
    fn transition(&self, state: U, alphabet: Alphabet<T>) -> HashSet<U>;

    fn epsilon_closure_states(&self, state: U) -> HashSet<U> {
        let mut result = hashset! { state.clone() };
        let mut stack = vec![state];
        while !stack.is_empty() {
            let cur_state = stack.pop().unwrap();
            for neighbor in self.transition(cur_state, None) {
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
            let cur_states = self.transition(start.clone(), Some(alphabet.clone()));
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

    fn is_deterministic(&self) -> bool {
        for state in self.states() {
            if self.epsilon_closure_states(state.clone()).len() > 1 {
                return false;
            }
            for alphabet in self.alphabets() {
                if self.transition(state.clone(), Some(alphabet.clone())).len() != 1 {
                    return false;
                }
            }
        }
        true
    }

    fn to_dfa_with_string_label(&self) -> NFA<T, String> {
        let mut stack = vec![CommunicativeHashSet::from(
            self.epsilon_closure_states(self.start_state()),
        )];
        let new_start_state = stack.first().unwrap().clone();
        let mut states_to_id = HashMap::new();
        let mut new_transition_map: HashMap<
            CommunicativeHashSet<U>,
            HashMap<T, CommunicativeHashSet<U>>,
        > = HashMap::new();
        while !stack.is_empty() {
            let cur_new_state = stack.pop().unwrap().clone();
            if !states_to_id.contains_key(&cur_new_state) {
                states_to_id.insert(
                    cur_new_state.clone(),
                    format!("{:?}", cur_new_state.hashset),
                );
            }
            for alphabet in self.alphabets() {
                let mut to_states = HashSet::new();
                for from_state in &cur_new_state.hashset {
                    to_states.extend(
                        self.epsilon_closure_transition(from_state.clone(), alphabet.clone()),
                    );
                }
                let to_states_set = CommunicativeHashSet::from(to_states);
                if !states_to_id.contains_key(&to_states_set) {
                    stack.push(to_states_set.clone());
                    states_to_id.insert(
                        cur_new_state.clone(),
                        format!("{:?}", cur_new_state.hashset),
                    );
                }
                match new_transition_map.get_mut(&cur_new_state) {
                    None => {
                        new_transition_map.insert(
                            cur_new_state.clone(),
                            hashmap! {alphabet.clone() => to_states_set },
                        );
                    }
                    Some(val) => {
                        val.insert(alphabet.clone(), to_states_set);
                    }
                };
            }
        }
        let new_accepted_states = HashSet::from_iter(
            states_to_id
                .keys()
                .filter(|ele| {
                    for state in &ele.hashset {
                        if self.accepted_states().contains(&state) {
                            return true;
                        }
                    }
                    false
                })
                .map(|ele| states_to_id.get(ele).unwrap().clone()),
        );
        let mut final_transition_map: TransitionMap<T, String> = HashMap::new();
        for (from_state, map) in new_transition_map.iter() {
            let mut new_map = HashMap::new();
            for (alphabet, to_state) in map.iter() {
                new_map.insert(
                    Some(alphabet.clone()),
                    hashset! {states_to_id.get(to_state).unwrap().clone()},
                );
            }
            final_transition_map.insert(states_to_id.get(from_state).unwrap().clone(), new_map);
        }
        let res = NFA::from_formal(
            HashSet::from_iter(states_to_id.iter().map(|(_, v)| v.clone())),
            self.alphabets().clone(),
            states_to_id.get(&new_start_state).unwrap().clone(),
            new_accepted_states,
            final_transition_map,
        );
        debug_assert!(res.is_deterministic());
        res
    }

    fn export_graphviz_dot_file(&self, output_file_path: String) {
        let mut output = File::create(output_file_path.clone()).unwrap();

        let nodes: Vec<U> = Vec::from_iter(self.states().clone());
        let mut nodes_to_idx = HashMap::new();
        for (idx, node) in nodes.iter().enumerate() {
            nodes_to_idx.insert(node, idx);
        }

        let mut edges = Vec::<(usize, usize, String)>::new();
        let epsilon_utf8 = "&#949;";
        let epsilon_string = String::from(epsilon_utf8);
        for (i, node) in nodes.iter().enumerate() {
            for next_state in self.epsilon_closure_states(node.clone()) {
                if *node != next_state {
                    edges.push((
                        i,
                        nodes_to_idx.get(&next_state).unwrap().clone(),
                        epsilon_string.clone(),
                    ));
                }
            }
            for alphabet in self.alphabets() {
                for next_state in self.transition(node.clone(), Some(alphabet.clone())) {
                    edges.push((
                        i,
                        nodes_to_idx.get(&next_state).unwrap().clone(),
                        alphabet.to_string().clone(),
                    ));
                }
            }
        }
        let nodes_string = nodes
            .iter()
            .map(|node| {
                node.to_string()
                    + (if self.accepted_states().contains(node) {
                        "*".to_string()
                    } else {
                        "".to_string()
                    })
                    .as_str()
            })
            .collect();
        let graph = Graph {
            nodes: nodes_string,
            edges,
        };

        dot::render(&graph, &mut output).unwrap();
        println!("GraphViz dot file rendered to: {}", output_file_path);
    }
}

type Nd = usize;
type Ed<'a> = &'a (usize, usize, String);
struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize, String)>,
}

impl<'a> dot::Labeller<'a, Nd, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("nfa").unwrap()
    }
    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("STATE{}", n)).unwrap()
    }
    fn node_label(&self, n: &Nd) -> dot::LabelText {
        dot::LabelText::LabelStr(("S".to_string() + self.nodes[*n].as_str()).into())
    }
    fn edge_label<'b>(&'b self, e: &Ed) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(e.2.clone().into())
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>> for Graph {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.nodes.len()).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges.iter().collect()
    }
    fn source(&self, e: &Ed) -> Nd {
        e.0.clone()
    }
    fn target(&self, e: &Ed) -> Nd {
        e.1.clone()
    }
}

#[derive(Clone)]
struct CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    hash: u64,
    hashset: HashSet<T>,
}

impl<T> CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    fn from(hashset: HashSet<T>) -> Self {
        let mut hash = 0;
        for ele in hashset.iter() {
            let mut hasher = DefaultHasher::new();
            ele.hash(&mut hasher);
            hash += hasher.finish();
        }
        Self { hash, hashset }
    }
}

impl<T> Hash for CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl<T> PartialEq for CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<T> Eq for CommunicativeHashSet<T> where T: Eq + Hash {}
