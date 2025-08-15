mod tools;

fn main() {
    // Example DFA over {a,b} that accepts strings ending with 'a'
    // States: q0 (start, non-accept), q1 (accept)
    // δ:
    //   (q0, 'a') -> q1
    //   (q0, 'b') -> q0
    //   (q1, 'a') -> q1
    //   (q1, 'b') -> q0
    let dfa = tools::DFA::new(
        ["q0", "q1"],
        ['a', 'b'],
        [
            (("q0", 'a'), "q1"),
            (("q0", 'b'), "q0"),
            (("q1", 'a'), "q1"),
            (("q1", 'b'), "q0"),
        ],
        "q0",
        ["q1"],
    );

    let tests = ["", "a", "b", "ba", "abb", "abba", "abaa"];
    for s in tests {
        println!("{s:5} -> accepts? {}", dfa.accepts(s));
    }
}
