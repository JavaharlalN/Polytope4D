use super::Vec4f;

pub struct Angle {
    xy: f32,
    xz: f32,
    xw: f32,
    yz: f32,
    yw: f32,
    zw: f32,
}

impl Angle {
    pub fn new() -> Angle {
        Angle {
            xy: 0.0,
            xz: 0.0,
            xw: 0.0,
            yz: 0.0,
            yw: 0.0,
            zw: 0.0,
        }
    }
}