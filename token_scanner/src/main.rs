#[macro_use]
extern crate maplit;
use token_scanner::{FiniteAutomaton, NFA};

fn main() {
    let nfa = NFA::from_map(
        0,
        hashset! {0, 2},
        hashmap! {
            0 => hashmap!(
                '0'=> hashset!{1}
            ),
            1 => hashmap!(
                '1'=> hashset!{2}
            ),
            2 => hashmap!(
                '0'=> hashset!{2},
                '1'=> hashset!{1}
            )
        },
    );

    let strings = vec!["", "0", "01", "010", "0100", "0101", "01011"];
    for string in strings {
        println!("{}: {}", string, nfa.accept(String::from(string).chars()));
    }
}
