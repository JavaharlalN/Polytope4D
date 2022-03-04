pub enum ALIGN {
    LEFT,
    CENTER,
    RIGHT,
}

pub struct Button {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    form: String,
}

pub struct TextConstField {
    x: f32,
    y:f32,
    align: ALIGN,
    value: String,
}