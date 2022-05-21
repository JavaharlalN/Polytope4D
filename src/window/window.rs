use super::ObjectField;
use crate::{objects::Object, button::Button};

#[derive(Debug, Copy, Clone)]
pub enum HintAlign {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

#[derive(Debug, Clone)]
pub enum Window {
    Main(MainWindow),
    Scene(SceneWindow),
}

impl Window {
    pub fn config(&self) -> Parameters {
        match self {
            Window::Main(w) => w.config.clone(),
            Window::Scene(w) => w.config.clone(),
        }
    }

    pub fn pos(&self) -> (f32, f32) {
        match self {
            Window::Main(w) => (w.config.x, w.config.y),
            Window::Scene(w) => (w.config.x, w.config.y),
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Window::Main(w) => (w.config.w, w.config.h),
            Window::Scene(w) => (w.config.w, w.config.h),
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        match self {
            Window::Main(win) => {win.config.w = w; win.config.h = h},
            Window::Scene(win) => {win.config.w = w; win.config.h = h},
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HintArea {
    pub w: f32,
    pub h: f32,
    pub align: HintAlign,
    pub busy: bool,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub x:       f32,
    pub y:       f32,
    pub w:       f32,
    pub h:       f32,
    pub grabbed: bool,
    pub name:    String,
}

#[derive(Debug, Clone)]
pub struct MainWindow {
    pub config:  Parameters,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone)]
pub struct SceneWindow {
    pub config:      Parameters,
    pub objects:     Vec<Object>,
    pub fields:      Vec<ObjectField<Self>>,
    pub left_area:   HintArea,
    pub right_area:  HintArea,
    pub top_area:    HintArea,
    pub bottom_area: HintArea,
}

pub struct WindowGroup {
    pub main:  Window,
    pub scene: Window,
}

impl WindowGroup {
    pub fn copy(self) -> WindowGroup {
        WindowGroup {
            main:  self.main,
            scene: self.scene,
        }
    }
}

impl MainWindow {
    pub fn new(screen_width: f32, screen_height: f32) -> MainWindow{
        MainWindow{
            config: Parameters{
                x:       0.0,
                y:       0.0,
                w:       screen_width,
                h:       screen_height,
                grabbed: false,
                name:    "Main".to_string(),
            },
            buttons: vec![],
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.config.w = w;
        self.config.h = h;
    }
}

impl HintArea {
    pub fn new(side: f32, align: HintAlign) -> HintArea {
        HintArea{
            w: match align {
                HintAlign::LEFT => side,
                HintAlign::RIGHT => side,
                _ => 200.0
            },
            h: match align {
                HintAlign::BOTTOM => side,
                HintAlign::TOP => side,
                _ => 200.0
            },
            align,
            busy:    false,
            visible: false,
        }
    }
}

impl SceneWindow {
    pub fn new(screen_width: f32, screen_height: f32) -> SceneWindow {
        SceneWindow{
            config: Parameters{
                x:       screen_width / 2.0,
                y:       screen_height / 2.0,
                w:       200.0,
                h:       200.0,
                grabbed: false,
                name:    "Scene".to_string(),
            },
            objects:     Vec::new(),
            fields:      Vec::new(),
            left_area:   HintArea::new(50.0, HintAlign::LEFT),
            right_area:  HintArea::new(50.0, HintAlign::RIGHT),
            bottom_area: HintArea::new(50.0, HintAlign::BOTTOM),
            top_area:    HintArea::new(50.0, HintAlign::TOP),
        }
    }
}