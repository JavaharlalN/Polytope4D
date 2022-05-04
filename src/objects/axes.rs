use crate::window::MainWindow;
use crate::angle::Angle;
use super::{Vec4f, find_closest_edge, dist2d};

#[derive(Debug, Copy, Clone)]
pub struct Axes {
    pub x: Vec4f,
    pub y: Vec4f,
    pub z: Vec4f,
    pub w: Vec4f,
    pub offset: (f32, f32),
}

#[derive(Debug, Copy, Clone)]
pub struct MotionAxes {
    pub x: Vec4f,
    pub y: Vec4f,
    pub z: Vec4f,
    pub w: Vec4f,
    pub pos: Option<Vec4f>,
    pub grabbed: bool,
    pub grab_start: Option<Vec4f>,
    pub grab_now: Option<Vec4f>,
}

impl MotionAxes {
    pub fn new() -> Self {
        MotionAxes {
            x: Vec4f::new(0.5, 0.0, 0.0, 0.0),
            y: Vec4f::new(0.0, 0.5, 0.0, 0.0),
            z: Vec4f::new(0.0, 0.0, 0.5, 0.0),
            w: Vec4f::new(0.0, 0.0, 0.0, 0.5),
            grab_start: None,
            grab_now: None,
            grabbed: false,
            pos: None,
        }
    }

    pub fn calc(&mut self, a: &Angle, window: &MainWindow) {
        if let Some(mut pos) = self.pos {
            let x = (self.x + pos).calc(a, 5.0, window);
            let y = (self.y + pos).calc(a, 5.0, window);
            let z = (self.z + pos).calc(a, 5.0, window);
            let w = (self.w + pos).calc(a, 5.0, window);
            if let Some(proj) = x.get_proj() { self.x.set_proj(proj); }
            if let Some(proj) = y.get_proj() { self.y.set_proj(proj); }
            if let Some(proj) = z.get_proj() { self.z.set_proj(proj); }
            if let Some(proj) = w.get_proj() { self.w.set_proj(proj); }
            self.pos = Some(pos.calc(a, 5.0, window));
        }
    }

    pub fn any_axe_selected(self) -> bool {
        return self.x.selected
            || self.y.selected
            || self.z.selected
            || self.w.selected;
    }

    pub fn try_grab(&mut self) {
        if self.any_axe_selected() {
            self.grab_start = self.pos;
            self.grabbed = true;
        }
    }

    pub fn ungrab(&mut self) {
        self.grab_start = None;
        self.grab_now = None;
        self.grabbed = false;
    }

    fn get_motion_delta_for_axe(
        &mut self,
        axe: Vec4f,
        xy_delta: (f32, f32),
        a: &Angle,
        window: &MainWindow
    ) -> Vec4f {
        if let (Some(proj), Some(pos)) = (axe.get_proj(), self.pos) {
            if let Some(pos_proj) = pos.get_proj() {
                let axe2d = super::sub2d(proj, pos_proj);
                // println!("{}", dist2d(axe2d, (0.0, 0.0)));
                let proj_delta_to_axe = super::proj2d(axe2d, xy_delta);
                let ratio = proj_delta_to_axe / dist2d((0.0, 0.0), axe2d);
                let delta = axe * ratio;
                if let Some(now) = self.grab_now {
                    self.grab_now = Some((now + delta).calc(a, 5.0, window));
                } else if let Some(start) = self.grab_start {
                    self.grab_now = Some((start + delta).calc(a, 5.0, window));
                }
                return delta;
            }
        }
        return Vec4f::new0();
    }

    pub fn get_motion_delta(&mut self, xy_delta: (f32, f32), a: &Angle, window: &MainWindow) -> Vec4f {
        if !self.any_axe_selected() { return Vec4f::new0(); }
        if self.x.selected { return self.get_motion_delta_for_axe(self.x, xy_delta, a, window); }
        if self.y.selected { return self.get_motion_delta_for_axe(self.y, xy_delta, a, window); }
        if self.z.selected { return self.get_motion_delta_for_axe(self.z, xy_delta, a, window); }
        if self.w.selected { return self.get_motion_delta_for_axe(self.w, xy_delta, a, window); }
        return Vec4f::new0();
    }

    pub fn move_to(&mut self, pos: Vec4f) {
        self.pos = Some(pos);
    }

    fn select_axe(&mut self, index: usize) {
        self.x.selected = index == 0;
        self.y.selected = index == 1;
        self.z.selected = index == 2;
        self.w.selected = index == 3;
    }

    fn clear_selection(&mut self) {
        self.x.selected = false;
        self.y.selected = false;
        self.z.selected = false;
        self.w.selected = false;
    }

    pub fn get_closest_axe(&mut self, x: f32, y: f32) -> Option<usize> {
        let pos: Vec4f = if let Some(p) = self.pos { p }
                         else { return None; };
        let axes_object = super::Object {
            vertices: vec![
                pos,
                self.x,
                self.y,
                self.z,
                self.w,
            ],
            edges: vec![
                (0, 1, false),
                (0, 2, false),
                (0, 3, false),
                (0, 4, false),
            ],
            faces: vec![],
            cells: vec![],
            name: None,
        };
        if let Some(index) = find_closest_edge(x, y, &axes_object) {
            self.select_axe(index);
            return Some(index);
        } else {
            self.clear_selection();
            return None;
        }
    }
}

impl Axes {
    pub fn new(x: f32, y: f32) -> Self {
        Axes {
            x: Vec4f::new(1.0, 0.0, 0.0, 0.0),
            y: Vec4f::new(0.0, 1.0, 0.0, 0.0),
            z: Vec4f::new(0.0, 0.0, 1.0, 0.0),
            w: Vec4f::new(0.0, 0.0, 0.0, 1.0),
            offset: (x, y),
        }
    }

    pub fn calc(&mut self, a: &Angle, window: &MainWindow) {
        self.x.calc(a, 8.0, window);
        self.y.calc(a, 8.0, window);
        self.z.calc(a, 8.0, window);
        self.w.calc(a, 8.0, window);
    }
}