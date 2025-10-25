use std::env;
use std::process;

mod tools;
use tools::{parse_grammar_file, run_demo};

fn main() {
    // Get grammar file from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <grammar_file.gmr>", args[0]);
        eprintln!("Example: {} grammars/mk9.gmr", args[0]);
        process::exit(1);
    }

    let grammar_path = &args[1];

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

    // Run the input handling demo
    if let Err(e) = run_demo(&grammar) {
        eprintln!("Error running input loop: {}", e);
        process::exit(1);
    }
}
