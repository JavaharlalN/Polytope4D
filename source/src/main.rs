use objects::*;
use macroquad::prelude::*;
// use ::miniquad::Context;
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

#[macroquad::main("Polytope 4D")]
async fn main() {
    unsafe {
        sapp::sapp_show_mouse(false);
    }
    loop {
        clear_background(Color::new(1.0, 1.0, 1.0, 1.0));
        next_frame().await
    }
}
