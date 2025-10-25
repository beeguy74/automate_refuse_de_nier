extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::tools::parsing::Grammar;

/// UI Constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const BACKGROUND_COLOR: Color = Color::RGB(20, 20, 30);
const TEXT_COLOR: Color = Color::RGB(200, 200, 255);
const HEADER_COLOR: Color = Color::RGB(255, 200, 100);
const BORDER_COLOR: Color = Color::RGB(100, 100, 150);

/// Render key mappings to the SDL canvas
pub fn render_key_mappings(canvas: &mut Canvas<Window>, grammar: &Grammar) {
    // Clear the screen
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    // Draw border
    canvas.set_draw_color(BORDER_COLOR);
    canvas.draw_rect(Rect::new(10, 10, WINDOW_WIDTH - 20, WINDOW_HEIGHT - 20)).ok();

    // Note: Full text rendering would require SDL2_ttf
    // For now, we draw colored boxes to represent the UI layout

    // Header area (title)
    canvas.set_draw_color(HEADER_COLOR);
    canvas.fill_rect(Rect::new(20, 20, WINDOW_WIDTH - 40, 60)).ok();

    // Key mappings area
    canvas.set_draw_color(Color::RGB(40, 40, 60));
    let mut y_offset = 100;
    let row_height = 30;
    let mappings_count = grammar.mappings.len().min(15); // Limit display

    for i in 0..mappings_count {
        let rect = Rect::new(30, y_offset + (i as i32 * row_height),
                             WINDOW_WIDTH - 60, (row_height - 5) as u32);
        canvas.fill_rect(rect).ok();
    }

    // Instructions area at bottom
    canvas.set_draw_color(TEXT_COLOR);
    canvas.fill_rect(Rect::new(20, WINDOW_HEIGHT as i32 - 100,
                                WINDOW_WIDTH - 40, 80)).ok();

    canvas.present();
}

/// Get window dimensions
pub fn get_window_size() -> (u32, u32) {
    (WINDOW_WIDTH, WINDOW_HEIGHT)
}
