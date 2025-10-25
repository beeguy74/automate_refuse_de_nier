use automate_refuse_de_nier::tools::{DFA, DFAConfig};

/// Simulated stress test: process 100,000 token transitions
#[test]
fn stress_test_dfa_performance() {
    // Create a DFA with several moves
    let moves = vec![
        (vec!['a'], "Move A".to_string()),
        (vec!['a', 'b'], "Move AB".to_string()),
        (vec!['a', 'b', 'c'], "Move ABC".to_string()),
        (vec!['b', 'c'], "Move BC".to_string()),
        (vec!['d', 'd', 'd'], "Move DDD".to_string()),
    ];

    let dfa = DFA::from_moves(moves);
    let config = DFAConfig { debug: false };

    let mut current_state = dfa.start_state().clone();
    let test_sequence = vec!['a', 'b', 'c', 'd', 'd', 'd', 'a', 'b'];

    // Process 100,000 tokens
    let iterations = 100_000;

    for i in 0..iterations {
        let token = test_sequence[i % test_sequence.len()];
        let token_name = format!("[{}]", token);

        let (next_state, _matches) = dfa.step(&current_state, token, &token_name, &config);

        if let Some(next) = next_state {
            current_state = next.clone();
        } else {
            // Reset to start
            current_state = dfa.start_state().clone();
        }
    }

    // If we get here, the test passed (no crashes, no excessive memory)
    println!("Stress test completed: {} transitions processed", iterations);
}

#[test]
fn benchmark_dfa_lookup() {
    use std::time::Instant;

    // Create a larger DFA
    let mut moves = Vec::new();
    for i in 0..50 {
        let seq = vec!['a', (b'a' + (i % 26) as u8) as char];
        moves.push((seq, format!("Move {}", i)));
    }

    let dfa = DFA::from_moves(moves);
    let config = DFAConfig { debug: false };

    let mut current_state = dfa.start_state().clone();
    let iterations = 1_000_000;

    let start = Instant::now();

    for i in 0..iterations {
        let token = (b'a' + (i % 26) as u8) as char;
        let token_name = format!("[{}]", token);

        let (next_state, _matches) = dfa.step(&current_state, token, &token_name, &config);

        if let Some(next) = next_state {
            current_state = next.clone();
        } else {
            current_state = dfa.start_state().clone();
        }
    }

    let duration = start.elapsed();
    println!("Benchmark: {} transitions in {:?}", iterations, duration);
    println!("Average: {:.2} ns/transition", duration.as_nanos() as f64 / iterations as f64);

    // Assert reasonable performance (should be well under 1 microsecond per transition)
    assert!(duration.as_micros() < iterations, "DFA transitions too slow");
}
