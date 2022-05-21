use super::Button;
use crate::window::Window;

const STEPS: u8 = 10;

#[derive(Copy, Clone)]
pub struct Config {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub r: f32,
}

#[derive(Copy, Clone)]
pub struct Cursor {
    pub conf: Config,
    pub rect: bool,
    pub last: Config,
    pub real: Config,
    pub need: Config,
    pub step: u8,
}

impl Config {
    fn default() -> Config {
        Config{
            x: 0.0,
            y: 0.0,
            w: 3.0,
            h: 3.0,
            r: 3.0,
        }
    }

    fn with_pos(pos: (f32, f32)) -> Config {
        Config{
            x: pos.0,
            y: pos.1,
            w: 3.0,
            h: 3.0,
            r: 3.0,
        }
    }
}

impl Cursor {
    pub fn new(pos: (f32, f32)) -> Cursor {
        Cursor{
            conf: Config::with_pos(pos), // то, что отображается
            rect: false,
            last: Config::default(),     // то, что должно отображаться
            real: Config::default(),     // то, что на самом деле
            need: Config::default(),     // то, что было в начале анимации
            step: 0,
        }
    }

    pub fn intersect_with_button(
        self,
        button: &Button,
        window: &Window,
    ) -> bool {
        let (x, y) = button.get_pos(window);
        let (w, h) = button.size();
        self.real.x >= x && self.real.x < x + w &&
        self.real.y >= y && self.real.y < y + h
    }

    pub fn next(&mut self) {
        if self.step < STEPS{
            self.conf.x = self.last.x + self.step as f32 / STEPS as f32 * (self.need.x - self.last.x);
            self.conf.y = self.last.y + self.step as f32 / STEPS as f32 * (self.need.y - self.last.y);
            self.conf.w = self.last.w + self.step as f32 / STEPS as f32 * (self.need.w - self.last.w);
            self.conf.h = self.last.h + self.step as f32 / STEPS as f32 * (self.need.h - self.last.h);
            self.conf.r = self.last.r + self.step as f32 / STEPS as f32 * (self.need.r - self.last.r);
            self.step += 1;
        } else {
            self.conf = self.need;
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.real.x = x;
        self.real.y = y;
        if !self.rect {
            self.conf.x = x;
            self.conf.y = y;
        }
    }

    pub fn set(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.need.x = x;
        self.need.y = y;
        self.need.w = w;
        self.need.h = h;
        self.last = self.real.clone();
        self.conf = self.real.clone();
        self.rect = true;
        self.step = 0;
    }

    pub fn reset(&mut self) {
        self.conf = self.real;
        self.last = self.real;
        self.need = self.real;
        self.rect = false;
    }
}