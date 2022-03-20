use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::clone::Clone;

pub fn dist(v1: Vec4f, v2: Vec4f) -> f32 {
    ((v1.x - v2.x).powf(2.0) + (v1.y - v2.y).powf(2.0) + (v1.z - v2.z).powf(2.0) + (v1.w - v2.w).powf(2.0)).sqrt()
}

#[derive(Debug, Copy, Clone)]
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
            x,
            y,
            z,
            w,
            proj_x: 0.0,
            proj_y: 0.0,
        }
    }

    pub fn newf(v: f32) -> Vec4f {
        Vec4f::new(v, v, v, v)
    }

    pub fn new0() -> Self {
        Vec4f::newf(0.0)
    }

    pub fn dot(self, v: Vec4f) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn len(self) -> f32 {
        dist(self, Vec4f::newf(0.0))
    }

    pub fn norm(self) -> Vec4f {
        let l = self.len();
        Vec4f::new(
            self.x / l,
            self.y / l,
            self.z / l,
            self.w / l,
        )
    }

    pub fn get_proj(self) -> (f32, f32) {
        (self.proj_x, self.proj_y)
    }

    pub fn set_proj(mut self, v: (f32, f32)) {
        self.proj_x = v.0;
        self.proj_y = v.1;
    }

    pub fn with_proj(self, v: (f32, f32)) -> Self {
        Vec4f {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
            proj_x: v.0,
            proj_y: v.1,
        }
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