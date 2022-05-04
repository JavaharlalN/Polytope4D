use super::*;

pub fn draw_cursor(cursor: &Cursor) {
    // println!("{}", cursor.rect);
    if cursor.rect {
        draw_rectangle(cursor.conf.x, cursor.conf.y, cursor.conf.w, cursor.conf.h, Color::new(0.3, 0.3, 0.3, 1.0));
    } else {
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }
}

pub fn draw_axes(axes: &Axes, w: f32, h: f32) {
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