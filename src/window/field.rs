use crate::objects::Object;

pub enum ALIGN {
    LEFT,
    CENTER,
    RIGHT,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub form: String,
}

#[derive(Debug, Clone)]
pub struct RadioButton {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub sprite: Vec<u8>,
    pub rect: bool,
    pub checked: bool,
}

pub struct RadioButtonGroup {
    pub buttons: Vec<RadioButton>,
    pub index: usize,
}

impl RadioButtonGroup {
    pub fn new(buttons: Option<Vec<RadioButton>>) -> Self {
        RadioButtonGroup {
            buttons: if let Some(b) = buttons { b } else { vec![] },
            index: 0,
        }
    }

    pub fn add(&mut self, btn: RadioButton) {
        self.buttons.push(btn);
    }
}

pub struct ObjectField<T> {
    pub parent: T,
    pub y:f32,
    pub object: Object,
}