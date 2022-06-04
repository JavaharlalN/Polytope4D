use super::*;

#[derive(Debug, Copy, Clone)]
pub enum Align {
    Middle,
    TopLeft,
    TopRight,
    TopCenter,
    BottomLeft,
    BottomRight,
    BottomCenter,
}

#[derive(Debug, Clone)]
pub struct CheckButton {
    x:       f32,
    y:       f32,
    w:       f32,
    h:       f32,
    texture: Texture2D,
    hover:   bool,
    checked: bool,
    align:   Align,
    btype:   ButtonType,
}

impl CheckButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, sprite: &str, align: Align, btype: ButtonType) -> Self {
        Self {
            x, y,
            w, h,
            texture: Texture2D::from_file_with_format(std::fs::read(sprite).unwrap().as_slice(), None),
            hover:   false,
            checked: false,
            align,
            btype,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonType {
    CreateTesseract,
    CreateSphere3D,
    SelectionType,
    Settings,
    Import,
    Export,
    Close,
    Info,
}

#[derive(Debug, Clone)]
pub struct ClickButton {
    x:       f32,
    y:       f32,
    w:       f32,
    h:       f32,
    texture: Texture2D,
    hover:   bool,
    hold:    bool,
    align:   Align,
    btype:   ButtonType,
}

impl ClickButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, sprite: Option<&str>, align: Align, btype: ButtonType) -> Self {
        let texture = match sprite {
            Some(path) => Texture2D::from_file_with_format(std::fs::read(path).unwrap().as_slice(), None),
            None => Texture2D::empty(),
        };
        Self {
            x, y,
            w, h,
            texture,
            hover:   false,
            hold:    false,
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

    pub fn align(&self) -> Align {
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

    pub fn get_pos(&self, window: Option<(f32, f32, f32, f32)>) -> (f32, f32) {
        let (xw, yw) = match window {
            Some(win) => (win.0, win.1),
            None => (0.0, 0.0),
        };
        let (w, h) = match window {
            Some(win) => (win.2, win.3),
            None => (screen_width(), screen_height()),
        };
        let (xb, yb) = self.offset();
        match self.align() {
            Align::Middle       => (xw + xb + w / 2.0, yw + yb + h / 2.0),
            Align::TopLeft      => (xw + xb,           yw + yb          ),
            Align::TopRight     => (xw + xb + w,       yw + yb          ),
            Align::TopCenter    => (xw + xb + w / 2.0, yw + yb          ),
            Align::BottomLeft   => (xw + xb,           yw + yb + h      ),
            Align::BottomRight  => (xw + xb + w,       yw + yb + h      ),
            Align::BottomCenter => (xw + xb + w,       yw + yb + h / 2.0),
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