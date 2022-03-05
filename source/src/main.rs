#[cfg(target_os = "macos")]
extern crate sapp_darwin as sapp;
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_arch = "wasm32",
    windows
)))]
extern crate sapp_dummy as sapp;
#[cfg(target_os = "linux")]
extern crate sapp_linux as sapp;
#[cfg(target_arch = "wasm32")]
extern crate sapp_wasm as sapp;
#[cfg(windows)]
extern crate sapp_windows as sapp;
mod cursor;
use macroquad::prelude::*;
use macroquad::input::mouse_position;
use cursor::*;
use objects::*;
use window::*;

#[macroquad::main("Polytope 4D")]
async fn main() {
    unsafe {
        sapp::sapp_show_mouse(false);
    }
    let main_window = MainWindow::new(screen_width(), screen_height());
    let mut cursor = Cursor::new(mouse_position());
    loop {
        let (x_pos, y_pos) = mouse_position();
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        cursor.conf.x = x_pos;
        cursor.conf.y = y_pos;
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
        next_frame().await
    }
}
