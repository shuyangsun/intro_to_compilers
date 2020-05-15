#[macro_use]
extern crate maplit;
#[macro_use]
extern crate token_scanner;
use token_scanner::{Alphabet, FiniteAutomaton, NFA};

fn main() {
    let nfa = NFA::from_map(
        0,
        hashset! {0, 2, 4},
        hashmap! {
            0 => hashmap!{
                alp!('0')=> hashset!{1}
            },
            1 => hashmap!{
                alp!('1')=> hashset!{2},
                eps!() => hashset!{3}
            },
            2 => hashmap!{
                alp!('0')=> hashset!{2},
                alp!('1')=> hashset!{1}
            },
            3 => hashmap!{
                alp!('0') => hashset!{4},
            }
        },
    );

    let strings = vec!["", "0", "00", "001", "01", "010", "0100", "0101", "01011"];
    for string in strings {
        println!("{}: {}", string, nfa.accept(String::from(string).chars()));
    }
}
