use super::Vec4f;

pub struct Camera {
    pub c: Vec4f, // center
    pub n: Vec4f, // normal (direction)
}

impl Camera {
    pub fn new(c: Vec4f) -> Self {
        Camera {
            c: c.clone(),
            n: Vec4f::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}