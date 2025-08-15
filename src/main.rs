use std::collections::BTreeSet;
mod tools;
use tools::{DFA, State, Symbol};

fn parsing() -> (
    BTreeSet<State>, //states
    BTreeSet<Symbol>, //alphabet
    BTreeSet<((State, Symbol), State)>, //delta_set
    State, //start
    BTreeSet<State>, //accept
) {
    use std::env::args;

    let argv: Vec<String> = args().collect();

    for ar in &argv {
        println!("{}", ar);
    }
    
    // Return tuple of arrays for DFA construction
    (
        ["q0", "q1"].into_iter().collect(), // states
        ['a', 'b'].into_iter().collect(), // alphabet
        [
            (("q0", 'a'), "q1"),
            (("q0", 'b'), "q0"),
            (("q1", 'a'), "q1"),
            (("q1", 'b'), "q0"),
        ].into_iter().collect(), // transitions
        "q0", // start state
        ["q1"].into_iter().collect(), // accept states
    )
}

fn main() {
    // Example DFA over {a,b} that accepts strings ending with 'a'
    // States: q0 (start, non-accept), q1 (accept)
    // δ:
    //   (q0, 'a') -> q1
    //   (q0, 'b') -> q0
    //   (q1, 'a') -> q1
    //   (q1, 'b') -> q0
    let (states, alphabet, delta_set, start, accept) = parsing();
    let dfa = tools::DFA::new(
        states, alphabet, delta_set, start, accept
    );

    let tests = ["", "a", "b", "ba", "abb", "abba", "abaa"];
    for s in tests {
        println!("{s:5} -> accepts? {}", dfa.accepts(s));
    }
}
