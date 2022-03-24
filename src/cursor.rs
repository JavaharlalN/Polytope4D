// const STEPS: u8 = 10;

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
            need: Config::default(),
            step: 0,
        }
    }
}