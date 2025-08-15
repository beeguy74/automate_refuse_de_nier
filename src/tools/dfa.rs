use std::collections::{BTreeSet, BTreeMap};

type State = &'static str;
type Symbol = char;

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
            if let Some(prev) = delta_map.insert(*key, *next) {
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

    // δ(q, a)
    fn delta(&self, q: State, a: Symbol) -> Option<State> {
        self.delta_map.get(&(q, a)).copied()
    }

    // Extended δ on strings
    fn run(&self, input: &str) -> Option<State> {
        let mut q = self.start;
        for c in input.chars() {
            q = self.delta(q, c)?;
        }
        Some(q)
    }

    pub fn accepts(&self, input: &str) -> bool {
        match self.run(input) {
            Some(qf) => self.accept.contains(qf),
            None => false,
        }
    }
}

