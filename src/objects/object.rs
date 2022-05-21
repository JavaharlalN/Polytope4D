use crate::Edge;
use super::Vec4f;
use std::ops::Add;
use std::ops::AddAssign;
use crate::angle::Angle;
use crate::window::Window;

#[derive(Debug, Clone)]
pub struct Object {
    pub vertices: Vec<Vec4f>,
    pub edges: Vec<Edge>,
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
    pub fn empty() -> Object {
        Object {
            vertices: vec![],
            edges:    vec![],
            faces:    vec![],
            cells:    vec![],
            name:     None,
        }
    }

    pub fn clear_selection(&mut self) {
        for v in &mut self.vertices { v.selected = false; }
        for e in &mut self.edges { e.selected = false; }
        for f in &mut self.faces { f.2 = false; }
        for c in &mut self.cells { c.3 = false; }
    }

    pub fn select(&mut self) -> &mut Self {
        for v in &mut self.vertices { v.selected = true; }
        for e in &mut self.edges { e.selected = true; }
        for f in &mut self.faces { f.2 = true; }
        for c in &mut self.cells { c.3 = true; }
        self
    }

    pub fn calc_vertices(&mut self, a: &Angle, d: f32,  main: &Window) {
        for (_, v) in self.vertices.iter_mut().enumerate() {
            v.calc(a, d, main);
        }
    }

    pub fn select_vertice(&mut self, index: usize) {
        self.vertices[index].selected = true;
        for e in &mut self.edges {
            if index == e.a && self.vertices[e.b].selected {
                e.selected = true;
            }
            else if index == e.b && self.vertices[e.a].selected {
                e.selected = true;
            }
        }
    }

    pub fn deselect_vertice(&mut self, index: usize) {
        self.vertices[index].selected = false;
        for e in &mut self.edges {
            if index == e.a || index == e.b {
                e.selected = false;
            }
        }
    }

    pub fn select_edge(&mut self, index: usize) {
        self.edges[index].selected = true;
        let i1 = self.edges[index].a;
        let i2 = self.edges[index].b;
        self.select_vertice(i1);
        self.select_vertice(i2);
    }

    pub fn deselect_edge(&mut self, index: usize) {
        self.edges[index].selected = false;
        let i1 = self.edges[index].a;
        let i2 = self.edges[index].b;
        self.deselect_vertice(i1);
        self.deselect_vertice(i2);
    }

    pub fn deselect_first_n_vertices(&mut self, n: usize) {
        for i in 0..n {
            self.deselect_vertice(i);
        }
    }

    pub fn get_selected_vertices(&self) -> Vec<usize> {
        let mut indices = vec![];
        for (i, v) in self.vertices.iter().enumerate() {
            if v.selected {
                indices.push(i);
            }
        }
        indices
    }

    pub fn delete_vertex(&mut self, index: usize) {
        if !self.vertices[index].selected { return; }
        self.vertices.remove(index);
        let mut indices = vec![];
        for (i, e) in self.edges.iter_mut().enumerate() {
            if index == e.a || index == e.b { indices.push(i); }
            if e.a > index { e.a -= 1; }
            if e.b > index { e.b -= 1; }
        }
        for i in indices.iter().rev() {
            self.edges.remove(*i);
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
                Edge::new(0, 1),
                Edge::new(0, 2),
                Edge::new(0, 4),
                Edge::new(0, 8),
                Edge::new(1, 3),
                Edge::new(1, 5),
                Edge::new(1, 9),
                Edge::new(2, 3),
                Edge::new(2, 6),
                Edge::new(2, 10),
                Edge::new(3, 7),
                Edge::new(3, 11),
                Edge::new(4, 5),
                Edge::new(4, 6),
                Edge::new(4, 12),
                Edge::new(5, 7),
                Edge::new(5, 13),
                Edge::new(6, 7),
                Edge::new(6, 14),
                Edge::new(7, 15),
                Edge::new(8, 9),
                Edge::new(8, 10),
                Edge::new(8, 12),
                Edge::new(9, 11),
                Edge::new(9, 13),
                Edge::new(10, 11),
                Edge::new(10, 14),
                Edge::new(11, 15),
                Edge::new(12, 13),
                Edge::new(12, 14),
                Edge::new(13, 15),
                Edge::new(14, 15),
            ],
            faces: vec![],
            cells: vec![],
            name: Some("Tessteract".to_string()),
        }
    }
}

impl Add for Object {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new = self.clone();
        let count = self.vertices.len();
        for v in other.vertices {
            new.vertices.push(v.clone());
        }
        for e in other.edges {
            new.edges.push(Edge::new(e.a + count, e.b + count));
        }
        return new;
    }
}

impl AddAssign for Object {
    fn add_assign(&mut self, other: Self) {
        let count = self.vertices.len();
        for v in other.vertices {
            self.vertices.push(v.clone());
        }
        for e in other.edges {
            self.edges.push(Edge::new(e.a + count, e.b + count).clone_and_select(e.selected));
        }
    }
}