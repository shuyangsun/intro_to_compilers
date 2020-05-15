use std::collections::HashMap;
use token_scanner::{FiniteAutomaton, NFA};

macro_rules! set {
    ($arr: expr) => {{
        $arr.iter().cloned().collect()
    }};
}

fn main() {
    let alphabets = set!(['0', '1']);
    let states = set!([0, 1, 2]);
    let accepted_states = set!([0, 2]);

    let mut zero_map = HashMap::new();
    zero_map.insert('0', set!([1]));

    let mut one_map = HashMap::new();
    one_map.insert('1', set!([2]));

    let mut two_map = HashMap::new();
    two_map.insert('0', set!([2]));
    two_map.insert('1', set!([1]));

    let mut transition_map = HashMap::new();
    transition_map.insert(0, zero_map);
    transition_map.insert(1, one_map);
    transition_map.insert(2, two_map);

    let nfa = NFA::from(states, alphabets, 0, accepted_states, transition_map);
    let strings = vec!["", "0", "01", "010", "0100", "0101"];

    for string in strings {
        println!("{}: {}", string, nfa.accept(String::from(string).chars()));
    }
}
