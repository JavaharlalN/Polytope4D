use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;
use std::clone::Clone;


pub fn dist(v1: Vec4f, v2: Vec4f) -> f32 {
    ((v1.x - v2.x).powf(2.0) + (v1.x - v2.x).powf(2.0) + (v1.x - v2.x).powf(2.0) + (v1.x - v2.x).powf(2.0)).sqrt()
}

#[derive(Debug, Clone)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    proj_x: f32,
    proj_y: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f {
            x: x,
            y: y,
            z: z,
            w: w,
            proj_x: 0.0,
            proj_y: 0.0,
        }
    }

    pub fn newf(v: f32) -> Vec4f {
        Vec4f::new (v, v, v, v)
    }

    pub fn dot(self, v: Vec4f) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn len(self) -> f32 {
        dist(self, Vec4f::newf(0.0))
    }
}

impl Add for Vec4f {
    type Output = Vec4f;
    fn add(self, v: Vec4f) -> Vec4f {
        Vec4f::new(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z,
            self.w + v.w
        )
    }
}

impl Sub for Vec4f {
    type Output = Vec4f;
    fn sub(self, v: Vec4f) -> Vec4f {
        Vec4f::new(
            self.x - v.x,
            self.y - v.y,
            self.z - v.z,
            self.w - v.w,
        )
    }
}

impl Mul<f32> for Vec4f {
    type Output = Vec4f;
    fn mul(self, v: f32) -> Vec4f {
        Vec4f::new(
            self.x * v,
            self.y * v,
            self.z * v,
            self.w * v,
        )
    }
}

impl Div<f32> for Vec4f {
    type Output = Vec4f;
    fn div(self, v: f32) -> Vec4f {
        Vec4f::new(
            self.x / v,
            self.y / v,
            self.z / v,
            self.w / v,
        )
    }
}

impl Display for Vec4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}