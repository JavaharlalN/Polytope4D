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

impl Object {
    pub fn rotated_xy(self, angle: &f32) -> Self {
        let mut vertices = vec![];
        for v in self.vertices.into_iter() {
            vertices.push(v.rotated_xy(angle));
        }
        Object {
            vertices,
            edges: self.edges.clone(),
            faces: self.faces.clone(),
            cells: self.cells.clone(),
            name: self.name.clone(),
        }
    }

    pub fn rotated_xz(self, angle: &f32) -> Self {
        let mut vertices = vec![];
        for v in self.vertices.into_iter() {
            vertices.push(v.rotated_xz(angle));
        }
        Object {
            vertices,
            edges: self.edges.clone(),
            faces: self.faces.clone(),
            cells: self.cells.clone(),
            name: self.name.clone(),
        }
    }

    pub fn rotated_xw(self, angle: &f32) -> Self {
        let mut vertices = vec![];
        for v in self.vertices.into_iter() {
            vertices.push(v.rotated_xw(angle));
        }
        Object {
            vertices,
            edges: self.edges.clone(),
            faces: self.faces.clone(),
            cells: self.cells.clone(),
            name: self.name.clone(),
        }
    }

    pub fn rotated_yz(self, angle: &f32) -> Self {
        let mut vertices = vec![];
        for v in self.vertices.into_iter() {
            vertices.push(v.rotated_yz(angle));
        }
        Object {
            vertices,
            edges: self.edges.clone(),
            faces: self.faces.clone(),
            cells: self.cells.clone(),
            name: self.name.clone(),
        }
    }

    pub fn rotated_yw(self, angle: &f32) -> Self {
        let mut vertices = vec![];
        for v in self.vertices.into_iter() {
            vertices.push(v.rotated_yw(angle));
        }
        Object {
            vertices,
            edges: self.edges.clone(),
            faces: self.faces.clone(),
            cells: self.cells.clone(),
            name: self.name.clone(),
        }
    }

    pub fn rotated_zw(self, angle: &f32) -> Self {
        let mut vertices = vec![];
        for v in self.vertices.into_iter() {
            vertices.push(v.rotated_zw(angle));
        }
        Object {
            vertices,
            edges: self.edges.clone(),
            faces: self.faces.clone(),
            cells: self.cells.clone(),
            name: self.name.clone(),
        }
    }
}