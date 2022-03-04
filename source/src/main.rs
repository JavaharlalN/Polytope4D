mod vec4f;
mod edge;
mod face;
mod display;
mod comparison;
mod cell;
mod window;
mod field;
use cell::*;
use vec4f::*;
use edge::*;
use face::*;
use macroquad::prelude::*;

#[macroquad::main("Polytope 4D")]
async fn main() {
    loop {
        clear_background(Color::new(1.0, 1.0, 1.0, 1.0));
        next_frame().await
    }
}
