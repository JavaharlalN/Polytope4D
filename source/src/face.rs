use super::*;

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<Vec4f>,
    pub edges: Vec<Edge>
}

impl Face {
    pub fn new(vertices: Vec<Vec4f>, edges: Vec<Edge>) -> Face {
        Face {
            vertices: vertices,
            edges: edges,
        }
    }

    pub fn from_verts(vertices: Vec<Vec4f>) -> Face {
        let a = vertices[0].clone();
        let b = vertices[1].clone();
        let c = vertices[2].clone();
        Face {
            vertices: vertices,
            edges: vec![
                Edge::new(a, b),
                Edge::new(b, c),
                Edge::new(c, a),
            ]
        }
    }

    pub fn area(self) -> f32 {
        let a = self.edges[0].len();
        let b = self.edges[1].len();
        let c = self.edges[2].len();
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}