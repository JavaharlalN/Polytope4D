use crate::angle::Angle;
use crate::window::MainWindow;
use super::Vec4f;

pub struct Object {
    pub vertices: Vec<Vec4f>,
    pub edges: Vec<(usize, usize, bool)>,
    pub faces: Vec<(
        (usize, usize, usize), // vertices
        (usize, usize, usize), // edges
        bool,                  // selected
    )>,
    pub cells: Vec<(
        (usize, usize, usize, usize),               // vertices
        (usize, usize, usize, usize, usize, usize), // edges
        (usize, usize, usize, usize),               // faces
        bool,                                       // selected
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
            edges: vec![
                (0, 1, false),
                (0, 2, false),
                (0, 4, false),
                (0, 8, false),
                (1, 3, false),
                (1, 5, false),
                (1, 9, false),
                (2, 3, false),
                (2, 6, false),
                (2, 10, false),
                (3, 7, false),
                (3, 11, false),
                (4, 5, false),
                (4, 6, false),
                (4, 12, false),
                (5, 7, false),
                (5, 13, false),
                (6, 7, false),
                (6, 14, false),
                (7, 15, false),
                (8, 9, false),
                (8, 10, false),
                (8, 12, false),
                (9, 11, false),
                (9, 13, false),
                (10, 11, false),
                (10, 14, false),
                (11, 15, false),
                (12, 13, false),
                (12, 14, false),
                (13, 15, false),
                (14, 15, false),
            ],
            faces: vec![],
            cells: vec![],
            name: Some("Tessteract".to_string()),
        }
    }
}
