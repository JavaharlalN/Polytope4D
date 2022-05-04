use super::*;

pub fn draw_cursor(cursor: &Cursor) {
    // println!("{}", cursor.rect);
    if cursor.rect {
        draw_rectangle(cursor.conf.x, cursor.conf.y, cursor.conf.w, cursor.conf.h, Color::new(0.3, 0.3, 0.3, 1.0));
    } else {
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
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
        if let Some((x, y)) = axes.x.get_proj() {
            let thickness = if axes.x.selected { 3.0 } else { 2.0 };
            let a = if axes.x.selected { 1.0 } else { 0.7 };
            draw_line(off_x, off_y, x, y, thickness, Color::new(1.0, 0.0, 0.0, a));
        }
        if let Some((x, y)) = axes.y.get_proj() {
            let thickness = if axes.y.selected { 3.0 } else { 2.0 };
            let a = if axes.y.selected { 1.0 } else { 0.7 };
            draw_line(off_x, off_y, x, y, thickness, Color::new(0.0, 1.0, 0.0, a));
        }
        if let Some((x, y)) = axes.z.get_proj() {
            let thickness = if axes.z.selected { 3.0 } else { 2.0 };
            let a = if axes.z.selected { 1.0 } else { 0.7 };
            draw_line(off_x, off_y, x, y, thickness, Color::new(0.0, 0.0, 1.0, a));
        }
        if let Some((x, y)) = axes.w.get_proj() {
            let thickness = if axes.w.selected { 3.0 } else { 2.0 };
            let a = if axes.w.selected { 1.0 } else { 0.7 };
            draw_line(off_x, off_y, x, y, thickness, Color::new(1.0, 0.0, 1.0, a));
        }
    }
}

fn draw_axe(off: (f32, f32), xy: (f32, f32), name: &str) {
    let color = match name {
        "X" => Color::new(1.0, 0.0, 0.0, 1.0),
        "Y" => Color::new(0.0, 1.0, 0.0, 1.0),
        "Z" => Color::new(0.0, 0.0, 1.0, 1.0),
         _  => Color::new(1.0, 0.0, 1.0, 1.0),
    };
    draw_line(off.0, off.1, xy.0 + off.0, xy.1 + off.1, 2.0, color);
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
        draw_axe(offset, xy, "X");
    }
    if let Some(xy) = axes.y.centered(w, h) {
        draw_axe(offset, xy, "Y");
    }
    if let Some(xy) = axes.z.centered(w, h) {
        draw_axe(offset, xy, "Z");
    }
    if let Some(xy) = axes.w.centered(w, h) {
        draw_axe(offset, xy, "W");
    }
}

pub fn draw_windows<'a>(windows: &'a WindowGroup) {
    draw_rectangle(
        windows.main.config.x,
        windows.main.config.y,
        windows.main.config.w,
        windows.main.config.h,
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

pub fn draw_button(x: f32, y: f32, w: f32, h: f32, texture: Texture2D, selected: bool, hover: bool) {
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