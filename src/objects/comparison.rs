use super::*;

impl PartialEq for Vec4f {
    fn eq(&self, v: &Self) -> bool {
        self.x == v.x &&
        self.y == v.y &&
        self.z == v.z &&
        self.w == v.w
    }
}