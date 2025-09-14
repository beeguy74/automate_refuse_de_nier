extern crate sdl2; 

use sdl2::pixels::Color;
use std::error::Error;
use sdl2::event::{self, Event};
use std::time::Duration;
use sdl2::keyboard::{Keycode, Scancode};

fn parse_key(name: &str) -> Option<Keycode> {
    // Try a layout-aware key first
    if let Some(kc) = Keycode::from_name(name) {
        return Some(kc);
    }
    // Fallback: try physical key name, then map to a Keycode
    Scancode::from_name(name).and_then(Keycode::from_scancode)
}


fn catch_keys(event: Event) -> Result<i32, Box<dyn Error>> {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            Ok(-1)
        },
        Event::KeyDown { keycode: Some(key), .. } => {
            Ok(key.into_i32())
        },
        _ => { Ok(-2)}
    }
}
 
pub fn run() -> Result<i32, Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'runnin: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match catch_keys(event) {
                Ok(key) if key > -1 => { println!("Key is {:?}", key) },  
                Ok(-1) => { break 'runnin },
                Ok(_) => {},
                Err(_) => { return Err("Wrong keys!".into()) }
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(0)
}