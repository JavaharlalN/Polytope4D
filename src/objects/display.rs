use super::*;

impl Display for Vec4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vertice({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Cell=====\n  {},\n  {},\n  {},\n  {}\n=====", self.faces[0], self.faces[1], self.faces[2], self.faces[3])
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Face-----\n  {},\n  {},\n  {},\n-----", self.edges[0], self.edges[1], self.edges[2])
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Edge(\n  {},\n  {}\n)", self.a, self.b)
    }
}