use std::collections::{BTreeSet, BTreeMap};

pub type State = String;
pub type Symbol = char;

// A DFA represented by:
// - Q: set of states
// - Σ: alphabet
// - δ_set: set of ordered pairs ((q, a), r)
// - start: start state
// - accept: set of accept states
#[derive(Debug)]
pub struct DFA {
    states: BTreeSet<State>,
    alphabet: BTreeSet<Symbol>,
    // transition function as a set of ordered pairs
    delta_set: BTreeSet<((State, Symbol), State)>,
    start: State,
    accept: BTreeSet<State>,
    // derived: for O(1)ish lookup of δ(q,a)
    delta_map: BTreeMap<(State, Symbol), State>,
}

impl DFA {
    pub fn new(
        states: impl IntoIterator<Item = State>,
        alphabet: impl IntoIterator<Item = Symbol>,
        delta_pairs: impl IntoIterator<Item = ((State, Symbol), State)>,
        start: State,
        accept: impl IntoIterator<Item = State>,
    ) -> Self {
        let states: BTreeSet<_> = states.into_iter().collect();
        let alphabet: BTreeSet<_> = alphabet.into_iter().collect();
        let delta_set: BTreeSet<_> = delta_pairs.into_iter().collect();

        // Build a map for quick lookup; also sanity-check determinism:
        let mut delta_map = BTreeMap::new();
        for (key, next) in &delta_set {
            if let Some(prev) = delta_map.insert(key.clone(), next.clone()) {
                panic!(
                    "Non-deterministic transitions specified: {:?} -> {:?} and {:?}",
                    key, prev, next
                );
            }
        }

        let accept: BTreeSet<_> = accept.into_iter().collect();

        Self {
            states,
            alphabet,
            delta_set,
            start,
            accept,
            delta_map,
        }
    }

    /// Build a DFA from a collection of moves. Each move is a pair of (sequence, name).
    /// The DFA will have a start state named "q0" and new states named "q1", "q2", ...
    /// Shared prefixes among moves will reuse states so the automaton is compact and deterministic.
    pub fn from_moves<I>(moves: I) -> Self
    where
        I: IntoIterator<Item = (Vec<Symbol>, String)>,
    {
        let mut states = BTreeSet::new();
        let mut alphabet = BTreeSet::new();
        let mut delta_map: BTreeMap<(State, Symbol), State> = BTreeMap::new();
        let mut delta_set = BTreeSet::new();
        let mut accept = BTreeSet::new();

        let start = "q0".to_string();
        states.insert(start.clone());

        // counter for new states
        let mut next_id: usize = 1;

        for (seq, _name) in moves.into_iter() {
            let mut current = start.clone();
            for sym in seq.into_iter() {
                alphabet.insert(sym);
                // if transition exists, follow it; otherwise create a new state
                if let Some(existing) = delta_map.get(&(current.clone(), sym)) {
                    current = existing.clone();
                    continue;
                }

                let new_state = format!("q{}", next_id);
                next_id += 1;
                // insert transition
                delta_map.insert((current.clone(), sym), new_state.clone());
                delta_set.insert(((current.clone(), sym), new_state.clone()));
                states.insert(new_state.clone());
                current = new_state;
            }
            // current is the final state for this move
            accept.insert(current.clone());
        }

        Self {
            states,
            alphabet,
            delta_set,
            start,
            accept,
            delta_map,
        }
    }

    // δ(q, a)
    fn delta(&self, q: State, a: Symbol) -> Option<State> {
        self.delta_map.get(&(q, a)).cloned()
    }

    // Extended δ on strings
    fn run(&self, input: &str) -> Option<State> {
        let mut q = self.start.clone();
        for c in input.chars() {
            q = self.delta(q, c)?;
        }
        Some(q)
    }

    pub fn accepts(&self, input: &str) -> bool {
        match self.run(input) {
            Some(qf) => self.accept.contains(&qf),
            None => false,
        }
    }
}

