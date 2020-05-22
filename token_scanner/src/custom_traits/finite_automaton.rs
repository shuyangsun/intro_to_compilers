use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, StateIdentifier, DFA};
use maplit::{hashmap, hashset};
use std::cmp::PartialEq;
use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Result};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, Iterator};
use std::vec::Vec;

pub type NFATransitionMap<T, U> = HashMap<U, HashMap<Alphabet<T>, HashSet<U>>>;
pub type DFATransitionMap<T, U> = HashMap<U, HashMap<T, U>>;

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

    fn to_dfa(&self) -> DFA<T, CommunicativeHashSet<U>> {
        let mut stack = vec![CommunicativeHashSet::from(
            self.epsilon_closure_states(self.start_state()),
        )];
        let new_start_state = stack.first().unwrap().clone();
        let mut new_states = HashSet::new();
        let mut new_transition_map: HashMap<
            CommunicativeHashSet<U>,
            HashMap<T, CommunicativeHashSet<U>>,
        > = HashMap::new();
        while !stack.is_empty() {
            let cur_new_state = stack.pop().unwrap().clone();
            if !new_states.contains(&cur_new_state) {
                new_states.insert(cur_new_state.clone());
            }
            for alphabet in self.alphabets() {
                let mut to_states = HashSet::new();
                for from_state in &cur_new_state.hashset {
                    to_states.extend(
                        self.epsilon_closure_transition(from_state.clone(), alphabet.clone()),
                    );
                }
                let to_states_set = CommunicativeHashSet::from(to_states);
                if !new_states.contains(&to_states_set) {
                    stack.push(to_states_set.clone());
                    new_states.insert(cur_new_state.clone());
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
            new_states
                .iter()
                .filter(|ele| {
                    for state in &ele.hashset {
                        if self.accepted_states().contains(state) {
                            return true;
                        }
                    }
                    false
                })
                .map(|ele| ele.clone()),
        );
        let mut final_transition_map: DFATransitionMap<T, CommunicativeHashSet<U>> = HashMap::new();
        for (from_state, map) in new_transition_map.iter() {
            let mut new_map = HashMap::new();
            for (alphabet, to_state) in map.iter() {
                new_map.insert(alphabet.clone(), to_state.clone());
            }
            final_transition_map.insert(from_state.clone(), new_map);
        }
        let res = DFA::from_formal(
            new_states,
            self.alphabets().clone(),
            new_start_state,
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
pub struct CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    hash: u128,
    pub hashset: HashSet<T>,
}

impl<T> CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    pub fn from(hashset: HashSet<T>) -> Self {
        let mut hash: u128 = 0;
        for ele in hashset.iter() {
            let mut hasher = DefaultHasher::new();
            ele.hash(&mut hasher);
            hash += hasher.finish() as u128;
        }
        Self { hash, hashset }
    }

    pub fn len(&self) -> usize {
        self.hashset.len()
    }

    pub fn contains(&self, val: &T) -> bool {
        self.hashset.contains(val)
    }

    pub fn insert(&mut self, val: T) -> bool {
        if self.contains(&val) {
            return true;
        }
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        self.hash += hasher.finish() as u128;
        self.hashset.insert(val)
    }
}

impl<T> Hash for CommunicativeHashSet<T>
where
    T: Eq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u128(self.hash);
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

impl<T> Display for CommunicativeHashSet<T>
where
    T: Eq + Hash + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.hashset.len() {
            0 => write!(f, "&empty;"),
            1 => write!(f, "{:?}", self.hashset.iter().next().unwrap()),
            _ => write!(f, "{:?}", self.hashset),
        }
    }
}

impl<T> Debug for CommunicativeHashSet<T>
where
    T: Eq + Hash + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T> Eq for CommunicativeHashSet<T> where T: Eq + Hash {}

impl<T> StateIdentifier for CommunicativeHashSet<T> where T: StateIdentifier {}
