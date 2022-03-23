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
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::clear_background;
use macroquad::prelude::Color;
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

fn draw_windows<'a>(windows: &'a WindowGroup) {
    draw_rectangle(
        windows.main.config.x,
        windows.main.config.y,
        windows.main.config.w,
        windows.main.config.h,
        Color::new(0.3, 0.3, 0.3, 0.5),
    );
}

fn draw_bg() {
    clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
    for i in 0..(screen_width() as i32 / 50) {
        for k in 0..(screen_height() as i32 / 50) {
            draw_line(
                (i * 50) as f32,
                0.0,
                (i * 50) as f32,
                screen_height(),
                1.0,
                Color::new(0.25, 0.25, 0.25, 1.0),
            );
            draw_line(
                0.0,
                (k * 50) as f32,
                screen_width(),
                (k * 50) as f32,
                1.0,
                Color::new(0.25, 0.25, 0.25, 1.0),
            );
        }
    }
}

fn resize_event<'a>(windows: &'a mut WindowGroup) {
    (*windows).main.set_size(screen_width(), screen_height());
}

fn draw_vertices(vertices: Vec<Vec4f>) {
    for v in vertices.into_iter() {
        let proj = v.get_proj();
        draw_circle(proj.0, proj.1, 2.0, Color::new(0.1, 0.1, 0.1, 1.0));
    }
}

fn draw_edges(edges: Vec<(usize, usize)>, obj: &Object) {
    for e in edges.into_iter() {
        let a = obj.vertices[e.0].get_proj();
        let b = obj.vertices[e.1].get_proj();
        // println!("a: ({}, {}), b: ({}, {})", a.0, a.1, b.0, b.1);
        draw_line(a.0, a.1, b.0, b.1, 1.0, Color::new(0.1, 0.1, 0.1, 1.0));
    }
}

#[macroquad::main("Polytope 4D")]
async fn main() {
    unsafe {
        sapp::sapp_show_mouse(false);
    }
    let mut windows = WindowGroup{
        main: MainWindow::new(screen_width(), screen_height()),
        scene: SceneWindow::new(screen_width(), screen_height()),
    };
    let mut cursor = Cursor::new(mouse_position());
    let mut last_size = (screen_width(), screen_height());
    let mut objects = vec![Object::tesseract()];
    let mut angle = Angle::new();
    let mut camera = Camera::new(Vec4f::new(0.0, 0.0, 0.0, -5.0));
    loop {
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));
        let (x_pos, y_pos) = mouse_position();
        let new_size = (screen_width(), screen_height());
        if new_size != last_size {
            resize_event(&mut windows);
            last_size = new_size;
        }
        draw_windows(&windows);
        cursor.conf.x = x_pos;
        cursor.conf.y = y_pos;
        angle.xz += 0.01;
        // let mut scene_vertices: Vec<Vec4f> = Vec::new();
        let d = dist(Vec4f::new0(), camera.c);
        for obj in (&mut objects).iter_mut() {
            obj.calc_vertices(&angle, d, &windows.main);
            if let Some(local_edges) = &obj.edges {
                draw_edges(local_edges.clone(), obj);
            }
            draw_vertices(obj.vertices.clone());
        }
        // draw_vertices(scene_vertices);
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
        next_frame().await
    }
}
