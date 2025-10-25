
# Project TODO — ft_ality (automate_refuse_de_nier)

This file contains a structured, actionable todo list for implementing the
ft_ality project described in `README.md`. Each item includes files to edit
and acceptance criteria so work can be verified quickly.

1. Project skeleton check
	- Verify repository contains `Cargo.toml`, `Makefile`, `README.md`, and `grammars/`.
	- Files: `Cargo.toml`, `Makefile`, `README.md`.
	- Acceptance: `cargo build` and `make` run (or document outstanding issues).

2. Grammar parser
	- Implement tokenizer/parser for `.gmr` grammar files to produce tokens and move definitions.
	- File: `src/tools/parsing.rs`.
	- Acceptance: unit tests parse `grammars/mk9.gmr` into moves with token sequences and names.

3. DFA structure & trainer
	- Implement DFA types and training algorithm to convert token sequences into states/transitions.
	- File: `src/tools/dfa.rs`.
	- Acceptance: can train on sample grammar and produce deterministic transitions; include unit tests.

4. Key mapping generator
	- Derive key mappings automatically from grammar tokens (e.g., `BP`, `FP`, `left`).
	- Files: `src/tools/parsing.rs`, `src/tools/keycatcher.rs`.
	- Acceptance: printed key mapping matches tokens in `mk9.gmr` as in README examples.

5. Input handling (keyboard/gamepad)
	- Implement live input capture, translating keypresses/joystick events into token stream.
	- File: `src/tools/keycatcher.rs`.
	- Acceptance: produces token events for the DFA; keyboard supported; optional SDL gamepad support.

6. Main wiring & runtime loop
	- Wire CLI, training from file args, show key mappings, run input loop feeding DFA and printing matches.
	- File: `src/main.rs`.
	- Acceptance: running the binary with a grammar prints mappings and recognizes moves during input tests.

7. Debug mode & tracing
	- Add runtime debug flag to show state transitions and matched rules.
	- Files: `src/main.rs`, `src/tools/dfa.rs`.
	- Acceptance: `--debug` prints state transitions and found end states similar to README.

8. Recognition output formatting
	- Format matched-move output (tokens then move names) and support multiple simultaneous matches.
	- Files: `src/main.rs`, `src/tools/dfa.rs`.
	- Acceptance: readable output matching README examples.

9. Unit tests & CI sanity
	- Add unit tests for parser and DFA (happy path + edge cases).
	- Files: tests in `src/tools/parsing.rs` and `src/tools/dfa.rs` using `#[cfg(test)]`.
	- Acceptance: `cargo test` passes for added tests.

10. Performance improvements
	 - Optimize hot paths: minimize allocations, ensure DFA lookup O(1) per symbol.
	 - Files: `src/tools/dfa.rs`, `src/tools/keycatcher.rs`.
	 - Acceptance: basic stress test (simulated input) completes without large memory growth; document results.

11. Makefile & README update
	 - Ensure `Makefile` builds and `README.md` documents usage and debug flag.
	 - Files: `Makefile`, `README.md`.
	 - Acceptance: `make` builds and README documents how to run and test.

12. Optional: GUI / Gamepad support (bonus)
	 - Add SDL-based UI to show mappings and state graph, and full gamepad input support.
	 - Files: `src/tools/keycatcher.rs` (+ new UI module).
	 - Acceptance: basic SDL window shows mappings and accepts a connected gamepad. Mark as optional/bonus.

---

Next steps: pick item 2 (Grammar parser) and start implementing parser + tests.
