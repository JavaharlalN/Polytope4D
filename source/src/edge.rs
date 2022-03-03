pub use super::vec4f::*;

#[derive(Debug, Copy, Clone)]
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

    pub fn len(self) -> f32 {
        dist(self.a, self.b)
    }
}