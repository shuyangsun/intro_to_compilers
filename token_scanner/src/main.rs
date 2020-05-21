use maplit::{hashmap, hashset};
use token_scanner::{FiniteAutomaton, NFA};

fn main() {
    let nfa = NFA::from_map(
        0,
        hashset! {0, 2, 4},
        hashmap! {
            0 => hashmap!{
                Some('0')=> hashset!{1}
            },
            1 => hashmap!{
                Some('1')=> hashset!{2},
                None => hashset!{3}
            },
            2 => hashmap!{
                Some('0')=> hashset!{2},
                Some('1')=> hashset!{1}
            },
            3 => hashmap!{
                Some('0') => hashset!{4},
            }
        },
    );

    println!(
        "Finite automaton is deterministic: {}",
        nfa.is_deterministic()
    );

    let strings = vec!["", "0", "00", "001", "01", "010", "0100", "0101", "01011"];
    for string in strings {
        println!("{}: {}", string, nfa.accept(String::from(string).chars()));
    }
    nfa.export_graphviz_dot_file(String::from("/Users/shuyangsun/Desktop/nfa.dot"));
    let dfa = nfa.to_dfa();
    dfa.export_graphviz_dot_file(String::from("/Users/shuyangsun/Desktop/dfa.dot"));
    dfa.to_dfa()
        .export_graphviz_dot_file(String::from("/Users/shuyangsun/Desktop/dfa2.dot"));
}
