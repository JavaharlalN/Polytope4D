use super::{ObjectField, Button};
use crate::objects::Object;

#[derive(Debug, Copy, Clone)]
pub enum HintAlign {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

#[derive(Debug, Copy, Clone)]
pub enum WINDOWS {
    MAIN = 0,
    SCENE = 1,
}

#[derive(Debug, Copy, Clone)]
pub struct HintArea {
    pub w: f32,
    pub h: f32,
    pub align: HintAlign,
    pub window: u16,
    pub busy: bool,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub grabbed: bool,
    pub name: String,
    pub id: u8,
}

#[derive(Debug, Clone)]
pub struct MainWindow {
    pub config: Parameters,
    pub buttons: Vec<Button>,
}

pub struct SceneWindow {
    pub config: Parameters,
    pub objects: Vec<Object>,
    pub fields: Vec<ObjectField<Self>>,
    pub left_area: HintArea,
    pub right_area: HintArea,
    pub top_area: HintArea,
    pub bottom_area: HintArea,
}

pub struct WindowGroup {
    pub main: MainWindow,
    pub scene: SceneWindow,
}

impl WindowGroup {
    pub fn copy(self) -> WindowGroup {
        WindowGroup {
            main: self.main,
            scene: self.scene,
        }
    }
}

impl MainWindow {
    pub fn new(screen_width: f32, screen_height: f32) -> MainWindow{
        MainWindow{
            config: Parameters{
                x: 0.0,
                y: 0.0,
                w: screen_width,
                h: screen_height,
                grabbed: false,
                name: "Main".to_string(),
                id: WINDOWS::MAIN as u8,
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
    pub fn new(side: f32, win_type: WINDOWS, align: HintAlign) -> HintArea {

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
            window: win_type as u16,
            busy: false,
            visible: false,
        }
    }
}

impl SceneWindow {
    pub fn new(screen_width: f32, screen_height: f32) -> SceneWindow {
        SceneWindow{
            config: Parameters{
                x: screen_width / 2.0,
                y: screen_height / 2.0,
                w: 200.0,
                h: 200.0,
                grabbed: false,
                name: "Scene".to_string(),
                id: WINDOWS::SCENE as u8,
            },
            objects: Vec::new(),
            fields: Vec::new(),
            left_area: HintArea::new(50.0, WINDOWS::SCENE, HintAlign::LEFT),
            right_area: HintArea::new(50.0, WINDOWS::SCENE, HintAlign::RIGHT),
            bottom_area: HintArea::new(50.0, WINDOWS::SCENE, HintAlign::BOTTOM),
            top_area: HintArea::new(50.0, WINDOWS::SCENE, HintAlign::TOP),
        }
    }
}