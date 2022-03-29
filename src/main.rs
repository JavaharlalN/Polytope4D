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
use macroquad::prelude::Font;
use macroquad::prelude::TextParams;
use macroquad::prelude::Texture2D;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::clear_background;
use macroquad::prelude::Color;
use macroquad::prelude::draw_rectangle_lines;
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
use std::time::Instant;

const CLICK_TIMEOUT: u128 = 150;
const CUR_TRANSFORM_TO: u128 = 20;
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
                draw_circle(proj.0, proj.1, 3.0, Color::new(0.0, 0.2, 0.4, 1.0));
                draw_circle(proj.0, proj.1, 2.0, Color::new(0.0, 0.6, 1.0, 1.0));
            } else {
                draw_circle(proj.0, proj.1, 2.0, Color::new(0.1, 0.1, 0.1, 1.0));
            }
        }
    }
}

fn draw_edges(obj: &Object) {
    for e in (&obj.edges).into_iter() {
        let a = obj.vertices[e.0].get_proj().unwrap();
        let b = obj.vertices[e.1].get_proj().unwrap();
        // println!("a: ({}, {}), b: ({}, {})", a.0, a.1, b.0, b.1);
        if e.2 {
            draw_line(a.0, a.1, b.0, b.1, 2.0, Color::new(0.1, 0.2, 0.4, 1.0));
            draw_line(a.0, a.1, b.0, b.1, 1.0, Color::new(0.1, 0.6, 1.0, 1.0));
        } else {
            draw_line(a.0, a.1, b.0, b.1, 1.0, Color::new(0.1, 0.1, 0.1, 1.0));
        }
    }
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, texture: Texture2D, selected: bool, hover: bool) {
    draw_texture(
        texture,
        x,
        y,
        Color::new(1.0, 1.0, 1.0, 1.0)
    );
    let thickness = if selected { 6.0 } else { 4.0 };
    draw_rectangle_lines(
        x, y, w, h,
        thickness,
        Color::new(0.4, 0.4, 0.4, 1.0)
    );

    if hover {
        draw_rectangle_lines(
            x + thickness / 2.0, y + thickness / 2.0,
            w - thickness, h - thickness,
            2.0,
            Color::new(0.3, 0.3, 0.3, 1.0)
        );
    }
}

fn find_closest_vertice(x: f32, y: f32, vertices: &Vec<Vec4f>) -> Option<usize> {
    let mut closest = None;
    let mut min_dist = None;
    for (i, v) in vertices.iter().enumerate() {
        if let Some((px, py)) = v.get_proj() {
            let d = dist2d(x, y, px, py);
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

fn dist_to_edge(x: f32, y: f32, pair: &(usize, usize, bool), vertices: &Vec<Vec4f>) -> Option<f32> {
    if let (Some(a), Some(b)) = (vertices[pair.0].get_proj(), vertices[pair.1].get_proj()) {
        let d1 = dist2d(a.0, a.1, b.0, b.1);
        let d2 = dist2d(a.0, a.1, x, y);
        let d3 = dist2d(x, y, b.0, b.1);
        if d2.powi(2) > d1.powi(2) + d3.powi(2) { return None }
        if d3.powi(2) > d1.powi(2) + d2.powi(2) { return None }
        let s = (d1 + d2 + d3) / 2.0;
        let heron = (s * (s - d1) * (s - d2) * (s - d3)).sqrt();
        Some(heron / d1 * 2.0)
    } else { None }
}

fn find_closest_edge(x: f32, y: f32, obj: &Object) -> Option<usize> {
    let mut closest = None;
    let mut min_dist = None;
    for (i, e) in obj.edges.iter().enumerate() {
        if let Some(d) = dist_to_edge(x, y, e, &obj.vertices) {
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

fn clear_selection_vertices(vertices: &mut Vec<Vec4f>, index: usize) {
    for (i, v) in vertices.iter_mut().enumerate() {
        v.selected = index == i;
    }
}

fn clear_selection_edges(edges: &mut Vec<(usize, usize, bool)>, index: usize) {
    for (i, e) in edges.iter_mut().enumerate() {
        e.2 = index == i;
    }
}

fn draw_cursor(cursor: &Cursor) {
    // println!("{}", cursor.rect);
    if cursor.rect {
        draw_rectangle(cursor.conf.x, cursor.conf.y, cursor.conf.w, cursor.conf.h, Color::new(0.3, 0.3, 0.3, 1.0));
    } else {
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }
}

fn get_buttons_enabled_count(buttons: &Vec<(Texture2D, bool, bool)>) -> i32 {
    let mut counter = 0;
    for b in buttons {
        if b.1 { counter += 1; }
    }
    counter
}

fn lmb_click_event(
    hover: bool,
    selection_type_buttons: &mut Vec<(Texture2D, bool, bool)>,
    hover_i: usize,
    objects: &mut Vec<Object>,
    xy: (f32, f32),
) {
    if hover {
        if is_key_down(KeyCode::LeftShift) {
            if get_buttons_enabled_count(selection_type_buttons) > 1 {
                selection_type_buttons[hover_i].1 = !selection_type_buttons[hover_i].1;
            } else if !selection_type_buttons[hover_i].1 {
                selection_type_buttons[hover_i].1 = true;
            }
        } else {
            for b in selection_type_buttons.iter_mut() {
                b.1 = false
            }
            selection_type_buttons[hover_i].1 = true;
        }
    }
    for obj in objects.iter_mut() {
        if selection_type_buttons[0].1 {
            if let Some(index) = find_closest_vertice(xy.0, xy.1, &obj.vertices) {
                let v = obj.vertices.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) { v.selected = !v.selected; }
                else { clear_selection_vertices(&mut obj.vertices, index); }
                break;
            }
        }
        if selection_type_buttons[1].1 {
            if let Some(index) = find_closest_edge(xy.0, xy.1, &obj) {
                let e = obj.edges.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) { e.2 = !e.2; }
                else { clear_selection_edges(&mut obj.edges, index); }
            }
        }
    }
}

fn freeze_objects(objects: &mut Vec<Object>, a: Angle) {
    for obj in objects.iter_mut() {
        for v in obj.vertices.iter_mut() {
            v.freeze(&a);
        }
    }
}

fn lmb_drag_event(
    is_lmb_down: &mut bool,
    click_timer: &mut Instant,
    pos: (f32, f32),
    last: (f32, f32),
    angle: &mut Angle,
    scroll_delta: f32,
) {
    if !*is_lmb_down {
        *is_lmb_down = true;
        *click_timer = Instant::now();
    }
    if click_timer.elapsed().as_millis() >= CLICK_TIMEOUT {
        let x_delta = (pos.0 - last.0) / 200.0;
        let y_delta = (pos.1 - last.1) / 200.0;
        if is_key_down(KeyCode::LeftShift) {
            angle.yz += y_delta;
            angle.xz += x_delta;
        } else {
            angle.yw += y_delta;
            angle.xw += x_delta;
        }
        angle.zw += scroll_delta / 100.0;
    }
}

fn draw_axes(axes: &Axes, w: f32, h: f32) {
    let (off_x, off_y) = axes.offset;
    let (off_x, off_y) = (off_x, off_y + h);
    if let Some((x, y)) = axes.x.centered(w, h) {
        draw_line(off_x, off_y, x + off_x, y + off_y, 2.0, Color::new(1.0, 0.0, 0.0, 1.0));
        draw_text_ex("X", x + off_x + 10.0, y + off_y, TextParams {
            font: Font::default(),
            font_size: 18,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::new(0.3, 0.3, 0.3, 1.0),
        })
    }
    if let Some((x, y)) = axes.y.centered(w, h) {
        draw_line(off_x, off_y, x + off_x, y + off_y, 2.0, Color::new(0.0, 1.0, 0.0, 1.0));
        draw_text_ex("Y", x + off_x + 10.0, y + off_y, TextParams {
            font: Font::default(),
            font_size: 18,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::new(0.3, 0.3, 0.3, 1.0),
        })
    }
    if let Some((x, y)) = axes.z.centered(w, h) {
        draw_line(off_x, off_y, x + off_x, y + off_y, 2.0, Color::new(0.0, 0.0, 1.0, 1.0));
        draw_text_ex("Z", x + off_x + 10.0, y + off_y, TextParams {
            font: Font::default(),
            font_size: 18,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::new(0.3, 0.3, 0.3, 1.0),
        })
    }
    if let Some((x, y)) = axes.w.centered(w, h) {
        draw_line(off_x, off_y, x + off_x, y + off_y, 2.0, Color::new(1.0, 0.0, 1.0, 1.0));
        draw_text_ex("W", x + off_x + 10.0, y + off_y, TextParams {
            font: Font::default(),
            font_size: 18,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::new(0.3, 0.3, 0.3, 1.0),
        })
    }

}

#[macroquad::main("Polytope 4D")]
async fn main() {
    unsafe {
        sapp::sapp_show_mouse(false);
    }
    let mut selection_type_buttons = vec![
        (Texture2D::from_file_with_format(std::fs::read("sprites/select0.png").unwrap().as_slice(), None), true, false),
        (Texture2D::from_file_with_format(std::fs::read("sprites/select1.png").unwrap().as_slice(), None), false, false),
        (Texture2D::from_file_with_format(std::fs::read("sprites/select2.png").unwrap().as_slice(), None), false, false),
        (Texture2D::from_file_with_format(std::fs::read("sprites/select3.png").unwrap().as_slice(), None), false, false),
    ];
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
    let mut cursor_transform_timer = Instant::now();
    let (mut x_pos, mut y_pos) = mouse_position();
    let mut axes = Axes::new(100.0, windows.main.config.y - 100.0);
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
        let mut hover_i = selection_type_buttons.len();
        for i in 0..4 {
            let xb = windows.main.config.w - 114.0 + 28.0 * i as f32;
            hover = cursor.intersect_with_box(
                xb,
                windows.main.config.y,
                28.0,
                30.0,
            );

            if hover {
                hover_i = i;
                if !cursor.rect || !selection_type_buttons[hover_i].2 {
                    cursor.set(xb, 0.0, 30.0, 32.0);
                }
                break;
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            lmb_drag_event(&mut is_lmb_down, &mut click_timer, (x_pos, y_pos), (x_last, y_last), &mut angle, scroll_delta);
        } else if is_lmb_down { // lmb up event
            if click_timer.elapsed().as_millis() < CLICK_TIMEOUT { // lmb click event
                lmb_click_event(hover, &mut selection_type_buttons, hover_i, &mut objects, (x_pos, y_pos));
            } else {
                freeze_objects(&mut objects, angle);
                axes.freeze(&angle);
            }
            angle.clear();
            is_lmb_down = false;
        }
        draw_windows(&windows);
        cursor.move_to(x_pos, y_pos);
        let d = dist(Vec4f::new0(), camera.c);
        for obj in (&mut objects).iter_mut() {
            obj.calc_vertices(&angle, d, &windows.main);
            draw_edges(obj);
            if selection_type_buttons[0].1 {
                draw_vertices(obj.vertices.clone());
            }
        }
        axes.calc(&angle, &windows.main);
        draw_axes(&axes, windows.main.config.w, windows.main.config.h);
        draw_cursor(&cursor);
        if !hover { cursor.reset(); }
        for i in 0..4 {
            let btn = selection_type_buttons.get_mut(i).unwrap();
            let xb = windows.main.config.w - 114.0 + 28.0 * i as f32;
            btn.2 = i == hover_i;
            draw_button(
                xb,
                0.0,
                30.0,
                30.0,
                btn.0,
                btn.1,
                btn.2,
            );
        }

        if cursor_transform_timer.elapsed().as_millis() >= CUR_TRANSFORM_TO {
            cursor_transform_timer = Instant::now();
            cursor.next();
            // println!("{}", cursor.conf.w);
        }
        next_frame().await
    }
}
