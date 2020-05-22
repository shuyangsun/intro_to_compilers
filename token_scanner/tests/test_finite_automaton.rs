#[cfg(test)]
mod tests {
    use maplit::{hashmap, hashset};
    use rand::{thread_rng, Rng};
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
        let dfa = nfa.to_dfa();
        let dfa_min = dfa.minimized();
        assert!(!nfa.is_deterministic());
        assert!(dfa.is_deterministic());
        assert!(dfa_min.is_deterministic());
        assert!(dfa_min.states().len() <= dfa.states().len());

        let accept_strings = vec!["", "00", "01", "010", "0100", "01011"];
        let reject_strings = vec!["0", "001", "0101"];

        for string in accept_strings.iter() {
            assert!(nfa.accept(string.to_string().chars()));
            assert!(dfa.accept(string.to_string().chars()));
            assert!(dfa_min.accept(string.to_string().chars()));
        }

        for string in reject_strings.iter() {
            assert!(!nfa.accept(string.to_string().chars()));
            assert!(!dfa.accept(string.to_string().chars()));
            assert!(!dfa_min.accept(string.to_string().chars()));
        }
    }

    #[test]
    fn nfa_2() {
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
        let dfa_min = dfa.minimized();

        assert!(nfa.is_deterministic());
        assert!(dfa.is_deterministic());
        assert!(dfa_min.is_deterministic());
        assert!(dfa_min.states().len() <= dfa.states().len());

        for length in 0..100usize {
            let string = gen_random_binary_string(length).to_string();
            let accepted = nfa.accept(string.chars());
            assert_eq!(dfa.accept(string.chars()), accepted);
            assert_eq!(dfa_min.accept(string.chars()), accepted);
        }
    }

    fn gen_random_binary_string(length: usize) -> String {
        let mut rng = thread_rng();
        let mut res = String::with_capacity(length);
        for _ in 0..length {
            res.push(if rng.gen_bool(0.5) { '1' } else { '0' });
        }
        res
    }
}
