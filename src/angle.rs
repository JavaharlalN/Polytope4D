pub struct Angle {
    pub xy: f32,
    pub xz: f32,
    pub xw: f32,
    pub yz: f32,
    pub yw: f32,
    pub zw: f32,
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