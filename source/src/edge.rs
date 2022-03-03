pub use super::vec4f::*;

pub struct Edge {
    pub a: Vec4f,
    pub b: Vec4f,
}

impl Edge {
    pub fn new(a: Vec4f, b: Vec4f) -> Edge {
        Edge {
            a: a,
            b: b
        }
    }
}