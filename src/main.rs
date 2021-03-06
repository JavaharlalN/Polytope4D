mod save;
mod draw;
mod angle;
mod error;
mod button;
mod import;
mod window;
mod cursor;
mod events;
mod objects;
use draw::*;
use angle::*;
use import::*;
use events::*;
use button::*;
use cursor::*;
use window::*;
use objects::*;
use save::save;
use std::time::Instant;
use lazy_static::lazy_static;
use macroquad::prelude::Conf;
use macroquad::prelude::Font;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;
use macroquad::prelude::draw_line;
use macroquad::prelude::draw_poly;
use macroquad::prelude::show_mouse;
use macroquad::prelude::TextParams;
use macroquad::prelude::next_frame;
use macroquad::prelude::draw_circle;
use macroquad::prelude::mouse_wheel;
use macroquad::prelude::draw_text_ex;
use macroquad::prelude::draw_texture;
use macroquad::prelude::screen_width;
use macroquad::prelude::load_ttf_font;
use macroquad::prelude::screen_height;
use macroquad::prelude::mouse_position;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::clear_background;

lazy_static! {
    static ref SCREEN_SIZE: (u64, u64) = rdev::display_size().unwrap();
    static ref COMFORTAA_BOLD: Font = {
        let future = load_ttf_font("fonts/Comfortaa-Bold.ttf");
        tokio::runtime::Runtime::new().unwrap().block_on(future).unwrap()
    };
    static ref COMFORTAA_SEMIBOLD: Font = {
        let future = load_ttf_font("fonts/Comfortaa-SemiBold.ttf");
        tokio::runtime::Runtime::new().unwrap().block_on(future).unwrap()
    };
    static ref COMFORTAA: Font = {
        let future = load_ttf_font("fonts/Comfortaa-Medium.ttf");
        tokio::runtime::Runtime::new().unwrap().block_on(future).unwrap()
    };
    static ref COMFORTAA_REGULAR: Font = {
        let future = load_ttf_font("fonts/Comfortaa-Regular.ttf");
        tokio::runtime::Runtime::new().unwrap().block_on(future).unwrap()
    };
    static ref COMFORTAA_LIGHT: Font = {
        let future = load_ttf_font("fonts/Comfortaa-Light.ttf");
        tokio::runtime::Runtime::new().unwrap().block_on(future).unwrap()
    };
}

fn find_closest_vertice(x: f32, y: f32, vertices: &Vec<Vec4f>) -> Option<usize> {
    let mut closest = None;
    let mut min_dist = None;
    for (i, v) in vertices.iter().enumerate() {
        if let Some(proj) = v.get_proj() {
            let d = dist2d((x, y), proj);
            if let Some(min_d) = min_dist {
                if d < min_d { min_dist = Some(d); closest = Some(i)}
            } else { min_dist = Some(d); closest = Some(i) }
        }
    }
    if let Some(d) = min_dist {
        if d < MAX_DIST { closest }
        else { None }
    } else { None }
}

fn dist_to_edge(x: f32, y: f32, pair: &Edge, vertices: &Vec<Vec4f>) -> Option<f32> {
    if let (Some(a), Some(b)) = (vertices[pair.a].get_proj(), vertices[pair.b].get_proj()) {
        let d1 = dist2d(a, b);
        let d2 = dist2d(a, (x, y));
        let d3 = dist2d((x, y), b);
        if d2.powi(2) > d1.powi(2) + d3.powi(2) { return None }
        if d3.powi(2) > d1.powi(2) + d2.powi(2) { return None }
        let s = (d1 + d2 + d3) / 2.0;
        let heron = (s * (s - d1) * (s - d2) * (s - d3)).sqrt();
        Some(heron / d1 * 2.0)
    } else { None }
}

fn clear_selection(object: &mut Object) {
    for v in &mut object.vertices {
        v.selected = false;
    }
    for e in &mut object.edges {
        e.selected = false;
    }
    for f in &mut object.faces {
        f.2 = false;
    }
    for c in &mut object.cells {
        c.3 = false;
    }
}

pub fn get_enabled_buttons_count(buttons: &Vec<Button>) -> i32 {
    let mut counter = 0;
    for b in buttons {
        if b.is_active() { counter += 1; }
    }
    counter
}

pub fn clear_hover(buttons: &mut Vec<Button>, windows: &mut WindowGroup) {
    for button in buttons.iter_mut() {
        button.set_hover(false);
    }
    windows.main.clear_hover();
    windows.scene.clear_hover();
    windows.start.clear_hover();
}

pub fn catch_hover_for_window(
    cursor: &mut Cursor,
    hover:  &mut bool,
    window: &mut Window,
) -> bool { // hovered
    if window.is_hidden() { return false }
    let geometry = window.as_tuple();
    match window.buttons_mut() {
        Some(buttons) => {
            for button in buttons {
                let (x, y) = button.get_pos(Some(geometry));
                let (w, h) = button.size();
                if cursor.intersect_with_button(button, Some(geometry), None) {
                    if !cursor.rect || !cursor.is_pos_set(x, y) {
                        cursor.set(x, y, w, h + 2.0);
                    }
                    button.set_hover(true);
                    *hover = true;
                    return true;
                }
            }
            return false;
        },
        None => { return false },
    }
}

pub fn catch_hover(
    cursor:  &mut Cursor,
    buttons: &mut Vec<Button>,
    hover:   &mut bool,
    windows: &mut WindowGroup,
) {
    clear_hover(buttons, windows);
    *hover = false;
    for button in buttons.iter_mut() {
        let (x, y) = button.get_pos(None);
        let (w, h) = button.size();
        if cursor.intersect_with_button(
            button,
            None,
            None,
        ) {
            *hover = true;
            if !cursor.rect || !cursor.is_pos_set(x, y) {
                cursor.set(x, y, w, h + 2.0);
            }
            button.set_hover(true);
            return;
        }
    }
    if catch_hover_for_window(cursor, hover, &mut windows.main) { return }
    if catch_hover_for_window(cursor, hover, &mut windows.start) { return }
}

pub struct MouseState {
    pub is_lmb_down:            bool,
    pub is_rmb_down:            bool,
    pub lmb_click_timer:        Instant,
    pub rmb_click_timer:        Instant,
    pub cursor_transform_timer: Instant,
    pub pos:                   (f32, f32),
    pub scroll_delta:           f32,
}

impl MouseState {
    pub fn new(pos: (f32, f32), scroll_delta: f32) -> MouseState {
        MouseState {
            is_lmb_down:            false,
            is_rmb_down:            false,
            lmb_click_timer:        Instant::now(),
            rmb_click_timer:        Instant::now(),
            cursor_transform_timer: Instant::now(),
            pos,
            scroll_delta,
        }
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Polytope 4D".to_string(),
        window_width:  SCREEN_SIZE.0 as i32,
        window_height: SCREEN_SIZE.1 as i32,
        ..Default::default()
    }
}

fn update_buttons(windows: &mut WindowGroup) {
    let hover = windows.main.hover_i();
    match windows.main.buttons_mut() {
        Some(buttons) => {
            match hover {
                Some(hover_i) => {
                    for (i, button) in buttons.iter_mut().enumerate() {
                        button.set_hover(i == hover_i);
                    }
                },
                None => {},
            }
        }
        None => {},
    }
    let hover = windows.start.hover_i();
    match windows.start.buttons_mut() {
        Some(buttons) => {
            match hover {
                Some(hover_i) => {
                    for (i, button) in buttons.iter_mut().enumerate() {
                        button.set_hover(i == hover_i);
                    }
                }
                None => {},
            }
        },
        None => {},
    }
}

#[macroquad::main(window_config)]
async fn main() {
    show_mouse(false);
    let mut buttons = vec![
        Button::Click(ClickButton::new( 0.0, 0.0, 20.0, 20.0, Some("sprites/logo.png"),     Align::TopLeft,  ButtonType::Info)),
        Button::Click(ClickButton::new(20.0, 0.0, 20.0, 20.0, Some("sprites/settings.png"), Align::TopLeft,  ButtonType::Settings)),
        Button::Click(ClickButton::new(40.0, 0.0, 20.0, 20.0, Some("sprites/import.png"),   Align::TopLeft,  ButtonType::Import)),
        Button::Click(ClickButton::new(60.0, 0.0, 20.0, 20.0, Some("sprites/save.png"),     Align::TopLeft,  ButtonType::Export)),
    ];
    let mut windows = WindowGroup {
        main:         Window::Main(MainWindow::new(screen_width(), screen_height())),
        scene:        Window::Scene(SceneWindow::new(screen_width(), screen_height())),
        start:        Window::Start(StartWindow::new(screen_width(), screen_height()).unwrap()),
        instructions: OverlappingWindow::instructions().unwrap(),
    };
    if let Window::Main(main) = &mut windows.main {
        main.buttons = vec![
            Button::Check(CheckButton::new(-120.0, 0.0, 30.0, 30.0, "sprites/select0.png",  Align::TopRight, ButtonType::SelectionType)),
            Button::Check(CheckButton::new( -90.0, 0.0, 30.0, 30.0, "sprites/select1.png",  Align::TopRight, ButtonType::SelectionType)),
            Button::Check(CheckButton::new( -60.0, 0.0, 30.0, 30.0, "sprites/select2.png",  Align::TopRight, ButtonType::SelectionType)),
            Button::Check(CheckButton::new( -30.0, 0.0, 30.0, 30.0, "sprites/select3.png",  Align::TopRight, ButtonType::SelectionType)),
        ];
        main.buttons[0].set_active(true);
    }
    windows.start.show();
    windows.main.hide();
    let mut objects = vec![Object::tesseract()];
    let mut mouse_state = MouseState::new(mouse_position(), mouse_wheel().1);
    let mut cursor = Cursor::new(mouse_position());
    let mut last_size = (screen_width(), screen_height());
    let mut angle = Angle::new();
    let camera = Camera::new(Vec4f::new(0.0, 0.0, 0.0, -5.0));
    let mut axes = Axes::new(100.0, windows.main.config().y - 100.0);
    let mut motion_axes = MotionAxes::new();
    let mut clipboard = Object::empty();
    loop {
        clear_background(Color::new(0.55294, 0.55294, 0.55294, 1.0));
        mouse_state.scroll_delta = mouse_wheel().1;
        let x_last = mouse_state.pos.0;
        let y_last = mouse_state.pos.1;
        mouse_state.pos = mouse_position();

        let new_size = (screen_width(), screen_height());
        if new_size != last_size {
            resize_event(&mut windows);
            last_size = new_size;
        }
        let mut hover = false;
        let d = dist(Vec4f::new0(), camera.c);
        catch_hover(&mut cursor, &mut buttons, &mut hover, &mut windows);
        catch_mouse_event(
            &mut mouse_state,
            &mut buttons,
            &mut objects,
            (x_last, y_last),
            &mut motion_axes,
            &mut angle,
            &mut windows,
        );
        catch_keyboard_event(&mut objects, &mut clipboard, &mut motion_axes);
        update_buttons(&mut windows);
        for obj in objects.iter_mut() {
            obj.calc_vertices(&angle, d, &windows.main);
        }
        draw_windows(
            &windows,
            &objects,
            &buttons,
            &axes,
            &motion_axes,
            &cursor,
        );
        if !hover { cursor.reset(); }
        cursor.move_to(mouse_state.pos.0, mouse_state.pos.1);
        axes.calc(&angle, &windows.main);
        motion_axes.calc(&angle, &windows.main);

        if mouse_state.cursor_transform_timer.elapsed().as_millis() >= CUR_TRANSFORM_TO {
            mouse_state.cursor_transform_timer = Instant::now();
            cursor.next();
            // println!("{}", cursor.conf.w);
        }
        draw_cursor_overlay(cursor);
        next_frame().await
    }
}
