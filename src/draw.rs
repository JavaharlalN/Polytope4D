use super::*;

pub fn draw_cursor(cursor: &Cursor) {
    // println!("{}", cursor.rect);
    if cursor.rect {
        draw_rectangle(cursor.conf.x, cursor.conf.y, cursor.conf.w, cursor.conf.h, Color::new(0.3, 0.3, 0.3, 1.0));
    } else {
        draw_poly(cursor.conf.x, cursor.conf.y, 10, cursor.conf.r, 0.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }
}

fn draw_selected_motion_axe(offset: (f32, f32), axes: &MotionAxes, axe_index: usize) {
    if let Some(pos) = axes.grab_now {
        if let Some((x, y)) = pos.get_proj() {
            let color = match axe_index {
                0 => Color::new(1.0, 0.0, 0.0, 1.0),
                1 => Color::new(0.0, 1.0, 0.0, 1.0),
                2 => Color::new(0.0, 0.0, 1.0, 1.0),
                _ => Color::new(1.0, 0.0, 1.0, 1.0),
            };
            draw_line(offset.0, offset.1, x, y, 3.0, color);
        }
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
            if axes.x.selected { draw_selected_motion_axe((off_x, off_y), axes, 0); }
            if axes.y.selected { draw_selected_motion_axe((off_x, off_y), axes, 1); }
            if axes.z.selected { draw_selected_motion_axe((off_x, off_y), axes, 2); }
            if axes.w.selected { draw_selected_motion_axe((off_x, off_y), axes, 3); }
        } else {
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
        draw_circle(off_x, off_y, 3.0, Color::new(0.0, 0.2, 0.4, 1.0));
        draw_circle(off_x, off_y, 2.0, Color::new(0.0, 0.6, 1.0, 1.0));
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

pub fn draw_main_window(
    window:      &Window,
    objects:     &Vec<Object>,
    buttons:     &Vec<Button>,
    axes:        &Axes,
    motion_axes: &MotionAxes,
    cursor:      &Cursor,
) {
    draw_rectangle(
        window.config().x,
        window.config().y,
        window.config().w,
        window.config().h,
        Color::new(0.3, 0.3, 0.3, 0.5),
    );
    for obj in objects.iter() {
        draw_edges(obj);
        if buttons.get(0).unwrap().is_active() {
            draw_vertices(obj.vertices.clone());
        }
    }
    draw_axes(axes, window.config().w, window.config().h);
    draw_motion_axes(motion_axes);
    if let Some(buttons) = window.buttons() {
        for button in buttons {
            draw_button(button, Some(window));
        }
    }
    draw_cursor(cursor);
}

pub fn draw_overlapping_window(
    window: &OverlappingWindow,
    cursor: &Cursor
) {
    draw_cursor(&cursor);
}

pub fn draw_windows<'a>(
    windows:     &'a WindowGroup,
    objects:     &Vec<Object>,
    buttons:     &Vec<Button>,
    axes:        &Axes,
    motion_axes: &MotionAxes,
    cursor:      &Cursor,
) {
    // if !windows.instructions.hidden {
    //     draw_overlapping_window(&windows.instructions, cursor);
    //     return;
    // }
    draw_main_window(
        &windows.main,
        objects,
        buttons,
        axes,
        motion_axes,
        cursor,
    );
    for button in buttons {
        draw_button( button, None);
    }
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

pub fn draw_button(button: &Button, window: Option<&Window>) {
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