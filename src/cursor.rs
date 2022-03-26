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
            conf: Config::with_pos(pos),
            rect: false,
            last: Config::default(),
            real: Config::default(),
            need: Config::default(),
            step: 0,
        }
    }

    pub fn intersect_with_box(
        self,
        xb: f32, yb: f32,
        w: f32, h: f32,
    ) -> bool {
        self.conf.x >= xb && self.conf.x < xb + w &&
        self.conf.y >= yb && self.conf.y < yb + h 
    }

    pub fn next(&mut self) {
        if self.step < STEPS{
            self.conf.x = self.last.x + self.step as f32 / STEPS as f32 * (self.need.x - self.last.x);
            self.conf.y = self.last.y + self.step as f32 / STEPS as f32 * (self.need.y - self.last.y);
            self.conf.w = self.last.w + self.step as f32 / STEPS as f32 * (self.need.w - self.last.w);
            self.conf.h = self.last.h + self.step as f32 / STEPS as f32 * (self.need.h - self.last.h);
            self.conf.r = self.last.r + self.step as f32 / STEPS as f32 * (self.need.r - self.last.r);
        }
        self.step += 1;
    }

    pub fn reset(&mut self) {
        self.conf = self.real;
        self.rect = false;
    }
}