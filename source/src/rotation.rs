use super::*;

trait Rotated {
    type Out;
    fn rotated_xy(self, angle: &f32) -> Self::Out;
    fn rotated_xz(self, angle: &f32) -> Self::Out;
    fn rotated_xw(self, angle: &f32) -> Self::Out;
    fn rotated_yz(self, angle: &f32) -> Self::Out;
    fn rotated_yw(self, angle: &f32) -> Self::Out;
    fn rotated_zw(self, angle: &f32) -> Self::Out;
}

impl Rotated for Vec4f {
    type Out = Vec4f;
    fn rotated_xy(self, angle: &f32) -> Self::Out {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x * cos + self.y * sin,
            self.y * cos - self.x * sin,
            self.z,
            self.w,
        )
    }

    fn rotated_xz(self, angle: &f32) -> Self::Out {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x * cos + self.z * sin,
            self.y,
            self.z * cos - self.x * sin,
            self.w,
        )
    }

    fn rotated_xw(self, angle: &f32) -> Self::Out {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x * cos + self.w * sin,
            self.y,
            self.z,
            self.w * cos - self.x * sin,
        )
    }

    fn rotated_yz(self, angle: &f32) -> Self::Out {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x,
            self.y * cos + self.z * sin,
            self.z * cos - self.y * sin,
            self.w,
        )
    }

    fn rotated_yw(self, angle: &f32) -> Self::Out {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec4f::new(
            self.x,
            self.y * cos + self.w * sin,
            self.z,
            self.w * cos - self.y * sin,
        )
    }

    fn rotated_zw(self, angle: &f32) -> Self::Out {
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

impl Rotated for Edge {
    type Out = Edge;
    fn rotated_xy(self, angle: &f32) -> Self::Out {
        Edge::new (
            self.a.rotated_xy(angle),
            self.b.rotated_xy(angle)
        )
    }

    fn rotated_xz(self, angle: &f32) -> Self::Out {
        Edge::new (
            self.a.rotated_xz(angle),
            self.b.rotated_xz(angle)
        )
    }

    fn rotated_xw(self, angle: &f32) -> Self::Out {
        Edge::new (
            self.a.rotated_xw(angle),
            self.b.rotated_xw(angle)
        )
    }

    fn rotated_yz(self, angle: &f32) -> Self::Out {
        Edge::new (
            self.a.rotated_yz(angle),
            self.b.rotated_yz(angle)
        )
    }

    fn rotated_yw(self, angle: &f32) -> Self::Out {
        Edge::new (
            self.a.rotated_yw(angle),
            self.b.rotated_yw(angle)
        )
    }

    fn rotated_zw(self, angle: &f32) -> Self::Out {
        Edge::new (
            self.a.rotated_zw(angle),
            self.b.rotated_zw(angle)
        )
    }
}