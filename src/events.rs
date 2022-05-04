use std::f32::consts::PI;

use super::*;

pub fn catch_mouse_event(
    is_lmb_down: &mut bool,
    is_rmb_down: &mut bool,
    scroll_delta: f32,
    lmb_click_timer: &mut Instant,
    rmb_click_timer: &mut Instant,
    hover: bool,
    hover_i: usize,
    selection_type_buttons: &mut Vec<(Texture2D, bool, bool)>,
    objects: &mut Vec<Object>,
    xy: (f32, f32),
    xy_last: (f32, f32),
    motion_axes: &mut MotionAxes,
    angle: &mut Angle,
) {
    if is_mouse_button_down(MouseButton::Left) {
        lmb_down_event(is_lmb_down, lmb_click_timer);
    } else if *is_lmb_down { // lmb up event
        if lmb_click_timer.elapsed().as_millis() < CLICK_TIMEOUT { // lmb click event
            lmb_click_event(
                hover,
                selection_type_buttons,
                hover_i,
                objects,
                xy,
                motion_axes,
            );
        }
        *is_lmb_down = false;
    } else if is_mouse_button_down(MouseButton::Right) {
        rmb_down_event(is_rmb_down, rmb_click_timer, motion_axes);
        drag_event(xy, xy_last, angle, scroll_delta, motion_axes, objects,);
    } else if *is_rmb_down {
        mouse_up_event(is_rmb_down, motion_axes, objects);
    }
    if xy_last != xy {
        mouse_move_event(xy, motion_axes);
    }
}

pub fn mouse_move_event(
    xy: (f32, f32),
    motion_axes: &mut MotionAxes,
) {
    if !motion_axes.grabbed {
        motion_axes.get_closest_axe(xy.0, xy.1);
    }
}

pub fn lmb_down_event(
    is_lmb_down: &mut bool,
    timer: &mut Instant,
) {
    if !*is_lmb_down {
        *is_lmb_down = true;
        *timer = Instant::now();
    }
}

pub fn rmb_down_event(
    is_rmb_down: &mut bool,
    timer: &mut Instant,
    motion_axes: &mut MotionAxes,
) {
    if !*is_rmb_down {
        *is_rmb_down = true;
        *timer = Instant::now();
        motion_axes.try_grab();
    }
}

pub fn mouse_up_event(
    is_mb_down: &mut bool,
    motion_axes: &mut MotionAxes,
    objects: &Vec<Object>,
) {
    *is_mb_down = false;
    motion_axes.ungrab();
    motion_axes.pos = get_center(objects);
}

pub fn drag_event(
    xy: (f32, f32),
    xy_last: (f32, f32),
    angle: &mut Angle,
    scroll_delta: f32,
    motion_axes: &mut MotionAxes,
    objects: &mut Vec<Object>,
) {
    if motion_axes.grabbed {
        if motion_axes.grabbed && is_mouse_button_down(MouseButton::Right) {
            let delta = motion_axes.get_motion_delta(sub2d(xy, xy_last));
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
    hover: bool,
    selection_type_buttons: &mut Vec<(Texture2D, bool, bool)>,
    hover_i: usize,
    objects: &mut Vec<Object>,
    xy: (f32, f32),
    motion_axes: &mut MotionAxes,
) {
    if hover {
        if is_key_down(KeyCode::LeftShift) {
            if get_enabled_buttons_count(selection_type_buttons) > 1 {
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
        if selection_type_buttons[1].1 {
            if let Some(index) = find_closest_edge(xy.0, xy.1, &obj) {
                let e = obj.edges.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) {
                    if e.2 { // if selected
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
    motion_axes.pos = get_center(objects);
}

pub fn resize_event<'a>(windows: &'a mut WindowGroup) {
    (*windows).main.set_size(screen_width(), screen_height());
}