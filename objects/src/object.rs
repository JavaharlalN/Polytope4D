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