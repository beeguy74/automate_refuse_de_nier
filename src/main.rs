use std::{array, collections::BTreeSet};
mod tools;
use tools::{DFA, State, Symbol, parsing, run};


fn main() {
    // Example DFA over {a,b} that accepts strings ending with 'a'
    // States: q0 (start, non-accept), q1 (accept)
    // δ:
    //   (q0, 'a') -> q1
    //   (q0, 'b') -> q0
    //   (q1, 'a') -> q1
    //   (q1, 'b') -> q0
    // let (states, alphabet, delta_set, start, accept) = parsing();
    // let dfa = DFA::new(
    //     states, alphabet, delta_set, start, accept
    // );

    // let tests = ["", "a", "w", "t", "wa", "add", "asda", "auia"];
    // for s in tests {
    //     println!("{s:5} -> accepts? {}", dfa.accepts(s));
    // }
    run();
}
