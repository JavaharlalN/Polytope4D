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
use macroquad::prelude::screen_height;
use macroquad::prelude::mouse_position;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::clear_background;

lazy_static! {
    static ref SCREEN_SIZE: (u64, u64) = rdev::display_size().unwrap();
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

pub fn catch_hover(
    cursor:  &mut Cursor,
    buttons: &mut Vec<Button>,
    hover:   &mut bool,
    hover_i: &mut usize,
    windows: &WindowGroup,
) {
    for i in 0..buttons.len() {
        let (x, y) = buttons[i].get_pos(&windows.main);
        let (w, h) = buttons[i].size();
        *hover = cursor.intersect_with_button(
            buttons.get(i).unwrap(),
            &windows.main,
        );

        if *hover {
            *hover_i = i;
            if !cursor.rect || !buttons[i].is_hover() {
                cursor.set(x, y, w, h + 2.0);
            }
            break;
        }
    }
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

fn update_buttons(buttons: &mut Vec<Button>, hover_i: usize) {
    for i in 0..buttons.len() {
        let btn = buttons.get_mut(i).unwrap();
        btn.set_hover(i == hover_i);
    }
}

#[macroquad::main(window_config)]
async fn main() {
    show_mouse(false);
    let mut buttons = vec![
        Button::Check(CheckButton::new(-120.0, 0.0, 30.0, 30.0, "sprites/select0.png",  Align::TopRight, ButtonType::SelectionType)),
        Button::Check(CheckButton::new( -90.0, 0.0, 30.0, 30.0, "sprites/select1.png",  Align::TopRight, ButtonType::SelectionType)),
        Button::Check(CheckButton::new( -60.0, 0.0, 30.0, 30.0, "sprites/select2.png",  Align::TopRight, ButtonType::SelectionType)),
        Button::Check(CheckButton::new( -30.0, 0.0, 30.0, 30.0, "sprites/select3.png",  Align::TopRight, ButtonType::SelectionType)),
        Button::Click(ClickButton::new(   0.0, 0.0, 20.0, 20.0, "sprites/logo.png",     Align::TopLeft,  ButtonType::Info)),
        Button::Click(ClickButton::new(  20.0, 0.0, 20.0, 20.0, "sprites/settings.png", Align::TopLeft,  ButtonType::Settings)),
        Button::Click(ClickButton::new(  40.0, 0.0, 20.0, 20.0, "sprites/import.png",   Align::TopLeft,  ButtonType::Import)),
        Button::Click(ClickButton::new(  60.0, 0.0, 20.0, 20.0, "sprites/save.png",     Align::TopLeft,  ButtonType::Export)),
    ];
    buttons[0].set_active(true);
    let mut windows = WindowGroup {
        main: Window::Main(MainWindow::new(screen_width(), screen_height())),
        scene: Window::Scene(SceneWindow::new(screen_width(), screen_height())),
        overlapping: vec![OverlappingWindow::instructions().unwrap()],
    };

    let mut mouse_state = MouseState::new(mouse_position(), mouse_wheel().1);
    let mut cursor = Cursor::new(mouse_position());
    let mut last_size = (screen_width(), screen_height());
    let mut objects = vec![Object::tesseract()];
    let mut angle = Angle::new();
    let camera = Camera::new(Vec4f::new(0.0, 0.0, 0.0, -5.0));
    let mut axes = Axes::new(100.0, windows.main.config().y - 100.0);
    let mut motion_axes = MotionAxes::new();
    let mut clipboard = Object::empty();
    loop {
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
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
        let mut hover_i = buttons.len();
        let d = dist(Vec4f::new0(), camera.c);
        catch_hover(&mut cursor, &mut buttons, &mut hover, &mut hover_i, &windows);
        catch_mouse_event(
            &mut mouse_state,
            hover,
            hover_i,
            &mut buttons,
            &mut objects,
            (x_last, y_last),
            &mut motion_axes,
            &mut angle,
            &windows.main,
        );
        catch_keyboard_event(&mut objects, &mut clipboard, &mut motion_axes);
        update_buttons(&mut buttons, hover_i);
        for obj in objects.iter_mut() {
            obj.calc_vertices(&angle, d, &windows.main);
        }
        draw_windows(
            &windows,
            &objects,
            &camera,
            &angle,
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
