use std::collections::{HashMap, HashSet};
use token_scanner::{FiniteAutomaton, NFA};

macro_rules! set {
    ($arr: expr) => {{
        $arr.iter().cloned().collect()
    }};
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn main() {
    let alphabets = set!(['0', '1']);
    let states = set!([0, 1, 2]);
    let accepted_states = set!([0, 2]);
    let transition_map: HashMap<i32, HashMap<char, HashSet<i32>>> = map!(
        0 => map!(
            '0' => set!([1])
        ),
        1 => map!(
            '1' => set!([2])
        ),
        2 => map!(
            '0' => set!([2]),
            '1' => set!([1])
        )
    );

    let nfa = NFA::from(states, alphabets, 0, accepted_states, transition_map);
    let strings = vec!["", "0", "01", "010", "0100", "0101"];

    for string in strings {
        println!("{}: {}", string, nfa.accept(String::from(string).chars()));
    }
}
