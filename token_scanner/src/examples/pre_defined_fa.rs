pub mod pre_defined_fa {
    use crate::DFA;
    use maplit::{hashmap, hashset};

    /// This function returns a DFA that checks is a binary number is divisible by 3.
    /// ```
    /// use token_scanner::FiniteAutomaton;
    /// use token_scanner::pre_defined_fa::bin_str_div_by_3;
    /// let dfa = bin_str_div_by_3();
    /// let numbers = vec![
    ///     "0",  // 0, true
    ///     "1",  // 1, false
    ///     "11",  // 3, true
    ///     "1001",  // 9, true
    ///     "10010",  // 18, true
    ///     "110100",  // 52, false
    /// ];
    /// for num in numbers {
    ///     println!("{}: {}", num, dfa.accept(String::from(num).chars()));
    /// }
    /// ```
    pub fn bin_str_div_by_3() -> DFA<char, u8> {
        DFA::from_map(
            0,
            hashset! { 0 },
            hashmap! {
                0 => hashmap!{
                    '0' => 0,
                    '1' => 1
                },
                1 => hashmap!{
                    '0' => 2,
                    '1' => 0
                },
                2 => hashmap!{
                    '0' => 1,
                    '1' => 2
                },
            },
        )
    }
}
