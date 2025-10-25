Functionnal project
ft_ality

In this project, you will have an insight into a particular kind of regular languages:
moves and combos from fighting games, such as Mortal Kombat. You will implement,
train, and run an automaton to recognise such moves from the list of key combinations
it is composed of; and you will understand that in the same way words are made with
symbols, combos are made with keys.
Your task is then simple: you will recreate a fighting game’s training mode.
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
