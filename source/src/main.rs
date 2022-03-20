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

const SCALE: f32 = 3000.0;

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

fn calc_vertice(v: Vec4f, a: &Angle, d: f32) -> Vec4f {
    let rotated = v.rotated_xy(&a.xy)
                           .rotated_xz(&a.xz)
                           .rotated_xw(&a.xw)
                           .rotated_yz(&a.yz)
                           .rotated_yw(&a.yw)
                           .rotated_zw(&a.zw);
    let w = 1.0 / (d - rotated.w);
    let x = rotated.x * w;
    let y = rotated.y * w;
    let z = rotated.z * w;
    let proj3d = (x, y, z);
    let z = 1.0 / (d - rotated.w - proj3d.2) * SCALE;
    let x = proj3d.0 * z + screen_width() / 2.0;
    let y = proj3d.1 * z + screen_height() / 2.0;
    v.with_proj((x, y))
}

fn draw_vertices(vertices: Vec<Vec4f>, a: &Angle, d: f32) {
    for v in vertices.into_iter() {
        let projected = calc_vertice(v, a, d);
        let proj = projected.get_proj();
        draw_circle(proj.0, proj.1, 2.0, Color::new(0.1, 0.1, 0.1, 1.0));
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
        clear_background(Color::new(0.6, 0.6, 0.6, 1.0));
        let (x_pos, y_pos) = mouse_position();
        let new_size = (screen_width(), screen_height());
        if new_size != last_size {
            resize_event(&mut windows);
            last_size = new_size;
        }
        draw_windows(&windows);
        cursor.conf.x = x_pos;
        cursor.conf.y = y_pos;
        let mut scene_vertices: Vec<Vec4f> = Vec::new();
        for obj in (&objects).into_iter() {
            for v in &obj.vertices {
                scene_vertices.push(v.clone());
            }
        }
        draw_vertices(scene_vertices, &angle, dist(Vec4f::new0(), camera.c));
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
        next_frame().await
    }
}
