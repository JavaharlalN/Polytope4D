mod draw;
mod save;
mod angle;
mod cursor;
mod events;
mod button;
mod import;
mod objects;
mod window;
mod error;
use draw::*;
use angle::*;
use import::*;
use events::*;
use button::*;
use save::save;
use macroquad::prelude::Font;
use macroquad::prelude::TextParams;
use macroquad::prelude::Texture2D;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::clear_background;
use macroquad::prelude::Color;
use macroquad::prelude::draw_text_ex;
use macroquad::prelude::draw_texture;
use macroquad::prelude::screen_width;
use macroquad::prelude::screen_height;
use macroquad::prelude::draw_line;
use macroquad::prelude::draw_poly;
use macroquad::prelude::next_frame;
use macroquad::prelude::draw_circle;
use macroquad::input::*;
use cursor::*;
use objects::*;
use window::*;
use std::env::set_current_dir;
use std::time::Instant;

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
    cursor: &mut Cursor,
    buttons: &mut Vec<Button>,
    hover: &mut bool,
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

#[macroquad::main("Polytope 4D")]
async fn main() {
    show_mouse(false);
    let mut buttons = vec![
        Button::SelectionType(CheckButton::new(-120.0, 0.0, 30.0, 30.0, "sprites/select0.png", ButtonAlign::TopRight)),
        Button::SelectionType(CheckButton::new( -90.0, 0.0, 30.0, 30.0, "sprites/select1.png", ButtonAlign::TopRight)),
        Button::SelectionType(CheckButton::new( -60.0, 0.0, 30.0, 30.0, "sprites/select2.png", ButtonAlign::TopRight)),
        Button::SelectionType(CheckButton::new( -30.0, 0.0, 30.0, 30.0, "sprites/select3.png", ButtonAlign::TopRight)),
        Button::Save(ClickButton::new(0.0, 0.0, 20.0, 20.0, "sprites/save.png", ButtonAlign::TopLeft)),
    ];
    buttons[0].set_active(true);
    let mut windows = WindowGroup {
        main: Window::Main(MainWindow::new(screen_width(), screen_height())),
        scene: Window::Scene(SceneWindow::new(screen_width(), screen_height())),
    };
    let mut cursor = Cursor::new(mouse_position());
    let mut last_size = (screen_width(), screen_height());
    let mut objects = vec![Object::tesseract()];
    let mut angle = Angle::new();
    let mut is_lmb_down = false;
    let mut is_rmb_down = false;
    let camera = Camera::new(Vec4f::new(0.0, 0.0, 0.0, -5.0));
    let mut lmb_click_timer = Instant::now();
    let mut rmb_click_timer = Instant::now();
    let mut cursor_transform_timer = Instant::now();
    let (mut x_pos, mut y_pos) = mouse_position();
    let mut axes = Axes::new(100.0, windows.main.config().y - 100.0);
    let mut motion_axes = MotionAxes::new();
    let mut clipboard = Object::empty();
    // let mut selected_vertices: Vec<Vec4f> = vec![];
    loop {
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
        let scroll_delta = mouse_wheel().1;
        let x_last = x_pos;
        let y_last = y_pos;
        match mouse_position() { (x, y) => { x_pos = x; y_pos = y } }
        let new_size = (screen_width(), screen_height());
        if new_size != last_size {
            resize_event(&mut windows);
            last_size = new_size;
        }
        let mut hover = false;
        let mut hover_i = buttons.len();
        catch_hover(&mut cursor, &mut buttons, &mut hover, &mut hover_i, &windows);
        catch_mouse_event(
            &mut is_lmb_down,
            &mut is_rmb_down,
            scroll_delta,
            &mut lmb_click_timer,
            &mut rmb_click_timer,
            hover,
            hover_i,
            &mut buttons,
            &mut objects,
            (x_pos, y_pos),
            (x_last, y_last),
            &mut motion_axes,
            &mut angle,
            &windows.main,
        );
        catch_keyboard_event(&mut objects, &mut clipboard, &mut motion_axes);
        draw_windows(&windows);
        cursor.move_to(x_pos, y_pos);
        let d = dist(Vec4f::new0(), camera.c);
        for obj in (&mut objects).iter_mut() {
            obj.calc_vertices(&angle, d, &windows.main);
            draw_edges(obj);
            if buttons.get(0).unwrap().is_active() {
                draw_vertices(obj.vertices.clone());
            }
        }
        axes.calc(&angle, &windows.main);
        motion_axes.calc(&angle, &windows.main);
        draw_axes(&axes, windows.main.config().w, windows.main.config().h);
        draw_motion_axes(&motion_axes);
        draw_cursor(&cursor);
        if !hover { cursor.reset(); }
        for i in 0..buttons.len() {
            let btn = buttons.get_mut(i).unwrap();
            btn.set_hover(i == hover_i);
            draw_button(
                btn,
                &windows.main,
            );
        }

        if cursor_transform_timer.elapsed().as_millis() >= CUR_TRANSFORM_TO {
            cursor_transform_timer = Instant::now();
            cursor.next();
            // println!("{}", cursor.conf.w);
        }
        draw_cursor_overlay(cursor);
        next_frame().await
    }
}
