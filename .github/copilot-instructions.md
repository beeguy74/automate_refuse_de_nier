
# Copilot instructions for ft_ality (Rust)

This repository implements ft_ality: a finite-state automaton trainer/runner for
recognizing fighting-game combos from grammar files. These instructions help AI
coding agents make focused, small, verifiable changes while preserving existing
structure.

1) Short project instruction (what to implement)
- The program trains a finite-state automaton from a grammar file (see
	`grammars/*.gmr`) and then listens to keyboard/gamepad input to recognise
	moves. Training is done at runtime by tokenising rules and building DFA
	transitions; running processes live input and prints/matches moves.

2) What is already present (quick map to code)
- Project root: `Cargo.toml`, `Makefile` — standard Rust crate and build helper.
- Grammar files: `grammars/mk9.gmr` — contains move/token rules used for tests.
- Binary entry: `src/main.rs` — program wiring and top-level run loop.
- Tools modules: `src/tools/mod.rs` and these key files:
	- `src/tools/dfa.rs` — automaton representation and state transitions.
	- `src/tools/parsing.rs` — grammar/token parsing and token -> symbol mapping.
	- `src/tools/keycatcher.rs` — input handling (keyboard/gamepad) and mapping.
	- other helpers in `src/tools/` contain support utilities used by the CLI/UI.

3) Coding contract for agents (inputs/outputs, conventions)
- Inputs: grammar file path(s) (e.g. `grammars/mk9.gmr`) and optional flags
	(debug). Outputs: key mapping and move names when combos are recognised.
- Data shapes: DFA types live in `dfa.rs` (states, transitions, start, accept
	sets). Parsing returns Vec<Token> or similar; key mappings are derived from
	grammar tokens.
- Error modes: invalid grammar -> user-facing error; runtime input mismatch ->
	ignore and continue. Prefer Result/Option and avoid panics for expected errors.

4) Patterns & idioms to follow (based on this codebase)
- Keep changes local to `src/tools/*` and `src/main.rs` for wiring. Small, single
	purpose functions are preferred.
- Use idiomatic Rust: ownership, iterators, Option/Result, and avoid global
	mutable state. Prefer pure functions for parsing/dfa logic and isolate IO in
	`main.rs` / `keycatcher.rs`.
- Performance: DFA operations should be O(1) per input symbol; prefer Vec/HashMap
	with preallocated capacities for transitions if needed.
- Tests: add unit tests alongside modules (e.g., `src/tools/dfa.rs` ->
	`#[cfg(test)] mod tests`) covering training+recognition small cases.

5) Concrete examples (where to change for common tasks)
- To add new grammar parsing features: edit `src/tools/parsing.rs`. See how
	`mk9.gmr` tokens are read and tokenised.
- To change automaton behaviour or add debug traces: modify `src/tools/dfa.rs`.
	Keep debug optional via a flag passed from `main.rs`.
- To change key mapping or input handling: edit `src/tools/keycatcher.rs` and
	follow existing key -> token mapping behaviour. The mapping must be
	automatically generated from grammar tokens.

6) Guidance for commits and edits
- Make small, focused commits that compile. Run `cargo build` and `cargo test`
	before committing. If you modify `Makefile`, ensure it still builds via the
	project's default steps.
- Prefer adding a small test and updating `main.rs` wiring for manual testing.
- Avoid large refactors across many files in one change. If necessary, split
	into multiple commits with passing builds each.

7) Quick run / verification hints
- Typical local run: `cargo run -- grammars/mk9.gmr` (the project is a Rust
	binary; Makefile may provide shortcuts). Verify key mappings print and that
	pressing keys (or simulated inputs in tests) yields expected move names.

8) What NOT to change or assume
- Do not replace the automaton with a third-party library — DFA logic belongs
	in `src/tools/dfa.rs` and should be implemented in-repo.
- Don’t assume other languages or build systems; keep changes within the Rust
	crate and Makefile.

If you find missing tests or small issues, add focused unit tests under the
relevant module and a short note in the commit message describing the intent.

Files worth checking when working on features:
- `Cargo.toml` — dependency and crate metadata
- `Makefile` — build shortcuts
- `grammars/` — grammar inputs used for training
- `src/main.rs` — CLI, flags, and debug wiring
- `src/tools/dfa.rs`, `src/tools/parsing.rs`, `src/tools/keycatcher.rs` — core
	logic locations

Thank you — keep changes small, tests green, and reference the files above in
your commit messages.
