use super::*;

#[derive(Debug, Copy, Clone)]
pub enum ButtonAlign {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct CheckButton {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    texture: Texture2D,
    hover: bool,
    checked: bool,
    align: ButtonAlign,
    btype: ButtonType,
}

impl CheckButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, sprite: &str, align: ButtonAlign, btype: ButtonType) -> Self {
        Self {
            x, y,
            w, h,
            texture: Texture2D::from_file_with_format(std::fs::read(sprite).unwrap().as_slice(), None),
            hover: false,
            checked: false,
            align,
            btype,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonType {
    Import,
    Export,
    SelectionType,
}

#[derive(Debug, Clone)]
pub struct ClickButton {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    texture: Texture2D,
    hover: bool,
    hold: bool,
    align: ButtonAlign,
    btype: ButtonType,
}

impl ClickButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, sprite: &str, align: ButtonAlign, btype: ButtonType) -> Self {
        Self {
            x, y,
            w, h,
            texture: Texture2D::from_file_with_format(std::fs::read(sprite).unwrap().as_slice(), None),
            hover: false,
            hold: false,
            align,
            btype,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Button {
    Check(CheckButton),
    Click(ClickButton),
}

impl Button {
    pub fn offset(&self) -> (f32, f32) {
        match self {
            Button::Check(btn) => { (btn.x, btn.y) },
            Button::Click(btn) => (btn.x, btn.y),
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Button::Check(btn) => (btn.w, btn.h),
            Button::Click(btn) => (btn.w, btn.h),
        }
    }

    pub fn is_hover(&self) -> bool {
        match self {
            Button::Check(btn) => btn.hover,
            Button::Click(btn) => btn.hover,
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Button::Check(btn) => btn.checked,
            Button::Click(btn) => btn.hold,
        }
    }

    pub fn set_hover(&mut self, h: bool) {
        match self {
            Button::Check(btn) => btn.hover = h,
            Button::Click(btn) => btn.hover = h,
        }
    }

    pub fn set_active(&mut self, a: bool) {
        match self {
            Button::Check(btn) => btn.checked = a,
            Button::Click(btn) => btn.hold = a,
        }
    }

    pub fn texture(&self) -> Texture2D {
        match self {
            Button::Check(btn) => btn.texture,
            Button::Click(btn) => btn.texture,
        }
    }

    pub fn align(&self) -> ButtonAlign {
        match self {
            Button::Check(btn) => btn.align,
            Button::Click(btn) => btn.align,
        }
    }

    pub fn get_type(&self) -> ButtonType {
        match self {
            Button::Check(btn) => btn.clone().btype,
            Button::Click(btn) => btn.clone().btype,
        }
    }

    pub fn get_pos(&self, window: &Window) -> (f32, f32) {
        let (xw, yw) = window.pos();
        let (w, h) = window.size();
        let (xb, yb) = self.offset();
        match self.align() {
            ButtonAlign::TopLeft     => (xw + xb,     yw + yb),
            ButtonAlign::TopRight    => (xw + xb + w, yw + yb),
            ButtonAlign::BottomLeft  => (xw + xb,     yw + yb + h),
            ButtonAlign::BottomRight => (xw + xb + w, yw + yb + h),
        }
    }

    pub fn is_check_button(&self) -> bool {
        match self {
            Button::Check(_) => true,
            _ => false,
        }
    }

    pub fn is_click_button(&self) -> bool {
        match self {
            Button::Click(_) => true,
            _ => false,
        }
    }
}