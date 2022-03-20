use super::*;


impl Vec4f {
    pub fn rotated_xy(self, angle: &f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x * cos + self.y * sin,
            self.y * cos - self.x * sin,
            self.z,
            self.w,
        )
    }

    pub fn rotated_xz(self, angle: &f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x * cos + self.z * sin,
            self.y,
            self.z * cos - self.x * sin,
            self.w,
        )
    }

    pub fn rotated_xw(self, angle: &f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x * cos + self.w * sin,
            self.y,
            self.z,
            self.w * cos - self.x * sin,
        )
    }

    pub fn rotated_yz(self, angle: &f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x,
            self.y * cos + self.z * sin,
            self.z * cos - self.y * sin,
            self.w,
        )
    }

    pub fn rotated_yw(self, angle: &f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x,
            self.y * cos + self.w * sin,
            self.z,
            self.w * cos - self.y * sin,
        )
    }

    pub fn rotated_zw(self, angle: &f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x,
            self.y,
            self.z * cos + self.w * sin,
            self.w * cos - self.z * sin
        )
    }
}

impl Edge {
    pub fn rotated_xy(self, angle: &f32) -> Self {
        Edge::new (
            self.a.rotated_xy(angle),
            self.b.rotated_xy(angle)
        )
    }

    pub fn rotated_xz(self, angle: &f32) -> Self {
        Edge::new (
            self.a.rotated_xz(angle),
            self.b.rotated_xz(angle)
        )
    }

    pub fn rotated_xw(self, angle: &f32) -> Self {
        Edge::new (
            self.a.rotated_xw(angle),
            self.b.rotated_xw(angle)
        )
    }

    pub fn rotated_yz(self, angle: &f32) -> Self {
        Edge::new (
            self.a.rotated_yz(angle),
            self.b.rotated_yz(angle)
        )
    }

    pub fn rotated_yw(self, angle: &f32) -> Self {
        Edge::new (
            self.a.rotated_yw(angle),
            self.b.rotated_yw(angle)
        )
    }

    pub fn rotated_zw(self, angle: &f32) -> Self {
        Edge::new (
            self.a.rotated_zw(angle),
            self.b.rotated_zw(angle)
        )
    }
}

impl Face {
    pub fn rotated_xy(self, angle: &f32) -> Self {
        Face::from_verts(vec![
            self.vertices[0].rotated_xy(angle),
            self.vertices[1].rotated_xy(angle),
            self.vertices[2].rotated_xy(angle),
        ])
    }

    pub fn rotated_xz(self, angle: &f32) -> Self {
        Face::from_verts(vec![
            self.vertices[0].rotated_xz(angle),
            self.vertices[1].rotated_xz(angle),
            self.vertices[2].rotated_xz(angle),
        ])
    }

    pub fn rotated_xw(self, angle: &f32) -> Self {
        Face::from_verts(vec![
            self.vertices[0].rotated_xw(angle),
            self.vertices[1].rotated_xw(angle),
            self.vertices[2].rotated_xw(angle),
        ])
    }

    pub fn rotated_yz(self, angle: &f32) -> Self {
        Face::from_verts(vec![
            self.vertices[0].rotated_yz(angle),
            self.vertices[1].rotated_yz(angle),
            self.vertices[2].rotated_yz(angle),
        ])
    }

    pub fn rotated_yw(self, angle: &f32) -> Self {
        Face::from_verts(vec![
            self.vertices[0].rotated_yw(angle),
            self.vertices[1].rotated_yw(angle),
            self.vertices[2].rotated_yw(angle),
        ])
    }

    pub fn rotated_zw(self, angle: &f32) -> Self {
        Face::from_verts(vec![
            self.vertices[0].rotated_zw(angle),
            self.vertices[1].rotated_zw(angle),
            self.vertices[2].rotated_zw(angle),
        ])
    }
}