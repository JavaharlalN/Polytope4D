mod axes;
mod edge;
mod vector;
mod object;
mod camera;
mod display;
mod rotation;
mod comparison;
pub use edge::*;
pub use axes::*;
pub use camera::*;
pub use vector::*;
pub use object::Object;
pub use std::fmt::Result;
pub use std::fmt::Display;
pub use std::fmt::Formatter;
use crate::dist_to_edge;

pub const CLICK_TIMEOUT:    u128 = 200;
pub const CUR_TRANSFORM_TO: u128 = 20;
pub const MAX_DIST:         f32  = 20.0;

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

pub fn find_closest_edge(x: f32, y: f32, obj: &Object) -> Option<usize> {
    let mut closest = None;
    let mut min_dist = None;
    for (i, e) in obj.edges.iter().enumerate() {
        if let Some(d) = dist_to_edge(x, y, e, &obj.vertices) {
            if let Some(min_d) = min_dist {
                if d < min_d { min_dist = Some(d); closest = Some(i)}
            } else { min_dist = Some(d); closest = Some(i) }
        }
    }
    if let Some(d) = min_dist {
        if d < MAX_DIST { closest }
        else { None }
    } else { None }
}