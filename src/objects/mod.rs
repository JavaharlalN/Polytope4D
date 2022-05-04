mod display;
mod vector;
mod comparison;
mod rotation;
mod object;
mod camera;

pub use std::fmt::Display;
pub use std::fmt::Result;
pub use std::fmt::Formatter;
pub use object::Object;
pub use vector::*;
pub use camera::*;

pub fn get_center(objects: &Vec<Object>) -> Option<Vec4f> {
    let mut center = Vec4f::new0();
    let mut sum: u64 = 0;
    for obj in objects {
        for v in &obj.vertices {
            if v.selected {
                center += *v;
                sum += 1;
            }
        }
    }
    if sum > 0 {
        return Some(center / sum as f32);
    }
    return None;
}