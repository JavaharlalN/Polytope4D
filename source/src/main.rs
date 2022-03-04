mod vec4f;
mod edge;
mod face;
mod display;
mod comparison;
mod cell;
use cell::*;
use vec4f::*;
use edge::*;
use face::*;
use macroquad::prelude::*;

#[macroquad::main("Polytope 4D")]
async fn main() {
    let v1 = Vec4f::newf(0.0);
    let v2 = Vec4f::newf(0.0);
    let v3 = Vec4f::newf(0.0);
    println!("{}", Face::from_verts(vec![v1, v2, v3]));
}
