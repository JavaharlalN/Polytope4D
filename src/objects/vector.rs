use crate::window::Window;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::clone::Clone;

use crate::angle::Angle;

pub const SCALE: f32 = 3000.0;

pub fn dist(v1: Vec4f, v2: Vec4f) -> f32 {
    ((v1.x - v2.x).powf(2.0) + (v1.y - v2.y).powf(2.0) + (v1.z - v2.z).powf(2.0) + (v1.w - v2.w).powf(2.0)).sqrt()
}

pub fn sub2d(xy1: (f32, f32), xy2: (f32, f32)) -> (f32, f32) {
    (xy1.0 - xy2.0, xy1.1 - xy2.1)
}

pub fn dot2d(xy1: (f32, f32), xy2: (f32, f32)) -> f32 {
    xy1.0 * xy2.0 + xy1.1 * xy2.1
}

pub fn proj2d(xy1: (f32, f32), xy2: (f32, f32)) -> f32 {
    dot2d(xy1, xy2) / dist2d(xy1, xy2)
}

pub fn dist2d(xy1: (f32, f32), xy2: (f32, f32)) -> f32 {
    ((xy2.0 - xy1.0).powf(2.0) + (xy2.1 - xy1.1).powf(2.0)).sqrt()
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    proj_x: Option<f32>,
    proj_y: Option<f32>,
    pub selected: bool,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f {
            x,
            y,
            z,
            w,
            proj_x:   None,
            proj_y:   None,
            selected: false,
        }
    }

    pub fn newf(v: f32) -> Vec4f {
        Vec4f::new(v, v, v, v)
    }

    pub fn new0() -> Self {
        Vec4f::newf(0.0)
    }

	pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
		(self.x, self.y, self.z, self.w)
	}
	
	pub fn as_vec(&self) -> Vec<f32> {
		vec![self.x, self.y, self.z, self.w]
	}

    pub fn dot(self, v: Vec4f) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn move_x(mut self, v: f32) {
        self.x += v;
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

    pub fn get_proj(self) -> Option<(f32, f32)> {
        if let Some((Some(x), Some(y))) = Some((self.proj_x, self.proj_y)) {
            Some((x, y))
        } else { None }
    }

    pub fn set_proj(&mut self, v: (f32, f32)) {
        self.proj_x = Some(v.0);
        self.proj_y = Some(v.1);
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self) {
        self.selected = false;
    }

    pub fn with_proj(self, v: (f32, f32)) -> Self {
        Vec4f {
            x:        self.x,
            y:        self.y,
            z:        self.z,
            w:        self.w,
            proj_x:   Some(v.0),
            proj_y:   Some(v.1),
            selected: false,
        }
    }

    pub fn centered(self, w: f32, h: f32) -> Option<(f32, f32)> {
        if let Some((x, y)) = self.get_proj() {
            Some((
                x - w / 2.0,
                y - h / 2.0,
            ))
        } else {
            None
        }
    }

    pub fn calc(&mut self, a: &Angle, d: f32, window: &Window) -> Vec4f {
        let rotated = self.rotated_xy(&a.xy)
                                 .rotated_xz(&a.xz)
                                 .rotated_xw(&a.xw)
                                 .rotated_yz(&a.yz)
                                 .rotated_yw(&a.yw)
                                 .rotated_zw(&a.zw);
        let w = 1.0 / (d - rotated.w);
        let x = rotated.x * w;
        let y = rotated.y * w;
        let z = rotated.z * w;
        let proj3d = (x, y, z);
        let z = 1.0 / (d - rotated.w - proj3d.2) * SCALE;
        let x = proj3d.0 * z + window.config().w / 2.0;
        let y = proj3d.1 * z + window.config().h / 2.0;
        self.set_proj((x, y));
        self.with_proj((x, y))
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

impl AddAssign for Vec4f {
    fn add_assign(&mut self, v: Vec4f) {
        let selected = self.selected;
        *self = *self + v; // we can lost selection and projection here!!!!!!!!!
        self.selected = selected;
    }
}