use maplit::{hashmap, hashset};
use token_scanner::{FiniteAutomaton, NFA};

fn main() {
    let nfa = NFA::from_map(
        0,
        hashset! { 2 },
        hashmap! {
            0 => hashmap!{
                Some('0')=> hashset!{1},
                Some('1')=> hashset!{5}
            },
            1 => hashmap!{
                Some('0')=> hashset!{6},
                Some('1')=> hashset!{2}
            },
            2 => hashmap!{
                Some('0')=> hashset!{0},
                Some('1')=> hashset!{2}
            },
            3 => hashmap!{
                Some('0')=> hashset!{2},
                Some('1')=> hashset!{6}
            },
            4 => hashmap!{
                Some('0')=> hashset!{7},
                Some('1')=> hashset!{5}
            },
            5 => hashmap!{
                Some('0')=> hashset!{2},
                Some('1')=> hashset!{6}
            },
            6 => hashmap!{
                Some('0')=> hashset!{6},
                Some('1')=> hashset!{4}
            },
            7 => hashmap!{
                Some('0')=> hashset!{6},
                Some('1')=> hashset!{2}
            },
        },
    );
    let dfa = nfa.to_dfa();
    let minimized = dfa.minimized();

    println!(
        "Finite automaton is deterministic: {}",
        nfa.is_deterministic()
    );

    println!(
        "Number of states in NFA, DFA, Minimized DFA: {}, {}, {}",
        nfa.states().len(),
        dfa.states().len(),
        minimized.states().len()
    );

    let strings = vec!["", "0", "00", "001", "01", "010", "0100", "0101", "01011"];
    for string in strings {
        println!("{}: {}", string, nfa.accept(String::from(string).chars()));
    }
    nfa.export_graphviz_dot_file(String::from("/Users/shuyangsun/Desktop/nfa.dot"));
    dfa.export_graphviz_dot_file(String::from("/Users/shuyangsun/Desktop/dfa.dot"));
    minimized.export_graphviz_dot_file(String::from("/Users/shuyangsun/Desktop/dfa_min.dot"));
}
