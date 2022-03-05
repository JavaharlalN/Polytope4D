pub use objects::Object;

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

pub struct ObjectField<ParentType> {
    parent: ParentType,
    y:f32,
    object: Object,
}