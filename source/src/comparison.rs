use std::cmp::Ordering;
use std::cmp::Ord;
use super::*;

impl PartialEq for Vec4f {
    fn eq(&self, v: &Self) -> bool {
        self.x == v.x &&
        self.y == v.y &&
        self.z == v.z &&
        self.w == v.w
    }
}

impl PartialEq for Edge {
    fn eq(&self, v: &Self) -> bool {
        self.len() == v.len()
    }
}

impl PartialEq for Face {
    fn eq(&self, v: &Self) -> bool {
        let f1: Face = self.clone();
        let f2: Face = v.clone();
        f1.area() == f2.area()
    }
}