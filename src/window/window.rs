use super::ObjectField;
use crate::COMFORTAA;
use crate::button::Align;
use crate::button::Button;
use crate::objects::Object;
use crate::button::ButtonType;
use crate::button::ClickButton;
use macroquad::prelude::Color;
use macroquad::prelude::TextParams;
use macroquad::prelude::measure_text;
use macroquad::prelude::screen_width;
use macroquad::prelude::screen_height;

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
    Start(StartWindow),
}

impl Window {
    pub fn config(&self) -> Parameters {
        match self {
            Window::Main(w) => w.config.clone(),
            Window::Scene(w) => w.config.clone(),
            Window::Start(_) => {
                Parameters {
                    x: 0.0,
                    y: 22.0,
                    w: screen_width(),
                    h: screen_height(),
                    grabbed: false,
                    name: "Start".to_string(),
                }
            }
        }
    }

    pub fn pos(&self) -> (f32, f32) {
        match self {
            Window::Main(w) => (w.config.x, w.config.y),
            Window::Scene(w) => (w.config.x, w.config.y),
            Window::Start(w) => (w.config.x, w.config.y),
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Window::Main(w) => (w.config.w, w.config.h),
            Window::Scene(w) => (w.config.w, w.config.h),
            Window::Start(w) => (w.config.w, w.config.h),
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        match self {
            Window::Main(win) => { win.config.w = w; win.config.h = h },
            Window::Scene(win) => { win.config.w = w; win.config.h = h },
            Window::Start(win) => { win.config.w = w; win.config.h = h },
        }
    }

    pub fn is_hidden(&self) -> bool {
        match self {
            Window::Main(win) => win.hidden,
            Window::Scene(win) => win.hidden,
            Window::Start(win) => win.hidden,
        }
    }

    pub fn set_visibility(&mut self, visible: bool) {
        match self {
            Window::Main(win) => { win.hidden = !visible },
            Window::Scene(win) => { win.hidden = !visible },
            Window::Start(win) => { win.hidden = !visible },
        }
    }

    pub fn buttons(&self) -> Option<&Vec<Button>> {
        match self {
            Window::Main(win) => Some(&win.buttons),
            Window::Scene(_) => None,
            Window::Start(win) => Some(&win.buttons),
        }
    }

    pub fn buttons_mut(&mut self) -> Option<&mut Vec<Button>> {
        match self {
            Window::Main(win) => Some(&mut win.buttons),
            Window::Scene(_) => None,
            Window::Start(win) => Some(&mut win.buttons),
        }
    }

    pub fn hover_i(&self) -> Option<usize> {
        match self {
            Window::Main(win) => {
                for (i, b) in win.buttons.iter().enumerate() {
                    if b.is_hover() { return Some(i) }
                }
                return None
            },
            Window::Scene(_) => None,
            Window::Start(win) => {
                for(i, b) in win.buttons.iter().enumerate() {
                    if b.is_hover() { return Some(i) }
                }
                return None
            }
        }
    }

    pub fn buttons_count(&self) -> usize {
        match self {
            Window::Main(win) => win.buttons.len(),
            Window::Scene(_) => 0,
            Window::Start(win) => win.buttons.len(),
        }
    }

    pub fn clear_hover(&mut self) {
        if self.buttons().is_none() { return; }
        for button in self.buttons_mut().unwrap() {
            button.set_hover(false);
        }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        let (x, y) = self.pos();
        let (w, h) = self.size();
        return (x, y, w, h);
    }

    pub fn show(&mut self) {
        self.set_visibility(true);
    }

    pub fn hide(&mut self) {
        self.set_visibility(false);
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
    pub hidden:  bool,
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
    pub hidden:      bool,
}

pub struct WindowGroup {
    pub main:         Window,
    pub scene:        Window,
    pub start:        Window,
    // pub settings:     OverlappingWindow,
    pub instructions: OverlappingWindow,
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
            hidden: false,
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
                HintAlign::LEFT  => side,
                HintAlign::RIGHT => side,
                _ => 200.0
            },
            h: match align {
                HintAlign::BOTTOM => side,
                HintAlign::TOP    => side,
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
            hidden: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Panel {
    pub buttons: Vec<Button>,
    pub active_button: usize,
}

#[derive(Debug, Clone)]
pub struct Panels {
    left: Option<Panel>,
    top: Option<Panel>,
    right: Option<Panel>,
    bottom: Option<Panel>,
}

impl Panels {
    pub fn empty() -> Self {
        Self {
            left: None,
            top: None,
            right: None,
            bottom: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextItem {
    pub offset: (f32, f32),
    pub size:    u16,
    pub value:   String,
    pub width:   f32,
    pub height:  f32,
    pub color:   Color,
    pub align:   Align,
    pub off_y:   f32,
}

impl TextItem {
    pub fn new(value: &str, offset: (f32, f32), size: u16, color: Color, align: Align) -> Self {
        let dimentions = measure_text(value, Some(*COMFORTAA), size, 1.0);
        println!("{}: {} {} {}", value, dimentions.width, dimentions.height, dimentions.offset_y);
        Self {
            offset,
            size,
            value: value.to_string(),
            width: dimentions.width,
            height: dimentions.height,
            color,
            align,
            off_y: dimentions.offset_y,
        }
    }

    pub fn get_pos(&self, container: Option<(f32, f32, f32, f32)>) -> (f32, f32) {
        let (x, y, w, h) = match container {
            Some((x, y, w, h)) => (x + self.offset.0, y + self.offset.1, w, h),
            None => (self.offset.0, self.offset.1, screen_width(), screen_height()),
        };
        let off = self.off_y;
        let (sw, sh) = (self.width, self.height);
        match self.align {
            Align::Middle       => (x + (w - sw) / 2.0, y + 22.0   + off - (sh + h) / 2.0),
            Align::TopLeft      => (x,                  y + 22.0   + off                 ),
            Align::TopRight     => (x +  w - sw,        y + 22.0   + off                 ),
            Align::TopCenter    => (x + (w - sw) / 2.0, y + 22.0   + off                 ),
            Align::BottomLeft   => (x,                  y + h - sh + off                 ),
            Align::BottomRight  => (x +  w - sw,        y + h - sh + off                 ),
            Align::BottomCenter => (x + (w - sw) / 2.0, y + h - sh + off                 ),
        }
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn get_params(&self) -> TextParams {
        TextParams {
            font: *COMFORTAA,
            font_size: self.size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: self.color,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContentItem {
    H1(TextItem),
    H2(TextItem),
    H3(TextItem),
    Text(TextItem),
    Div((f32, f32), Align, Content),
}

impl ContentItem {
    pub fn header(
        value:   &str,
        offset: (f32, f32),
        level:   usize,
        color:   Color,
        align:   Align,
    ) -> Result<Self, String> {
        match level {
            1 => Ok(Self::H1(TextItem::new(value, offset, 60, color, align))),
            2 => Ok(Self::H2(TextItem::new(value, offset, 48, color, align))),
            3 => Ok(Self::H3(TextItem::new(value, offset, 36, color, align))),
            _ => Err("invalid header level".to_string()),
        }
    }

    pub fn text(value: &str, offset: (f32, f32), color: Color, align: Align) -> Self {
        Self::Text(TextItem::new(value, offset, 24, color, align))
    }

    pub fn div(content: Content, offset: (f32, f32), align: Align) -> Self {
        Self::Div(offset, align, content)
    }

    pub fn get_pos(&self) -> (f32, f32) {
        match self {
            ContentItem::H1(t)   => t.get_pos(None),
            ContentItem::H2(t)   => t.get_pos(None),
            ContentItem::H3(t)   => t.get_pos(None),
            ContentItem::Text(t) => t.get_pos(None),
            ContentItem::Div(pos, _, _) => *pos,
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            ContentItem::H1(t) => t.size(),
            ContentItem::H2(t) => t.size(),
            ContentItem::H3(t) => t.size(),
            ContentItem::Text(t) => t.size(),
            ContentItem::Div(_, _, _) => todo!(),
        }
    }
}

pub type Content = Vec<ContentItem>;

#[derive(Debug, Clone)]
pub struct OverlappingWindow {
    pub panels: Panels,
    pub hidden: bool,
    pub content: Content,
}

impl OverlappingWindow {
    pub fn instructions() -> Result<Self, String> {
        let mut content = Content::new();
        let w = screen_width();
        let hotkeys = vec![
            ("Выделить", "ЛКМ"),
            ("Вращать (XW, YW, ZW)", "ПКМ + <>, ПКМ + КОЛЕСО"),
            ("Вращать (XZ, YZ)", "LSHIFT + ПКМ + <>"),
            ("Экструдировать", "E"),
            ("Соединить вершины", "F"),
            ("Заполнить 2D поверхность", "SHIFT + F"),
            ("Заполнить 3D поверхность", "CTRL + F"),
            ("Копировать", "CTRL + C"),
            ("Вставить", "CTRL + V"),
            ("Дублировать", "CTRL + D"),
            ("Удалить", "DEL"),
        ];
        content.push(ContentItem::header(
            "Polytope 4D",
            (0.0, 0.0),
            1,
            Color::new(0.4, 0.4, 0.4, 1.0),
            Align::TopCenter,
        )?);
        content.push(ContentItem::header(
            "Горячие клавиши",
            (0.0, 70.0),
            3,
            Color::new(0.4, 0.4, 0.4, 1.0),
            Align::TopCenter,
        )?);
        for i in 0..hotkeys.len() {
            content.push(ContentItem::text(
                hotkeys[i].0,
                (-w / 2.0 - 23.448, 130.0 + 30.0 * i as f32),
                Color::new(0.4, 0.4, 0.4, 1.0),
                Align::TopRight,
            ));
            content.push(ContentItem::text(
                " - ",
                (0.0, 130.0 + 30.0 * i as f32),
                Color::new(0.4, 0.4, 0.4, 1.0),
                Align::TopCenter,
            ));
            content.push(ContentItem::text(
                hotkeys[i].1,
                (w / 2.0 + 23.448, 130.0 + 30.0 * i as f32),
                Color::new(0.4, 0.4, 0.4, 1.0),
                Align::TopLeft,
            ));
        }
        Ok(Self {
            panels: Panels::empty(),
            hidden: true,
            content,
        })
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn show(&mut self) {
        self.hidden = false;
    }
}

#[derive(Debug, Clone)]
pub struct StartWindow {
    pub config: Parameters,
    pub buttons: Vec<Button>,
    pub hidden: bool,
    pub content: Content,
}

impl StartWindow {
    pub fn new(w: f32, h: f32) -> Result<Self, String> {
        let mut buttons = vec![];
        buttons.push(Button::Click(ClickButton::new(
            -200.0,
            -100.0,
            100.0,
            100.0,
            Some("sprites/tesseract.png"),
            Align::Middle,
            ButtonType::CreateTesseract,
        )));
        buttons.push(Button::Click(ClickButton::new(
            100.0,
            -100.0,
            100.0,
            100.0,
            Some("sprites/sphere3d.png"),
            Align::Middle,
            ButtonType::CreateSphere3D,
        )));
        buttons.push(Button::Click(ClickButton::new(
            -15.0,
            0.0,
            15.0,
            15.0,
            Some("sprites/exit.png"),
            Align::TopRight,
            ButtonType::Close,
        )));
        let mut content = Content::new();
        content.push(ContentItem::header(
            "Добро пожаловать",
            (0.0, h * 0.1 + 30.0),
            1,
            Color::new(0.4, 0.4, 0.4, 1.0),
            Align::TopCenter,
        )?);
        content.push(ContentItem::header(
            "Создать",
            (0.0, h * 0.1 + 120.0),
            3,
            Color::new(0.4, 0.4, 0.4, 1.0),
            Align::TopCenter,
        )?);
        content.push(ContentItem::text(
            "Тессеракт",
            (-150.0, h * 0.5),
            Color::new(0.4, 0.4, 0.4, 1.0),
            Align::TopCenter,
        ));
        content.push(ContentItem::text(
            "3D сфера",
            (150.0, h * 0.5),
            Color::new(0.4, 0.4, 0.4, 1.0),
            Align::TopCenter,
        ));
        Ok(Self {
            config: Parameters {
                x: w * 0.1,
                y: h * 0.1,
                w: w * 0.8,
                h: h * 0.8,
                grabbed: false,
                name: "Welcome".to_string(),
            },
            buttons,
            hidden: false,
            content,
        })
    }
}