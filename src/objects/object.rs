use crate::angle::Angle;
use crate::window::MainWindow;

use super::Face;
use super::Cell;
use super::Edge;
use super::Vec4f;

pub struct Object {
    pub vertices: Vec<Vec4f>,
    pub edges: Option<Vec<(usize, usize)>>,
    pub faces: Option<(
        Vec<usize>, // vertices
        Vec<usize>, // edges
    )>,
    pub cells: Option<(
        Vec<usize>, // vertices
        Vec<usize>, // edges
        Vec<usize>, // faces
    )>,
    pub name: Option<String>,
}

impl Object {
    pub fn calc_vertices(&mut self, a: &Angle, d: f32,  main: &MainWindow) {
        for (_, v) in self.vertices.iter_mut().enumerate() {
            v.calc(a, d, main);
        }
    }

    pub fn tesseract() -> Object {
        Object{
            vertices: vec![
                Vec4f::new(-1.0, -1.0, -1.0, -1.0), // 0
                Vec4f::new(-1.0, -1.0, -1.0,  1.0), // 1
                Vec4f::new(-1.0, -1.0,  1.0, -1.0), // 2
                Vec4f::new(-1.0, -1.0,  1.0,  1.0), // 3
                Vec4f::new(-1.0,  1.0, -1.0, -1.0), // 4
                Vec4f::new(-1.0,  1.0, -1.0,  1.0), // 5
                Vec4f::new(-1.0,  1.0,  1.0, -1.0), // 6
                Vec4f::new(-1.0,  1.0,  1.0,  1.0), // 7
                Vec4f::new( 1.0, -1.0, -1.0, -1.0), // 8
                Vec4f::new( 1.0, -1.0, -1.0,  1.0), // 9
                Vec4f::new( 1.0, -1.0,  1.0, -1.0), // 10
                Vec4f::new( 1.0, -1.0,  1.0,  1.0), // 11
                Vec4f::new( 1.0,  1.0, -1.0, -1.0), // 12
                Vec4f::new( 1.0,  1.0, -1.0,  1.0), // 13
                Vec4f::new( 1.0,  1.0,  1.0, -1.0), // 14
                Vec4f::new( 1.0,  1.0,  1.0,  1.0), // 15
            ],
            edges: Some(vec![
                (0, 1),
                (0, 2),
                (0, 4),
                (0, 8),
                (1, 3),
                (1, 5),
                (1, 9),
                (2, 3),
                (2, 6),
                (2, 10),
                (3, 7),
                (3, 11),
                (4, 5),
                (4, 6),
                (4, 12),
                (5, 7),
                (5, 13),
                (6, 7),
                (6, 14),
                (7, 15),
                (8, 9),
                (8, 10),
                (8, 12),
                (9, 11),
                (9, 13),
                (10, 11),
                (10, 14),
                (11, 15),
                (12, 13),
                (12, 14),
                (13, 15),
                (14, 15),
            ]),
            faces: Some((vec![], vec![])),
            cells: Some((vec![], vec![], vec![])),
            name: Some("Tessteract".to_string()),
        }
    }
}
