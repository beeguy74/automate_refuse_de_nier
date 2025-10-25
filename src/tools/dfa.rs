use std::collections::{BTreeMap, BTreeSet};

pub type State = String;
pub type Symbol = char;

/// Represents a recognized move with its name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub name: String,
}

/// Configuration for DFA runtime behavior
#[derive(Debug, Clone)]
pub struct DFAConfig {
    pub debug: bool,
}

impl Default for DFAConfig {
    fn default() -> Self {
        Self { debug: false }
    }
}

// A DFA for combo recognition that tracks which moves end at each state
#[derive(Debug)]
pub struct DFA {
    start: State,
    // derived: for O(1)ish lookup of δ(q,a)
    delta_map: BTreeMap<(State, Symbol), State>,
    // Maps each accept state to the list of moves that end at that state
    state_moves: BTreeMap<State, Vec<String>>,
}

impl DFA {
    /// Build a DFA from a collection of moves. Each move is a pair of (sequence, name).
    /// The DFA will have a start state named "q0" and new states named "q1", "q2", ...
    /// Shared prefixes among moves will reuse states so the automaton is compact and deterministic.
    pub fn from_moves<I>(moves: I) -> Self
    where
        I: IntoIterator<Item = (Vec<Symbol>, String)>,
    {
        let mut delta_map: BTreeMap<(State, Symbol), State> = BTreeMap::new();
        let mut state_moves: BTreeMap<State, Vec<String>> = BTreeMap::new();

        let start = "q0".to_string();

        // counter for new states
        let mut next_id: usize = 1;

        for (seq, name) in moves.into_iter() {
            let mut current = start.clone();
            for sym in seq.into_iter() {
                // if transition exists, follow it; otherwise create a new state
                if let Some(existing) = delta_map.get(&(current.clone(), sym)) {
                    current = existing.clone();
                    continue;
                }

                let new_state = format!("q{}", next_id);
                next_id += 1;
                // insert transition
                delta_map.insert((current.clone(), sym), new_state.clone());
                current = new_state;
            }
            // current is the final state for this move
            // Track which move(s) end at this state
            state_moves
                .entry(current)
                .or_insert_with(Vec::new)
                .push(name);
        }

        Self {
            start,
            delta_map,
            state_moves,
        }
    }

    /// Get the start state
    pub fn start_state(&self) -> &State {
        &self.start
    }

    /// Transition function: δ(q, a) -> q'
    /// Returns None if no transition exists
    /// Optimized to avoid unnecessary clones in hot path
    pub fn delta(&self, q: &State, a: Symbol) -> Option<&State> {
        self.delta_map.get(&(q.clone(), a))
    }

    /// Check if a state is an accept state and return the moves that match
    pub fn get_matches(&self, state: &State) -> Option<&Vec<String>> {
        self.state_moves.get(state)
    }

    /// Process a single symbol from a given state, with optional debug output
    /// Returns (new_state, matched_moves)
    /// Optimized to minimize allocations in the hot path
    pub fn step(
        &self,
        current: &State,
        symbol: Symbol,
        token_name: &str,
        config: &DFAConfig,
    ) -> (Option<&State>, &[String]) {
        let next = self.delta(current, symbol);

        if config.debug {
            if let Some(next_state) = next {
                println!(
                    "State {}, \"{}\" -> State {}",
                    current, token_name, next_state
                );
            } else {
                println!("State {}, \"{}\" -> <no transition>", current, token_name);
            }
        }

        let matches = if let Some(next_state) = next {
            if let Some(moves) = self.get_matches(next_state) {
                if config.debug {
                    for move_name in moves {
                        println!("Found end state for \"{}\" at: {}", move_name, next_state);
                    }
                }
                moves.as_slice()
            } else {
                &[]
            }
        } else {
            &[]
        };

        (next, matches)
    }
}
