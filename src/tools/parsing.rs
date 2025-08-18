use std::{array, collections::BTreeSet};
use crate::tools::{State, Symbol};

pub fn parsing() -> (
    BTreeSet<State>, //states
    BTreeSet<Symbol>, //alphabet
    BTreeSet<((State, Symbol), State)>, //delta_set
    State, //start
    BTreeSet<State>, //accept
) {
    use std::env::args;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let argv: Vec<String> = args().collect();
    if argv.len() != 2 {
        panic!("Expected exactly one argument (filename)");
    }
    let start_state = "q0".to_string();
    let finish_state= "q1".to_string();
    let mut states: BTreeSet<State> = BTreeSet::new();
    let mut alphabet: BTreeSet<Symbol> = BTreeSet::new();
    let mut delta_set: BTreeSet<((State, Symbol), String)> = BTreeSet::new();

    let file = File::open(&argv[1]).expect("Failed to open file");
    let buf = BufReader::new(file);
    for line in buf.lines() {
        if line.is_ok() {
            let line_content = line.unwrap();
            let mut split_parts = line_content.split(", ");
            let sym = split_parts.next().unwrap().chars().next().unwrap();
            let st = split_parts.next().unwrap().to_string();
            alphabet.insert(sym);
            states.insert(st.clone());
            delta_set.insert(((start_state.clone(), sym), finish_state.clone()));
        }
    }
    // for i in &states {
    //     delta_set.insert(((i.clone(), sym), st.clone()));
    // }
    
    // let accepted_states = states.clone();
    // states.insert(start_state.clone());
    // Return tuple of arrays for DFA construction
    (
        [start_state.clone(), finish_state.clone()].into(),
        alphabet,
        delta_set,
        start_state,
        [finish_state].into()
    )
}