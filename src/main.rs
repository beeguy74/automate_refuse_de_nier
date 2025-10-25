use std::env;
use std::process;
use std::sync::{Arc, Mutex};

mod tools;
use tools::{parse_grammar_file, run_input_loop, DFA, DFAConfig};

/// Simple token buffer for tracking recent input
struct TokenBuffer {
    tokens: Vec<char>,
    max_length: usize,
}

impl TokenBuffer {
    fn new(max_length: usize) -> Self {
        Self {
            tokens: Vec::new(),
            max_length,
        }
    }

    fn push(&mut self, token: char) {
        self.tokens.push(token);
        if self.tokens.len() > self.max_length {
            self.tokens.remove(0);
        }
    }

    fn clear(&mut self) {
        self.tokens.clear();
    }
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <grammar_file.gmr> [--debug]", args[0]);
        eprintln!("Example: {} grammars/mk9.gmr", args[0]);
        eprintln!("         {} grammars/mk9.gmr --debug", args[0]);
        process::exit(1);
    }

    let grammar_path = &args[1];
    let debug_mode = args.iter().any(|arg| arg == "--debug");

    // Parse the grammar file
    let grammar = match parse_grammar_file(grammar_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing grammar file: {}", e);
            process::exit(1);
        }
    };

    // Display key mappings automatically derived from grammar
    grammar.display_key_mappings();

    // Build DFA from moves in the grammar
    let dfa = if grammar.moves.is_empty() {
        eprintln!("Warning: No moves defined in grammar file. The DFA will not recognize any combos.");
        eprintln!("Add move definitions like: Move Name: k e y s");
        DFA::from_moves(vec![])
    } else {
        let move_data: Vec<_> = grammar.moves.iter()
            .map(|m| (m.sequence.clone(), m.name.clone()))
            .collect();
        DFA::from_moves(move_data)
    };

    // Create DFA configuration
    let config = DFAConfig { debug: debug_mode };

    // Track current DFA state
    let current_state = Arc::new(Mutex::new(dfa.start_state().clone()));
    let token_buffer = Arc::new(Mutex::new(TokenBuffer::new(20)));

    // Clone references for the closure
    let current_state_clone = Arc::clone(&current_state);
    let token_buffer_clone = Arc::clone(&token_buffer);

    // Run input loop with DFA integration
    if let Err(e) = run_input_loop(&grammar, move |ch, token_name| {
        // Add token to buffer
        {
            let mut buffer = token_buffer_clone.lock().unwrap();
            buffer.push(ch);
        }

        // Print the token as it's pressed (echo input)
        print!("[{}]", token_name);

        // Process token through DFA
        let mut state = current_state_clone.lock().unwrap();
        let (next_state, matches) = dfa.step(&state, ch, token_name, &config);

        if let Some(next) = next_state {
            // Valid transition
            *state = next;

            // Print matched moves
            if !matches.is_empty() {
                println!();  // New line after token
                for move_name in &matches {
                    println!("{} !!", move_name);
                }
            } else {
                // Continue on same line if no match
                print!(", ");
            }
        } else {
            // No valid transition - reset to start
            println!();  // New line
            *state = dfa.start_state().clone();

            // Try from start state with this token
            let (next_state2, matches2) = dfa.step(&state, ch, token_name, &config);
            if let Some(next2) = next_state2 {
                *state = next2;
                if !matches2.is_empty() {
                    for move_name in &matches2 {
                        println!("{} !!", move_name);
                    }
                }
            }
        }
    }) {
        eprintln!("Error running input loop: {}", e);
        process::exit(1);
    }

    println!("\nExiting...");
}
