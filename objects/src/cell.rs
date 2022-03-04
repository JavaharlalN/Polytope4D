use super::Vec4f;
use super::Edge;
use super::Face;

#[derive(Debug, Clone)]
pub struct Cell {
    pub vertices: Vec<Vec4f>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

impl Cell {
    pub fn new(vertices: Vec<Vec4f>, edges: Vec<Edge>, faces: Vec<Face>) -> Cell {
        Cell{
            vertices: vertices,
            edges: edges,
            faces: faces,
        }
    }

    pub fn from_verts(vertices: Vec<Vec4f>) -> Cell {
        let a = vertices[0];
        let b = vertices[1];
        let c = vertices[2];
        let d = vertices[3];
        let edges = vec![
            Edge::new(a, b), // 0
            Edge::new(b, c), // 1
            Edge::new(c, a), // 2
            Edge::new(a, d), // 3
            Edge::new(b, d), // 4
            Edge::new(c, d), // 5
        ];
        let faces = vec![
            Face::new(
                vec![a, b, c],
                vec![
                    edges[0],
                    edges[1],
                    edges[2],
                ]
            ),
            Face::new(
                vec![a, b, d],
                vec![
                    edges[0],
                    edges[3],
                    edges[4],
                ]
            ),
            Face::new(
                vec![a, c, d],
                vec![
                    edges[2],
                    edges[3],
                    edges[5],
                ]
            ),
            Face::new(
                vec![b, c, d],
                vec![
                    edges[1],
                    edges[4],
                    edges[5],
                ]
            )
        ];
        Cell::new(vertices, edges, faces)
    }

    fn get_equation(self) -> (f32, f32, f32, f32, f32) {
        let x1 = self.vertices[0].x;
        let x2 = self.vertices[1].y;
        let x3 = self.vertices[2].z;
        let x4 = self.vertices[3].w;
        let y1 = self.vertices[0].x;
        let y2 = self.vertices[1].y;
        let y3 = self.vertices[2].z;
        let y4 = self.vertices[3].w;
        let z1 = self.vertices[0].x;
        let z2 = self.vertices[1].y;
        let z3 = self.vertices[2].z;
        let z4 = self.vertices[3].w;
        let w1 = self.vertices[0].x;
        let w2 = self.vertices[1].y;
        let w3 = self.vertices[2].z;
        let w4 = self.vertices[3].w;
        let a1 = x2 - x1;
        let b1 = y2 - y1;
        let c1 = z2 - z1;
        let d1 = w2 - w1;
        let a2 = x3 - x1;
        let b2 = y3 - y1;
        let c2 = z3 - z1;
        let d2 = w3 - w1;
        let a3 = x4 - x1;
        let b3 = y4 - y1;
        let c3 = z4 - z1;
        let d3 = w4 - w1;
        let a = b1 * c2 * d3 - b2 * c3 * d1 - b3 * c1 * d2;
        let b = a1 * c2 * d3 - a2 * c3 * d1 - a3 * c1 * d2;
        let c = a1 * b2 * d3 - a2 * b3 * d1 - a3 * b1 * d2;
        let d = a1 * b2 * c3 - a2 * b3 * c1 - a3 * b1 * c2;
        let e = -a * x1 - b * y1 - c * z1 - d * w1;
        (a, b, c, d, e)
    }
}