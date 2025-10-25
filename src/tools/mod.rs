pub mod dfa;
pub use dfa::{DFA, DFAConfig};
pub mod parsing;
pub use parsing::{parse_grammar_file, Grammar};
pub mod keycatcher;
pub use keycatcher::run_input_loop;