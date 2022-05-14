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
}

impl CheckButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, sprite: &str, align: ButtonAlign) -> Self {
        Self {
            x, y,
            w, h,
            texture: Texture2D::from_file_with_format(std::fs::read(sprite).unwrap().as_slice(), None),
            hover: false,
            checked: false,
            align,
        }
    }
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
}

impl ClickButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, sprite: &str, align: ButtonAlign) -> Self {
        Self {
            x, y,
            w, h,
            texture: Texture2D::from_file_with_format(std::fs::read(sprite).unwrap().as_slice(), None),
            hover: false,
            hold: false,
            align,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Button {
    SelectionType(CheckButton),
    Save(ClickButton)
}

impl Button {
    pub fn offset(&self) -> (f32, f32) {
        match self {
            Button::SelectionType(btn) => { (btn.x, btn.y) },
            Button::Save(btn) => { (btn.x, btn.y) },
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Button::SelectionType(btn) => { (btn.w, btn.h) },
            Button::Save(btn) => { (btn.w, btn.h) },
        }
    }

    pub fn is_hover(&self) -> bool {
        match self {
            Button::SelectionType(btn) => { btn.hover },
            Button::Save(btn) => { btn.hover },
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Button::SelectionType(btn) => { btn.checked },
            Button::Save(btn) => { btn.hold }
        }
    }

    pub fn set_hover(&mut self, h: bool) {
        match self {
            Button::SelectionType(btn) => { btn.hover = h },
            Button::Save(btn) => { btn.hover = h },
        }
    }

    pub fn set_active(&mut self, a: bool) {
        match self {
            Button::SelectionType(btn) => { btn.checked = a },
            Button::Save(btn) => { btn.hold = a },
        }
    }

    pub fn texture(&self) -> Texture2D {
        match self {
            Button::SelectionType(btn) => { btn.texture },
            Button::Save(btn) => { btn.texture },
        }
    }

    pub fn align(&self) -> ButtonAlign {
        match self {
            Button::SelectionType(btn) => { btn.align },
            Button::Save(btn) => { btn.align },
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
            Button::SelectionType(_) => true,
            Button::Save(_) => false,
        }
    }

    pub fn is_click_button(&self) -> bool {
        match self {
            Button::SelectionType(_) => false,
            Button::Save(_) => true,
        }
    }
}