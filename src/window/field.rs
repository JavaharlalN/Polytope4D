use crate::objects::Object;

pub enum ALIGN {
    LEFT,
    CENTER,
    RIGHT,
}

#[derive(Debug, Clone)]
pub struct ObjectField<T> {
    pub parent: T,
    pub y:f32,
    pub object: Object,
}