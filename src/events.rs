use super::*;
use std::f32::consts::PI;
use macroquad::prelude::KeyCode;
use macroquad::prelude::MouseButton;
use macroquad::prelude::is_key_down;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::is_mouse_button_down;

pub fn catch_mouse_event(
    ms:          &mut MouseState,
    hover:       bool,
    hover_i:     usize,
    buttons:     &mut Vec<Button>,
    objects:     &mut Vec<Object>,
    xy_last:    (f32, f32),
    motion_axes: &mut MotionAxes,
    angle:       &mut Angle,
    window:      &Window,
) {
    if is_mouse_button_down(MouseButton::Left) {
        lmb_down_event(&mut ms.is_lmb_down, &mut ms.lmb_click_timer, buttons);
    } else if ms.is_lmb_down { // lmb up event
        if ms.lmb_click_timer.elapsed().as_millis() < CLICK_TIMEOUT { // lmb click event
            lmb_click_event(
                hover,
                buttons,
                hover_i,
                objects,
                ms.pos,
                motion_axes,
            );
        }
		lmb_up_event(buttons, objects);
        ms.is_lmb_down = false;
    } else if is_mouse_button_down(MouseButton::Right) {
        rmb_down_event(&mut ms.is_rmb_down, &mut ms.rmb_click_timer, motion_axes);
        drag_event(ms.pos, xy_last, angle, ms.scroll_delta, motion_axes, objects, window);
    } else if ms.is_rmb_down {
        mouse_up_event(&mut ms.is_rmb_down, motion_axes, objects);
    }
    if xy_last != ms.pos {
        mouse_move_event(ms.pos, motion_axes);
    }
}

// TODO: merge to mouse_up_event
pub fn lmb_up_event(buttons: &mut Vec<Button>, objects: &mut Vec<Object>) {
    for btn in buttons {
        if btn.is_active() && btn.is_click_button() {
            btn.set_active(false);
            match btn.get_type() {
                ButtonType::Export =>  { save(objects); },
                ButtonType::Import => { match open_4dp() {
                    Ok(obj) => {
                        objects.clear();
                        objects.push(obj);
                    }, Err(e) => println!("{}", e),
                } },
                _ => {},
            }
        }
    }
}

pub fn catch_keyboard_event(
    objects:     &mut Vec<Object>,
    clipboard:   &mut Object,
    motion_axes: &mut MotionAxes,
) {
    if is_key_pressed(KeyCode::E) {
        extrude_event(objects);
    } else if is_key_pressed(KeyCode::Delete) {
        delete_event(objects, motion_axes);
    } else if is_key_pressed(KeyCode::F) {
        fill_event(objects, motion_axes);
    } else if is_key_down(KeyCode::LeftControl) {
        if is_key_pressed(KeyCode::C) { copy_event(objects, clipboard); }
        else if is_key_pressed(KeyCode::V) { paste_event(objects, clipboard); }
    }
}

pub fn fill_event(
    objects:     &mut Vec<Object>,
    motion_axes: &mut MotionAxes,
) {
    for obj in objects.iter_mut() {
        let indices = obj.get_selected_vertices();
        if indices.len() != 2 { return; }
        else {
            obj.edges.push(Edge::new(indices[0], indices[1]).clone_and_select(true));
        }
    }
    motion_axes.move_to(get_center(objects));
}

pub fn delete_event(
    objects:     &mut Vec<Object>,
    motion_axes: &mut MotionAxes,
) {
    for obj in objects {
        let len = obj.vertices.len();
        for i in 1..len + 1 {
            obj.delete_vertex(len - i);
        }
    }
    motion_axes.move_to(None);
}
/// Copies selected vertices, edges and faces from objects
/// and writes to specified clipboard.
pub fn copy_event(objects: &Vec<Object>, clipboard: &mut Object) {
    let mut v_indices = vec![];
    let mut e_indices = vec![];
    let mut clipboards = vec![];
    *clipboard = Object::empty();
    for (o, obj) in objects.iter().enumerate() {
        v_indices.push(vec![]);
        e_indices.push(vec![]);
        clipboards.push(Object::empty());
        for (i, v) in obj.vertices.iter().enumerate() {
            if v.selected {
                v_indices[o].push(i);
            }
        }
        for e in &obj.edges {
            if e.selected {
                e_indices[o].push((e.a, e.b));
            }
        }
    }
    for o in v_indices.iter().enumerate() {
        for (i, v) in o.1.iter().enumerate() {
            clipboards[o.0].vertices.push(objects[o.0].vertices[*v]);
            for e in &mut e_indices[o.0] {
                if (*e).0 == *v { e.0 = i; }
                if e.1 == *v { e.1 = i; }
            }
        }
    }
    for o in e_indices.iter().enumerate() {
        for e in o.1 {
            clipboards[o.0].edges.push(Edge::new(e.0, e.1));
        }
    }
    for c in clipboards {
        *clipboard += c;
    }
}

pub fn paste_event(objects: &mut Vec<Object>, clipboard: &Object) {
    for obj in objects.iter_mut() {
        obj.clear_selection();
    }
    let mut new_data = clipboard.clone();
    new_data.select();
    objects.push(new_data);
}

pub fn extrude_event(objects: &mut Vec<Object>) {
    for i in 0..objects.len() {
        let mut buffer = Object::empty();
        let vertices_count = objects[i].vertices.len();
        copy_event(&vec![objects[i].clone()], &mut buffer);
        // objects[i].clear_selection();
        buffer.select();
        objects[i] += buffer;
        let mut counter = 0;
        for j in 0..vertices_count {
            if objects[i].vertices[j].selected {
                objects[i].edges.push(Edge::new(j, vertices_count + counter));
                counter += 1;
            }
        }
        objects[i].deselect_first_n_vertices(vertices_count);
    }
}

pub fn mouse_move_event(
    xy:         (f32, f32),
    motion_axes: &mut MotionAxes,
) {
    if !motion_axes.grabbed {
        motion_axes.get_closest_axe(xy.0, xy.1);
    }
}

pub fn lmb_down_event(
    is_lmb_down: &mut bool,
    timer:       &mut Instant,
    buttons:     &mut Vec<Button>,
) {
    if !*is_lmb_down {
        *is_lmb_down = true;
        *timer = Instant::now();
    }
    for btn in buttons {
        if btn.is_hover() && btn.is_click_button() {
            btn.set_active(true);
        }
    }
}

pub fn rmb_down_event(
    is_rmb_down: &mut bool,
    timer:       &mut Instant,
    motion_axes: &mut MotionAxes,
) {
    if !*is_rmb_down {
        *is_rmb_down = true;
        *timer = Instant::now();
        motion_axes.try_grab();
    }
}

pub fn mouse_up_event(
    is_mb_down:  &mut bool,
    motion_axes: &mut MotionAxes,
    objects:     &Vec<Object>,
) {
    *is_mb_down = false;
    motion_axes.ungrab();
    motion_axes.pos = get_center(objects);
    motion_axes.grab_start = None;
}

pub fn drag_event(
    xy:          (f32, f32),
    xy_last:     (f32, f32),
    angle:        &mut Angle,
    scroll_delta: f32,
    motion_axes:  &mut MotionAxes,
    objects:      &mut Vec<Object>,
    window:       &Window,
) {
    if motion_axes.grabbed {
        if motion_axes.grabbed && is_mouse_button_down(MouseButton::Right) {
            let delta = motion_axes.get_motion_delta(sub2d(xy, xy_last), angle, window);
            for obj in objects {
                for v in &mut obj.vertices {
                    if v.selected {
                        *v += delta;
                    }
                }
            }
        }
        return;
    }
    let x_delta = (xy.0 - xy_last.0) / 200.0;
    let y_delta = (xy.1 - xy_last.1) / 200.0;
    if is_key_down(KeyCode::LeftShift) {
        angle.yz += y_delta;
        angle.xz += x_delta;
    } else {
        angle.yw += y_delta;
        angle.xw += x_delta;
    }
    angle.yz = angle.yz.max(-PI / 2.0).min(PI / 2.0);
    angle.yw = angle.yw.max(-PI / 2.0).min(PI / 2.0);
    angle.zw += scroll_delta / 100.0;
}

pub fn lmb_click_event(
    hover:       bool,
    buttons:     &mut Vec<Button>,
    hover_i:     usize,
    objects:     &mut Vec<Object>,
    xy:         (f32, f32),
    motion_axes: &mut MotionAxes,
) {
    if hover && hover_i < buttons.len() && buttons[hover_i].is_check_button() {
        if is_key_down(KeyCode::LeftShift) {
            if get_enabled_buttons_count(buttons) > 1 {
                let h = buttons.get(hover_i).unwrap().is_active();
                buttons.get_mut(hover_i).unwrap().set_active(!h);
            } else if !buttons[hover_i].is_active() {
                buttons[hover_i].set_active(true);
            }
        } else {
            for b in buttons.iter_mut() {
                b.set_active(false);
            }
            buttons[hover_i].set_active(true);
        }
    }
    for obj in objects.iter_mut() {
        if buttons[0].is_active() {
            if let Some(index) = find_closest_vertice(xy.0, xy.1, &obj.vertices) {
                let v = obj.vertices.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) {
                    if v.selected {
                        obj.deselect_vertice(index);
                    } else {
                        obj.select_vertice(index);
                    }
                } else {
                    clear_selection(obj);
                    obj.vertices[index].selected = true;
                }
                break;
            }
        }
        if buttons[1].is_active() {
            if let Some(index) = find_closest_edge(xy.0, xy.1, &obj) {
                let e = obj.edges.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) {
                    if e.selected {
                        obj.deselect_edge(index);
                    } else {
                        obj.select_edge(index);
                    }
                } else {
                    clear_selection(obj);
                    obj.select_edge(index);
                }
            }
        }
    }
    motion_axes.move_to(get_center(objects));
}

pub fn resize_event(windows: &mut WindowGroup) {
    (*windows).main.set_size(screen_width(), screen_height());
}