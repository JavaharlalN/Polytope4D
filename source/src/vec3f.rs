use std::ops::Add;

struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
    proj_x: f32,
    proj_y: f32
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) {
        Vec3f {
            x: x,
            y: y,
            z: z,
            w: w,
            proj_x: 0.0,
            proj_y: 0.0,
        }
    }

    pub fn newi(x: i64, y: i64, z: i64, w: i64) {
        Vec3f::new(
            x: x as f32,
            y: y as f32,
            z: z as f32,
            w: w as f32,
        )
    }
}