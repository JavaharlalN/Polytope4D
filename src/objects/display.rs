use super::*;

impl Display for Vec4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vertice({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Edge(&{}, &{}, {}selected)", self.0, self.1, if self.2 { "" } else { "not " })
    }
}