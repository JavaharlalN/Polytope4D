#[cfg(target_os = "macos")]
extern crate sapp_darwin as sapp;
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_arch = "wasm32",
    windows
)))]
extern crate sapp_dummy as sapp;
#[cfg(target_os = "linux")]
extern crate sapp_linux as sapp;
#[cfg(target_arch = "wasm32")]
extern crate sapp_wasm as sapp;
#[cfg(windows)]
extern crate sapp_windows as sapp;
mod cursor;
mod angle;
pub mod objects;
pub mod window;
use angle::*;
use macroquad::prelude::Texture2D;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::clear_background;
use macroquad::prelude::Color;
use macroquad::prelude::draw_rectangle_lines;
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
use std::time::Instant;

const CLICK_TIMEOUT: u128 = 150;
const MAX_DIST: f32 = 20.0;

fn draw_windows<'a>(windows: &'a WindowGroup) {
    draw_rectangle(
        windows.main.config.x,
        windows.main.config.y,
        windows.main.config.w,
        windows.main.config.h,
        Color::new(0.3, 0.3, 0.3, 0.5),
    );
}

fn resize_event<'a>(windows: &'a mut WindowGroup) {
    (*windows).main.set_size(screen_width(), screen_height());
}

fn draw_vertices(vertices: Vec<Vec4f>) {
    for v in vertices.into_iter() {
        if let Some(proj) = v.get_proj() {
            if v.selected {
                draw_circle(proj.0, proj.1, 2.0, Color::new(0.0, 0.8, 0.1, 1.0));
            } else {
                draw_circle(proj.0, proj.1, 2.0, Color::new(0.1, 0.1, 0.1, 1.0));
            }
        }
    }
}

fn draw_edges(edges: Vec<(usize, usize)>, obj: &Object) {
    for e in edges.into_iter() {
        let a = obj.vertices[e.0].get_proj().unwrap();
        let b = obj.vertices[e.1].get_proj().unwrap();
        // println!("a: ({}, {}), b: ({}, {})", a.0, a.1, b.0, b.1);
        draw_line(a.0, a.1, b.0, b.1, 1.0, Color::new(0.1, 0.1, 0.1, 1.0));
    }
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, texture: Texture2D, selected: bool) {
    draw_texture(
        texture,
        x,
        y,
        Color::new(1.0, 1.0, 1.0, 1.0)
    );
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        if selected { 6.0 } else { 4.0 },
        Color::new(0.4, 0.4, 0.4, 1.0)
    );
}

fn find_closest_vertice(x: f32, y: f32, vertices: &mut Vec<Vec4f>) -> Option<usize> {
    let mut closest = None;
    let mut min_dist = None;
    for (i, v) in vertices.iter_mut().enumerate() {
        if let Some((px, py)) = v.get_proj() {
            let d = dist2d(x, y, px, py);
            if let Some(min_d) = min_dist {
                if d < min_d { min_dist = Some(d); closest = Some(i)}
            } else { min_dist = Some(d); closest = Some(i) }
        }
    }
    println!("{}\n", closest.unwrap());
    if let Some(d) = min_dist {
        if d < MAX_DIST { closest }
        else { None }
    } else { None }
}

#[macroquad::main("Polytope 4D")]
async fn main() {
    unsafe {
        sapp::sapp_show_mouse(false);
    }
    let selection_type_buttons = vec!(
        Texture2D::from_file_with_format(std::fs::read("sprites/select0.png").unwrap().as_slice(), None),
        Texture2D::from_file_with_format(std::fs::read("sprites/select1.png").unwrap().as_slice(), None),
        Texture2D::from_file_with_format(std::fs::read("sprites/select2.png").unwrap().as_slice(), None),
        Texture2D::from_file_with_format(std::fs::read("sprites/select3.png").unwrap().as_slice(), None),
    );
    let selection_type_id = 0;
    let mut windows = WindowGroup{
        main: MainWindow::new(screen_width(), screen_height()),
        scene: SceneWindow::new(screen_width(), screen_height()),
    };
    let mut cursor = Cursor::new(mouse_position());
    let mut last_size = (screen_width(), screen_height());
    let mut objects = vec![Object::tesseract()];
    let mut angle = Angle::new();
    let mut is_lmb_down = false;
    let mut camera = Camera::new(Vec4f::new(0.0, 0.0, 0.0, -5.0));
    let mut click_timer = Instant::now();
    let (mut x_pos, mut y_pos) = mouse_position();
    let mut selected_vertices: Vec<Vec4f> = vec![];
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
        if is_mouse_button_down(MouseButton::Left) {
            if !is_lmb_down { // lmb down event
                is_lmb_down = true;
                click_timer = Instant::now();
            }
            if click_timer.elapsed().as_millis() >= CLICK_TIMEOUT {
                let x_delta = (x_pos - x_last) / 200.0;
                let y_delta = (y_pos - y_last) / 200.0;
                if is_key_down(KeyCode::LeftShift) {
                    angle.yz += y_delta;
                    angle.xz += x_delta;
                } else {
                    angle.yw += y_delta;
                    angle.xw += x_delta;
                }
                angle.zw += scroll_delta / 100.0;
            }
        } else if is_lmb_down { // lmb up event
            if click_timer.elapsed().as_millis() < CLICK_TIMEOUT { // lmb click event
                for obj in objects.iter_mut() {
                    if let Some(i) = find_closest_vertice(x_pos, y_pos, &mut obj.vertices) {
                        println!("{}", i);
                        if let Some(v) = obj.vertices.get_mut(i) { v.selected = true; }
                    }
                }
            } else {
                for obj in objects.iter_mut() {
                    for v in obj.vertices.iter_mut() {
                        v.freeze(&angle);
                    }
                }
            }
            angle.clear();
            is_lmb_down = false;
        }
        draw_windows(&windows);
        cursor.conf.x = x_pos;
        cursor.conf.y = y_pos;
        let d = dist(Vec4f::new0(), camera.c);
        for obj in (&mut objects).iter_mut() {
            obj.calc_vertices(&angle, d, &windows.main);
            if let Some(local_edges) = &obj.edges {
                draw_edges(local_edges.clone(), obj);
            }
            draw_vertices(obj.vertices.clone());
        }
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
        for i in 0..4 {
            draw_button(
                windows.main.config.w - 114.0 + 28.0 * i as f32,
                0.0,
                30.0,
                30.0,
                selection_type_buttons[i],
                selection_type_id == i,
            );
        }
        next_frame().await
    }
}
