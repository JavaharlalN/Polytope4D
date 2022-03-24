use super::*;

impl Display for Vec4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vertice({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}