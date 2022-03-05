use objects::*;
use macroquad::prelude::*;
use ::miniquad::Context;

extern "system" {
    
}

#[macroquad::main("Polytope 4D")]
async fn main() {
    let ctx = Context::new();
    ctx.show_mouse(false);
    loop {
        clear_background(Color::new(1.0, 1.0, 1.0, 1.0));
        next_frame().await
    }
}
