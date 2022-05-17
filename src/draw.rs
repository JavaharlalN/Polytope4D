use super::*;

pub fn draw_cursor(cursor: &Cursor) {
    // println!("{}", cursor.rect);
    if cursor.rect {
        draw_rectangle(cursor.conf.x, cursor.conf.y, cursor.conf.w, cursor.conf.h, Color::new(0.3, 0.3, 0.3, 1.0));
    } else {
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }
}

fn draw_selected_motion_axe(axe: &Vec4f, (off_x, off_y): (f32, f32), axe_index: usize) {
    if let Some((x, y)) = axe.get_proj() {
        let color = match axe_index {
            0 => Color::new(1.0, 0.0, 0.0, 1.0),
            1 => Color::new(0.0, 1.0, 0.0, 1.0),
            2 => Color::new(0.0, 0.0, 1.0, 1.0),
            _ => Color::new(1.0, 0.0, 1.0, 1.0),
        };
        draw_line(off_x, off_y, x, y, 3.0, color);
    }
}

fn draw_motion_axe(axe: &Vec4f, (off_x, off_y): (f32, f32), axe_index: usize) {
    if let Some((x, y)) = axe.get_proj() {
        let thickness = if axe.selected { 3.0 } else { 2.0 };
        let a = if axe.selected { 1.0 } else { 0.7 };

        let color = match axe_index {
            0 => Color::new(1.0, 0.0, 0.0, a),
            1 => Color::new(0.0, 1.0, 0.0, a),
            2 => Color::new(0.0, 0.0, 1.0, a),
            _ => Color::new(1.0, 0.0, 1.0, a),
        };

        draw_line(off_x, off_y, x, y, thickness, color);
    }
}

pub fn draw_motion_axes(axes: &MotionAxes) {
    if let Some(pos) = axes.pos {
        let (off_x, off_y) =
        if let Some(off) = pos.get_proj() {
            off
        } else {
            return;
        };
        if axes.grabbed {
            if let Some(grabbed_axe) = axes.grab_now {
                if axes.x.selected { draw_selected_motion_axe(&grabbed_axe, (off_x, off_y), 0); }
                if axes.y.selected { draw_selected_motion_axe(&grabbed_axe, (off_x, off_y), 1); }
                if axes.z.selected { draw_selected_motion_axe(&grabbed_axe, (off_x, off_y), 2); }
                if axes.w.selected { draw_selected_motion_axe(&grabbed_axe, (off_x, off_y), 3); }
            }
        } else {
            draw_motion_axe(&axes.x, (off_x, off_y), 0);
            draw_motion_axe(&axes.y, (off_x, off_y), 1);
            draw_motion_axe(&axes.z, (off_x, off_y), 2);
            draw_motion_axe(&axes.w, (off_x, off_y), 3);
        }
        draw_circle(off_x, off_y, 3.0, Color::new(0.0, 0.2, 0.4, 1.0));
        draw_circle(off_x, off_y, 2.0, Color::new(0.0, 0.6, 1.0, 1.0));
    }
}

fn draw_axe(off: (f32, f32), xy: (f32, f32), axe_index: usize) {
    let color = match axe_index {
        0 => Color::new(1.0, 0.0, 0.0, 1.0),
        1 => Color::new(0.0, 1.0, 0.0, 1.0),
        2 => Color::new(0.0, 0.0, 1.0, 1.0),
        _  => Color::new(1.0, 0.0, 1.0, 1.0),
    };
    draw_line(off.0, off.1, xy.0 + off.0, xy.1 + off.1, 2.0, color);

    let name = match axe_index {
        0 => "X",
        1 => "Y",
        2 => "Z",
        _  => "W",
    };
    draw_text_ex(name, xy.0 + off.0 + 10.0, xy.1 + off.1, TextParams {
        font: Font::default(),
        font_size: 18,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: Color::new(0.3, 0.3, 0.3, 1.0),
    })
}

pub fn draw_axes(axes: &Axes, w: f32, h: f32) {
    let (off_x, off_y) = axes.offset;
    let offset = (off_x, off_y + h);
    if let Some(xy) = axes.x.centered(w, h) {
        draw_axe(offset, xy, 0);
    }
    if let Some(xy) = axes.y.centered(w, h) {
        draw_axe(offset, xy, 1);
    }
    if let Some(xy) = axes.z.centered(w, h) {
        draw_axe(offset, xy, 2);
    }
    if let Some(xy) = axes.w.centered(w, h) {
        draw_axe(offset, xy, 3);
    }
}

pub fn draw_windows<'a>(windows: &'a WindowGroup) {
    draw_rectangle(
        windows.main.config().x,
        windows.main.config().y,
        windows.main.config().w,
        windows.main.config().h,
        Color::new(0.3, 0.3, 0.3, 0.5),
    );
}

pub fn draw_vertices(vertices: Vec<Vec4f>) {
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

pub fn draw_edges(obj: &Object) {
    for e in (&obj.edges).into_iter() {
        let a = obj.vertices[e.a].get_proj().unwrap();
        let b = obj.vertices[e.b].get_proj().unwrap();
        // println!("a: ({}, {}), b: ({}, {})", a.0, a.1, b.0, b.1);
        if e.selected {
            draw_line(a.0, a.1, b.0, b.1, 2.0, Color::new(0.1, 0.2, 0.4, 1.0));
            draw_line(a.0, a.1, b.0, b.1, 1.0, Color::new(0.1, 0.6, 1.0, 1.0));
        } else {
            draw_line(a.0, a.1, b.0, b.1, 1.0, Color::new(0.1, 0.1, 0.1, 1.0));
        }
    }
}

pub fn draw_cursor_overlay(cursor: Cursor) {
    draw_circle(
        cursor.real.x,
        cursor.real.y,
        cursor.real.r,
        Color::new(0.3, 0.3, 0.3, 0.4),
    );
}

pub fn draw_button(button: &Button, window: &Window) {
    let k = if button.is_hover() { 0.6 } else { 0.5 };
    let (x, y) = button.get_pos(window);
    let (w, h) = button.size();
    draw_rectangle(
        x, y,
        w, h,
        // thickness,
        Color::new(k, k, k, 1.0)
    );
    draw_texture(
        button.texture(),
        x, y,
        Color::new(1.0, 1.0, 1.0, 1.0)
    );
    if button.is_active() {
        draw_line(
            x,     h - 1.0,
            x + w, h - 1.0,
            2.0,
            Color::new(0.0, 0.6, 1.0, 1.0),
        );
    }
}