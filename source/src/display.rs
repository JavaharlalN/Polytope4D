use super::*;
use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

impl Display for Vec4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "V({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "E(\n  {},\n  {}\n)", self.a, self.b)
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "F(\n  {},\n  {},\n  {},\n)", self.edges[0], self.edges[1], self.edges[2])
    }
}