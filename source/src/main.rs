mod vec4f;
mod edge;
mod face;
mod display;
use vec4f::*;
use edge::*;
use face::*;

fn main() {
    let v = Vec4f::newf(0.0);
    println!("{}", v);
}
