#[cfg(test)]
mod tests {
    use maplit::{hashmap, hashset};
    use std::collections::{HashMap, HashSet};
    use token_scanner::{Alphabet, FiniteAutomaton, NFA};

    #[test]
    fn empty_nfa() {
        let empty = NFA::from_map(
            0i32,
            HashSet::<i32>::new(),
            HashMap::<i32, HashMap<Alphabet<char>, HashSet<i32>>>::new(),
        );

        let tests = ["", "0", "01"];
        for ele in tests.iter() {
            assert!(
                !empty.accept(ele.to_string().chars()),
                "NFA should not accept string \"{}\"",
                ele
            );
        }
    }

    #[test]
    fn nfa_empty_string() {
        let nfa = NFA::from_map(
            0,
            hashset! {0},
            HashMap::<i32, HashMap<Alphabet<char>, HashSet<i32>>>::new(),
        );

        let tests = ["0", "01"];
        for ele in tests.iter() {
            assert!(
                !nfa.accept(ele.to_string().chars()),
                "NFA should not accept string \"{}\"",
                ele
            );
        }
        assert!(
            nfa.accept("".to_string().chars()),
            "NFA should accept empty string.",
        );
    }

    #[test]
    #[should_panic]
    fn nfa_invalid() {
        let _ = NFA::from_formal(
            hashset! {0, 1},
            hashset! {'0', '1'},
            2,
            hashset! {1},
            HashMap::<i32, HashMap<Alphabet<char>, HashSet<i32>>>::new(),
        );
    }

    #[test]
    fn nfa_1() {
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
        assert!(!nfa.is_deterministic());
        let dfa = nfa.to_dfa_with_string_label();
        assert!(dfa.is_deterministic());

        let accept_strings = vec!["", "00", "01", "010", "0100", "01011"];
        let reject_strings = vec!["0", "001", "0101"];

        for string in accept_strings.iter() {
            assert!(nfa.accept(string.to_string().chars()));
            assert!(dfa.accept(string.to_string().chars()));
        }

        for string in reject_strings.iter() {
            assert!(!nfa.accept(string.to_string().chars()));
            assert!(!dfa.accept(string.to_string().chars()));
        }
    }
}
