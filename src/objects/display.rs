use super::*;

impl Display for Vec4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vertice({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Edge(&{}, &{}, {}selected)", self.a, self.b, if self.selected { "" } else { "not " })
    }
}