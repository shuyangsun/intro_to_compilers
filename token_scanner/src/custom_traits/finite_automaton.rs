use crate::custom_traits::alphabet::NoneEmptyAlphabet;
use crate::{Alphabet, StateIdentifier};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::iter::{FromIterator, Iterator};
use std::vec::Vec;

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

    fn is_deterministic(&self) -> bool {
        for state in self.states() {
            if self.epsilon_closure_states(state.clone()).len() > 1 {
                return false;
            }
            for alphabet in self.alphabets() {
                if self
                    .transition(state.clone(), Alphabet::Content(alphabet.clone()))
                    .len()
                    != 1
                {
                    return false;
                }
            }
        }
        true
    }

    fn export_graphviz_dot_file(&self, output_file_path: String) {
        let mut output = File::create(output_file_path.clone()).unwrap();

        let nodes: Vec<U> = Vec::from_iter(self.states().clone());
        let mut nodes_to_idx = HashMap::new();
        for (idx, node) in nodes.iter().enumerate() {
            nodes_to_idx.insert(node, idx);
        }

        let mut edges = Vec::<(usize, usize, String)>::new();
        let epsilon_string = String::from("$");
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
                for next_state in self.transition(node.clone(), Alphabet::Content(alphabet.clone()))
                {
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
