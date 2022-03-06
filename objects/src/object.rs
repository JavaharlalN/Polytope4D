use super::Face;
use super::Cell;
use super::Edge;
use super::Vec4f;

pub struct Object {
    pub vertices: Vec<Vec4f>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub cells: Vec<Cell>,
    pub name: String,
}

impl Object {
    pub fn tesseract() -> Object {
        Object{
            vertices: vec![
                Vec4f::new(-0.5, -0.5, -0.5,  0.5),
                Vec4f::new(-0.5, -0.5,  0.5, -0.5),
                Vec4f::new(-0.5, -0.5,  0.5,  0.5),
                Vec4f::new(-0.5,  0.5, -0.5, -0.5),
                Vec4f::new(-0.5,  0.5, -0.5,  0.5),
                Vec4f::new(-0.5,  0.5,  0.5, -0.5),
                Vec4f::new(-0.5,  0.5,  0.5,  0.5),
                Vec4f::new( 0.5, -0.5, -0.5, -0.5),
                Vec4f::new( 0.5, -0.5, -0.5,  0.5),
                Vec4f::new( 0.5, -0.5,  0.5, -0.5),
                Vec4f::new( 0.5, -0.5,  0.5,  0.5),
                Vec4f::new( 0.5,  0.5, -0.5, -0.5),
                Vec4f::new( 0.5,  0.5, -0.5,  0.5),
                Vec4f::new( 0.5,  0.5,  0.5, -0.5),
                Vec4f::new( 0.5,  0.5,  0.5,  0.5),
            ],
            edges: vec![
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5, -0.5), Vec4f::new(-0.5, -0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5, -0.5), Vec4f::new(-0.5, -0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5, -0.5), Vec4f::new(-0.5,  0.5, -0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5, -0.5), Vec4f::new( 0.5, -0.5, -0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5,  0.5), Vec4f::new(-0.5, -0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5,  0.5), Vec4f::new(-0.5,  0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5, -0.5,  0.5), Vec4f::new( 0.5, -0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5,  0.5, -0.5), Vec4f::new(-0.5, -0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5,  0.5, -0.5), Vec4f::new(-0.5,  0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5,  0.5, -0.5), Vec4f::new( 0.5, -0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5,  0.5,  0.5), Vec4f::new(-0.5,  0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5, -0.5,  0.5,  0.5), Vec4f::new( 0.5, -0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5, -0.5, -0.5), Vec4f::new(-0.5,  0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5, -0.5, -0.5), Vec4f::new(-0.5,  0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5, -0.5, -0.5), Vec4f::new( 0.5,  0.5, -0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5, -0.5,  0.5), Vec4f::new(-0.5,  0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5, -0.5,  0.5), Vec4f::new( 0.5,  0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5,  0.5, -0.5), Vec4f::new( 0.5,  0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5,  0.5, -0.5), Vec4f::new( 0.5,  0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new(-0.5,  0.5,  0.5,  0.5), Vec4f::new( 0.5,  0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5, -0.5, -0.5), Vec4f::new( 0.5, -0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5, -0.5, -0.5), Vec4f::new( 0.5, -0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5, -0.5, -0.5), Vec4f::new( 0.5,  0.5, -0.5, -0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5, -0.5,  0.5), Vec4f::new( 0.5, -0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5, -0.5,  0.5), Vec4f::new( 0.5,  0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5,  0.5, -0.5), Vec4f::new( 0.5, -0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5,  0.5, -0.5), Vec4f::new( 0.5,  0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new( 0.5, -0.5,  0.5,  0.5), Vec4f::new( 0.5,  0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5,  0.5, -0.5, -0.5), Vec4f::new( 0.5,  0.5, -0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5,  0.5, -0.5, -0.5), Vec4f::new( 0.5,  0.5,  0.5, -0.5)),
                Edge::new(Vec4f::new( 0.5,  0.5, -0.5,  0.5), Vec4f::new( 0.5,  0.5,  0.5,  0.5)),
                Edge::new(Vec4f::new( 0.5,  0.5,  0.5, -0.5), Vec4f::new( 0.5,  0.5,  0.5,  0.5)),
            ],
            faces: vec![],
            cells: vec![],
            name: "Tessteract".to_string(),
        }
    }
}
