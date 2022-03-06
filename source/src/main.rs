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
mod angle;
use angle::*;
use macroquad::prelude::*;
use macroquad::input::*;
use cursor::*;
use objects::*;
use window::*;

fn draw_windows<'a>(windows: &'a WindowGroup) {
    draw_rectangle(
        windows.main.config.x,
        windows.main.config.y,
        windows.main.config.w,
        windows.main.config.h,
        Color::new(0.3, 0.3, 0.3, 0.5),
    );
}

fn draw_bg() {
    clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
    for i in 0..(screen_width() as i32 / 50) {
        for k in 0..(screen_height() as i32 / 50) {
            draw_line(
                (i * 50) as f32,
                0.0,
                (i * 50) as f32,
                screen_height(),
                1.0,
                Color::new(0.25, 0.25, 0.25, 1.0),
            );
            draw_line(
                0.0,
                (k * 50) as f32,
                screen_width(),
                (k * 50) as f32,
                1.0,
                Color::new(0.25, 0.25, 0.25, 1.0),
            );
        }
    }
}

fn resize_event<'a>(windows: &'a mut WindowGroup) {
    (*windows).main.set_size(screen_width(), screen_height());
}

#[macroquad::main("Polytope 4D")]
async fn main() {
    unsafe {
        sapp::sapp_show_mouse(false);
    }
    let mut windows = WindowGroup{
        main: MainWindow::new(screen_width(), screen_height()),
        scene: SceneWindow::new(screen_width(), screen_height()),
    };
    let mut cursor = Cursor::new(mouse_position());
    let mut last_size = (screen_width(), screen_height());
    let mut objects = vec![Object::tesseract()];
    let mut angle = Angle::new();
    loop {
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
        let (x_pos, y_pos) = mouse_position();
        let new_size = (screen_width(), screen_height());
        if new_size != last_size {
            resize_event(&mut windows);
            last_size = new_size;
        }
        draw_windows(&windows);
        cursor.conf.x = x_pos;
        cursor.conf.y = y_pos;
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
        next_frame().await
    }
}
