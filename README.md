# ft_ality - Fighting Game Combo Recognizer

A Rust implementation of a DFA-based combo recognition system for fighting games, inspired by Mortal Kombat. This project demonstrates finite-state automata, regular languages, and real-time input processing.

## Overview

In this project, you will have an insight into a particular kind of regular languages:
moves and combos from fighting games, such as Mortal Kombat. You will implement,
train, and run an automaton to recognise such moves from the list of key combinations
it is composed of; and you will understand that in the same way words are made with
symbols, combos are made with keys.

Your task is then simple: you will recreate a fighting game's training mode.
3
Chapter III
Objectives
This project is the first step of a series, aimed at helping you acquire the skills to perform
syntactic analysis (a.k.a. parsing) on formal languages. Implementing it should make
you familiar with the following concepts:
• Formal grammars
• The Chomsky hierarchy
• Regular languages (or type 3 languages)
• Finite-State Automata (Finite-State Machines)
This is also, incidentally, one of your first long and complex projects in functional
programming. You should take this opportunity to think carefully about your program’s
structure in terms of modules, and set yourself up with an efficient workflow using external
libraries.
4
Chapter IV
General instructions
• The inputs and outputs presented in this subject are only given as guidelines. You
are free to format those as you wish. As such, you may index your automaton’s
states whichever way you see fit. If you have a doubt, just play it safe and do it
like in the subject.
• This project is to do in functionnal. Any functionnal language is okay, as long
as the whole project is done using functionnal logic. This will be verified during
evaluation.
• Any library that does the job for you is forbidden. As usual.
• The open keyword is forbidden and considered cheating. Yes, yes, I know. But still.
• Your code will be compiled, which means you should never have the ;; token in
your source files.
• You are allowed to use the following modules:
◦ Pervasives
◦ List
◦ String
◦ Sys
◦ Sdl
◦ Sdlevent
◦ Sdlkey
• If you want to use a module and it’s not in this list, well, you can’t.
• No coding style is enforced for this project; but remember that small functions and
wisely designed modules are elements to writing well thought code. Your graders
5
Functionnal project ft_ality
will have the possibility to give you bonus points if your code is elegant and robust.
• You must provide a Makefile which will build your entire project.
6
Chapter V
Mandatory part
The runtime of this project mainly consists of two steps: training the automaton and
running it.
V.1 Formal definition
A finite-state automaton A is a tuple containing the following elements:
A = ⟨Q, Σ, Q0, F, δ⟩
• Σ is the automaton’s input alphabet.
• Q is the set of states in the automaton.
• Q0 is the starting state, with Q0 ∈ Q of course.
• F is the set of recognition states, with F ⊆ Q.
• δ is a function that assigns transitions to the automaton’s states; a transition is a
state associated with a pair consisting of a state and a symbol from the alphabet,
making the function’s type Q × Σ → Q.
Of course, feel free to do your research on the Internet if you need more information.
But the main idea is that an automaton reads the input (which is a word), symbol by
symbol, and goes from one state to another at each symbol using the transition function
δ. At the end of the input, if the automaton is in a recognition state, it recognizes the
word. If it’s not in a recognition state, then it... well, it doesn’t recognize the word.
Actually, the transition function could be a set. But then again,
functions and sets are actually the same thing. You do know that,
right?
7
Functionnal project ft_ality
V.2 Automaton training
Your automaton will be built at runtime, using grammar files that contain the moves
to be learned by the automaton. The file path of one grammar will be given in the
command line arguments. Since moves are usually very simple, all you have to do is split
(i.e., tokenize) your rule to get a list of tokens, and then give it to the automaton, which
will generate transitions for each token in succession.
This operation should be very quick (as in, near-instant).
V.3 Automaton running and language recognition
Once you have trained your automaton, go ahead and run it! Your program will wait for
input from the keyboard, just like the training mode of a fighting game. The user must
be able to press keys on their keyboard, using a key mapping displayed on the screen,
and move names should be displayed when their key combinations are executed.
The key mappings must be automatically computed from the grammar. If
they’re hardcoded, I will personally come and break your bones.
Example:
% ./ft_ality grammars/mk9.gmr
Key mappings:
q -> Block
down -> Down
w -> Flip Stance
left -> Left
right -> Right
e -> Tag
a -> Throw
up -> Up
s -> [BK]
d -> [BP]
z -> [FK]
x -> [FP]
----------------------
[BP]
Claw Slam (Freddy Krueger) !!
Knockdown (Sonya) !!
Fist of Death (Liu-Kang) !!
[BP], [FP]
Saibot Blast (Noob Saibot) !!
Active Duty (Jax) !!
8

VI.1 Code optimization
Your code will also be evaluated for performance outside of formal algorithmic criteria.
Here are a few guidelines to follow:
• Functional languages rely on recursion to loop through calculations. As such, your
functions should be written responsibly and be tail-recursive if they can be computationally heavy.
• The program should be reasonably scalable – input processing should be online. For
example, the inputs should always be processed line by line without the program
storing everything in memory; if it does, it will crash if given sufficiently large input.
VI.2 Functional programming philosophy
This project is in functional programming (whether you want it or not), which means
your code should follow functional programming principles as much as possible. Here are
some guidelines to help you (but really, this should be obvious to everyone).
• Your functions should be short (preferably less than 20 lines) and should use nested
definitions when needed. “Utility” functions left at the module’s top level result in
messy code.
• With the same idea in mind, your code should be split into well-thought-out modules; these modules should not export unnecessary functions (e.g. previously mentioned “utility” functions).
• Your code should avoid exceptions as much as possible. Exceptions break the
mechanism of stack frames, resulting in awkward constructions relying on implicitly
potential errors. You can’t do much to avoid system-induced exceptions such as
Sys_error, but if one of your functions can fail in one way or another, you can use
9
Functionnal project ft_ality
your type system to signify that. The option type or writing a try monad should
be better ideas.
• You should avoid mutable constructions as much as possible. Constant definitions
promote code scalability and behaviour validation at compile time. To be more
precise, you should avoid:
◦ the ref type
◦ mutable records
◦ arrays
◦ mutable attributes in objects
If your code respects all these guidelines, then congratulations! You’re in for some
bonus points :)
VI.3 Graphical interface
Your output could be implemented using a graphical interface to make it more visually
appealing (in case OCaml wasn’t, you know, visually appealing enough). You could draw
the automaton and show it recognizing your input step by step, for example. You can use
any library you want to implement your graphical interface. Unleash your imagination!
VI.4 Gamepads
Don’t blame me, I’m all with you on the PC master race idea. But let’s face it, keyboards
are a horrible way to play fighting games. You can use the Sdljoystick module and
implement gamepad handling. And when I say gamepads, it could be actual gamepads
or arcade sticks... You are free to do what you want, just make sure you do something
awesome.
VI.5 Debug mode
Your program can include a debug mode. You can trigger it in whatever way you want,
but it must be reachable at runtime (i.e., you must run the program in normal and debug
modes without having to rebuild the project to switch between modes). The debug mode
must at least show your automaton going from state to state and eventually finding (or
not) an end state with the rule(s) associated.
Butt slam (Ermac)
% ./ft_ality grammars/mk9.gmr
Key mappings:
q -> Block
down -> Down
w -> Flip Stance
left -> Left
right -> Right
e -> Tag
a -> Throw
up -> Up
s -> [BK]
10
Functionnal project ft_ality
d -> [BP]
z -> [FK]
x -> [FP]
----------------------
[BP]
State 1, "[BP]" -> State 11
Found end state for "Claw Slam (Freddy Krueger)" at: 11
Found end state for "Knockdown (Sonya)" at: 11
Found end state for "Fist of Death (Liu-Kang)" at: 11
Claw Slam (Freddy Krueger) !!
Knockdown (Sonya) !!
Fist of Death (Liu-Kang) !!
[BP], [FP]
State 11, "[FP]" -> State 162
Found end state for "Saibot Blast (Noob Saibot)" at: 162
Found end state for "Active Duty (Jax)" at: 162
Saibot Blast (Noob Saibot) !!
Active Duty (Jax) !!
The bonus part will only be assessed if the mandatory part is
PERFECT. Perfect means the mandatory part has been integrally done
and works without malfunctioning. If you have not passed ALL the
mandatory requirements, your bonus part will not be evaluated at all.

---

## Quick Start

### Building the Project

```bash
# Using make (recommended)
make

# Or using cargo directly
cargo build

# For optimized release build
make release
# or
cargo build --release
```

### Running the Program

```bash
# Run in console mode (default, no GUI)
make run

# Run with debug tracing (console mode)
make debug

# Run with SDL GUI window
make gui

# Run with GUI and debug mode
make gui-debug

# Manual execution examples
./target/debug/automate_refuse_de_nier grammars/mk9.gmr                    # Console mode
./target/debug/automate_refuse_de_nier grammars/mk9.gmr --gui              # GUI mode
./target/debug/automate_refuse_de_nier grammars/mk9.gmr --debug            # Console + debug
./target/debug/automate_refuse_de_nier grammars/mk9.gmr --gui --debug      # GUI + debug
```

### Usage

```
Usage: automate_refuse_de_nier <grammar_file.gmr> [OPTIONS]

Options:
  --gui      Enable graphical SDL window (optional)
  --debug    Enable debug mode with state transition tracing (optional)

Examples:
  automate_refuse_de_nier grammars/mk9.gmr                 # Console mode (default)
  automate_refuse_de_nier grammars/mk9.gmr --gui           # GUI mode
  automate_refuse_de_nier grammars/mk9.gmr --debug         # Console + debug
  automate_refuse_de_nier grammars/mk9.gmr --gui --debug   # GUI + debug
```

## Grammar File Format

Grammar files (`.gmr`) define key mappings and move sequences:

```
# Key mappings (single character to token name)
w, Up
a, Left
s, Down
d, Right
o, [BP]  # Back Punch
l, [FP]  # Front Punch

# Move definitions (move name: key sequence)
Claw Slam (Freddy Krueger): o
Saibot Blast (Noob Saibot): o l
Teleport Punch (Scorpion): s s l
```

### Format Rules:
- Lines starting with `#` are comments
- Key mappings: `<single_char>, <TokenName>`
- Move definitions: `<Move Name>: <key> <key> <key>`
- Keys in move sequences must be defined in the key mappings section

## Example Output

### Normal Mode
```
Key mappings:
a -> Left
d -> Right
o -> [BP]
l -> [FP]
----------------------
[BP]
Claw Slam (Freddy Krueger) !!
Knockdown (Sonya) !!
[BP], [FP]
Saibot Blast (Noob Saibot) !!
Active Duty (Jax) !!
```

### Debug Mode (`--debug` flag)
```
Key mappings:
a -> Left
d -> Right
o -> [BP]
l -> [FP]
----------------------
[BP]
State q0, "[BP]" -> State q1
Found end state for "Claw Slam (Freddy Krueger)" at: q1
Found end state for "Knockdown (Sonya)" at: q1
Claw Slam (Freddy Krueger) !!
Knockdown (Sonya) !!
[BP], [FP]
State q1, "[FP]" -> State q2
Found end state for "Saibot Blast (Noob Saibot)" at: q2
Found end state for "Active Duty (Jax)" at: q2
Saibot Blast (Noob Saibot) !!
Active Duty (Jax) !!
```

## Features Implemented

✅ **DFA-based combo recognition** - Deterministic finite automaton with shared prefix optimization
✅ **Automatic key mapping generation** - Derived from grammar tokens (not hardcoded)
✅ **Real-time input handling** - SDL2-based keyboard capture
✅ **Debug mode** - State transition tracing and end state detection
✅ **Multiple simultaneous matches** - Support for moves sharing ending states
✅ **Performance optimized** - Zero-copy transitions, O(1) lookups
✅ **Comprehensive testing** - Unit tests and stress tests included

## Performance

The system has been optimized for real-time input processing:

- **Average transition time**: ~428 ns (0.428 µs)
- **Throughput**: ~2.3 million transitions per second
- **Memory**: Minimal allocations in hot paths
- **Scaling**: Linear with DFA size, suitable for complex move sets

See [PERFORMANCE.md](PERFORMANCE.md) for detailed benchmarks.

## Testing

```bash
# Run unit tests
make test

# Run performance stress tests
make stress

# Run specific test
cargo test test_name
```

## Development

```bash
# Check code without building
make check

# Run linter
make lint

# Format code
make fmt

# Clean build artifacts
make clean
```

## Controls

- **Press mapped keys** (a, d, f, i, j, k, l, o, p, s, u, w) to input tokens
- **ESC or close window** to quit
- All key mappings are displayed at startup

## SDL Window

The program opens an SDL2 window to capture keyboard input. The window displays the key mappings automatically derived from your grammar file.

## Project Structure

```
.
├── src/
│   ├── main.rs              # Main entry point, CLI handling, runtime loop
│   ├── lib.rs               # Library exports for testing
│   └── tools/
│       ├── dfa.rs           # DFA implementation with move tracking
│       ├── parsing.rs       # Grammar file parser
│       ├── keycatcher.rs    # SDL input handling
│       └── mod.rs           # Module exports
├── tests/
│   └── stress_test.rs       # Performance stress tests
├── grammars/
│   ├── mk9.gmr              # Basic key mappings
│   └── mk9_with_moves.gmr   # Full grammar with moves
├── Makefile                 # Build automation
├── PERFORMANCE.md           # Performance benchmarks
└── README.md               # This file
```

## Technical Details

### DFA Construction
- States are created on-demand during move training
- Shared prefixes automatically reuse existing states
- Deterministic: exactly one transition per (state, symbol) pair
- Compact representation using BTreeMap for O(log n) lookups

### Input Processing
- SDL2 event loop at 60 FPS
- Token validation against grammar
- State tracking with automatic reset on invalid transitions
- Zero-copy processing for minimal latency

### Debug Mode
- Prints every state transition: `State X, "token" -> State Y`
- Shows when end states are reached: `Found end state for "Move" at: N`
- Useful for understanding automaton behavior and debugging grammars

## Requirements

- Rust 2021 edition or later
- SDL2 development libraries
- Linux/Unix environment (tested on Ubuntu)

## License

Educational project for learning formal languages and automata theory.

## Input Modes

### Console Mode (Default)
By default, the program runs in console mode, which reads input from stdin:
- No SDL window required
- Type tokens character by character, then press Enter
- Type 'quit' or 'exit' to terminate
- Ideal for scripting and testing

```bash
./target/debug/automate_refuse_de_nier grammars/mk9.gmr
# Type: ol
# Output: recognizes "Saibot Blast" when you press Enter
```

### GUI Mode (--gui flag)
When enabled with the `--gui` flag, opens an SDL window for real-time keyboard input:
- Visual display of key mappings
- Real-time keypress detection (no need to press Enter)
- Press ESC or close window to quit
- Better for interactive gameplay simulation

```bash
./target/debug/automate_refuse_de_nier grammars/mk9.gmr --gui
# Press keys directly - combos are recognized immediately
```

### Debug Mode (--debug flag)
Can be combined with either input mode to show DFA state transitions:
- Prints: `State X, "token" -> State Y`
- Shows: `Found end state for "Move Name" at: N`
- Useful for understanding automaton behavior

```bash
# Console mode with debug
./target/debug/automate_refuse_de_nier grammars/mk9.gmr --debug

# GUI mode with debug  
./target/debug/automate_refuse_de_nier grammars/mk9.gmr --gui --debug
```
