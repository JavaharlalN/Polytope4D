
#[derive(Debug, Clone)]
pub struct Edge {
    pub a:        usize,
    pub b:        usize,
    pub selected: bool,
}

impl Edge {
    pub fn new(a: usize, b: usize) -> Self {
        Edge { a, b, selected: false }
    }

    pub fn clone_and_select(&self, select: bool) -> Self {
        Edge {
            a:        self.a,
            b:        self.b,
            selected: select,
        }
    }
}