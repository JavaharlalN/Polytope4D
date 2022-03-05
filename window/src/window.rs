use super::*;
use objects::Object;

pub enum HINT_ALIGN {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

pub enum WINDOWS {
    MAIN = 0,
    SCENE = 1,
}

struct HintArea {
    w: f32,
    h: f32,
    align: HINT_ALIGN,
    window: u16,
    busy: bool,
    visible: bool,
}

struct Parameters {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    grabbed: bool,
    name: String,
    id: u16,
}

pub struct MainWindow {
    config: Parameters,
}

pub struct SceneWindow {
    config: Parameters,
    objects: Vec<Object>,
    fields: Vec<ObjectField<Self>>,
    left_area: HintArea,
    right_area: HintArea,
    top_area: HintArea,
    bottom_area: HintArea,
}

impl MainWindow {
    pub fn new(screen_width: f32, screen_height: f32) -> MainWindow{
        MainWindow{
            config: Parameters{
                x: screen_width / 2.0,
                y: screen_height / 2.0,
                w: 200.0,
                h: 200.0,
                grabbed: false,
                name: "Main".to_string(),
                id: WINDOWS::MAIN as u16,
            }
        }
    }
}

impl HintArea {
    pub fn new(side: f32, win_type: WINDOWS, align: HINT_ALIGN) -> HintArea {

        HintArea{
            w: match align {
                HINT_ALIGN::LEFT => side,
                HINT_ALIGN::RIGHT => side,
                _ => 200.0
            },
            h: match align {
                HINT_ALIGN::BOTTOM => side,
                HINT_ALIGN::TOP => side,
                _ => 200.0
            },
            align: align,
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
                id: WINDOWS::SCENE as u16,
            },
            objects: Vec::new(),
            fields: Vec::new(),
            left_area: HintArea::new(50.0, WINDOWS::SCENE, HINT_ALIGN::LEFT),
            right_area: HintArea::new(50.0, WINDOWS::SCENE, HINT_ALIGN::RIGHT),
            bottom_area: HintArea::new(50.0, WINDOWS::SCENE, HINT_ALIGN::BOTTOM),
            top_area: HintArea::new(50.0, WINDOWS::SCENE, HINT_ALIGN::TOP),
        }
    }
}