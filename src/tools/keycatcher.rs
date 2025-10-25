extern crate sdl2;

use std::error::Error;
use sdl2::event::Event;
use std::time::Duration;
use sdl2::keyboard::Keycode;
use crate::tools::parsing::Grammar;

/// Represents an input event from keyboard or gamepad
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    Token(char),    // A valid token character
    Quit,           // User wants to quit
    Invalid,        // Other events to ignore
}

/// Translates SDL Keycode to a character for our token system
fn keycode_to_char(keycode: Keycode) -> Option<char> {
    match keycode {
        // Letters
        Keycode::A => Some('a'),
        Keycode::B => Some('b'),
        Keycode::C => Some('c'),
        Keycode::D => Some('d'),
        Keycode::E => Some('e'),
        Keycode::F => Some('f'),
        Keycode::G => Some('g'),
        Keycode::H => Some('h'),
        Keycode::I => Some('i'),
        Keycode::J => Some('j'),
        Keycode::K => Some('k'),
        Keycode::L => Some('l'),
        Keycode::M => Some('m'),
        Keycode::N => Some('n'),
        Keycode::O => Some('o'),
        Keycode::P => Some('p'),
        Keycode::Q => Some('q'),
        Keycode::R => Some('r'),
        Keycode::S => Some('s'),
        Keycode::T => Some('t'),
        Keycode::U => Some('u'),
        Keycode::V => Some('v'),
        Keycode::W => Some('w'),
        Keycode::X => Some('x'),
        Keycode::Y => Some('y'),
        Keycode::Z => Some('z'),
        // Numbers
        Keycode::Num0 => Some('0'),
        Keycode::Num1 => Some('1'),
        Keycode::Num2 => Some('2'),
        Keycode::Num3 => Some('3'),
        Keycode::Num4 => Some('4'),
        Keycode::Num5 => Some('5'),
        Keycode::Num6 => Some('6'),
        Keycode::Num7 => Some('7'),
        Keycode::Num8 => Some('8'),
        Keycode::Num9 => Some('9'),
        _ => None,
    }
}

/// Processes an SDL event and returns an InputEvent
fn process_event(event: Event, grammar: &Grammar) -> InputEvent {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            InputEvent::Quit
        },
        Event::KeyDown { keycode: Some(keycode), .. } => {
            if let Some(ch) = keycode_to_char(keycode) {
                // Check if this character maps to a token in our grammar
                if grammar.get_token_for_key(ch).is_some() {
                    return InputEvent::Token(ch);
                }
            }
            InputEvent::Invalid
        },
        _ => InputEvent::Invalid
    }
}

/// Main input handling loop with SDL
/// Takes a grammar and a callback function that processes token events
/// Returns Ok(()) on normal exit, Err on error
pub fn run_input_loop<F>(grammar: &Grammar, mut on_token: F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(char, &str),
{
    let sdl_context = sdl2::init().map_err(|e| format!("SDL init failed: {}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| format!("Video subsystem failed: {}", e))?;

    let _window = video_subsystem.window("ft_ality - Fighting Game Training Mode", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| format!("Window creation failed: {}", e))?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| format!("Event pump failed: {}", e))?;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match process_event(event, grammar) {
                InputEvent::Token(ch) => {
                    // Get the token name and pass it to the callback
                    if let Some(token_name) = grammar.get_token_for_key(ch) {
                        on_token(ch, token_name);
                    }
                },
                InputEvent::Quit => {
                    break 'main_loop;
                },
                InputEvent::Invalid => {
                    // Ignore other events
                }
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

/// Simple demo function that prints tokens as they are pressed
pub fn run_demo(grammar: &Grammar) -> Result<(), Box<dyn Error>> {
    run_input_loop(grammar, |_ch, token_name| {
        println!("[{}]", token_name);
    })
}