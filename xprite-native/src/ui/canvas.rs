use crate::prelude::*;
use super::*;
use xprite::rendering::Renderer;

pub fn draw_canvas(rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("canvas"))
        .position((LEFT_SIDE_WIDTH, 20.0), ImGuiCond::Always)
        .size((sz.0 as f32 - RIGHT_SIDE_WIDTH - LEFT_SIDE_WIDTH, sz.1 as f32 - 20.), ImGuiCond::Always)
        .resizable(false)
        .title_bar(false)
        .movable(false)
        .collapsible(false)
        .build(|| {
            let styles = [
                StyleVar::FramePadding(ImVec2::new(0., 0.)),
                StyleVar::WindowPadding(ImVec2::new(0., 0.)),
            ];
            let colors = [ (ImGuiCol::ChildBg, GREY) ];
            ui.with_style_and_color_vars(&styles, &colors, || {
                let win_sz = ui.get_window_size();
                let child_frame_sz = (win_sz.0, win_sz.1 - 10.);
                ui.child_frame(im_str!("scrolling_region"), child_frame_sz)
                    .show_scrollbar(false)
                    .movable(false)
                    .build(|| {
                        update_viewport(state, ui);
                        state.xpr.render(rdr);
                        // draw_grid(state, ui);
                        bind_input(state, ui);
                    });
            });

            // ui.drag_float(im_str!("scale"), &mut state.xpr.canvas.scale)
            //   .min(1.)
            //   .max(50.)
            //   .speed(0.1)
            //   .build();
            // checkbox for show grid

            // ui.checkbox(im_str!("grid"), &mut state.xpr.canvas.show_grid);
            // ui.text(
            //     im_str!("{}, {}",
            //     state.xpr.last_mouse_pos.0,
            //     state.xpr.last_mouse_pos.1)
            // );


        });
}

fn update_viewport(state: &mut State, ui: &Ui) {
    let cvs = &mut state.xpr.canvas;
    let win_pos = ui.get_cursor_screen_pos();
    cvs.update_pos(win_pos.0, win_pos.1);

    let canvas_sz = ui.get_window_size();
    cvs.update_sz(canvas_sz.0, canvas_sz.1);

    if !cvs.initialized {
        cvs.scale = cvs.canvas_w / cvs.art_w / CANVAS_INIT_SCALE;
        cvs.scroll.x = (cvs.canvas_w - cvs.scale * cvs.art_w) / 2.;
        cvs.scroll.y = (cvs.canvas_h - cvs.scale * cvs.art_h) / 2.;
    }

    state.xpr.canvas.initialized = true;
}

fn bind_input(state: &mut State, ui: &Ui) {
    use self::InputItem::*;
    use self::InputEvent::*;

    let wheel_delta = ui.imgui().mouse_wheel();
    let (x, y) = ui.imgui().mouse_pos();

    if (state.xpr.last_mouse_pos.0 != x || state.xpr.last_mouse_pos.1 != y)
    && !state.xpr.inputs.space
    {
        state.xpr.mouse_move(&MouseMove{ x, y });
    }

    let left = ui.imgui().is_mouse_down(ImMouseButton::Left);
    let right = ui.imgui().is_mouse_down(ImMouseButton::Right);

    let using_window = ui.is_window_hovered() && !ui.is_item_active();

    if state.xpr.inputs.space {
        ui.imgui().set_mouse_cursor(ImGuiMouseCursor::Move);
    }

    // middle key for scrolling
    if using_window &&
        (
            ui.imgui().is_mouse_dragging(ImMouseButton::Middle)
        ||  (state.xpr.inputs.space && state.xpr.inputs.left)
        )
    {
        // set cursor
        ui.imgui().set_mouse_cursor(ImGuiMouseCursor::Move);
        let d = ui.imgui().mouse_delta();
        state.xpr.canvas.scroll.x += d.0;
        state.xpr.canvas.scroll.y += d.1;
    }

    if using_window {
        state.xpr.canvas.scale += wheel_delta
    }
    if state.xpr.canvas.scale > 100. {
        state.xpr.canvas.scale = 100.;
    }
    if state.xpr.canvas.scale < 0. {
        state.xpr.canvas.scale = 1.;
    }

    // left
    if state.xpr.inputs.debounce(InputItem::Left, left)
    && using_window
    && !state.xpr.inputs.space {
        if left {
            trace!("mouse left down");
            state.xpr.event(&MouseDown{ x, y, button: Left });
        } else {
            trace!("mouse left up");
            state.xpr.event(&MouseUp{ x, y });
        }
    }

    // right
    if state.xpr.inputs.debounce(InputItem::Right, right) && using_window {
        if right {
            let (x, y) = ui.imgui().mouse_pos();
            state.xpr.event(&MouseDown{ x, y, button: Right });
        }
    }

    // ctrl
    let ctrl = ui.imgui().key_ctrl();
    if state.xpr.inputs.debounce(InputItem::Ctrl, ctrl) {
        if ctrl {
            trace!("ctrl down");
            state.xpr.event(&KeyDown{ key: Ctrl });
        } else {
            trace!("ctrl up");
            state.xpr.event(&KeyUp{ key: Ctrl });
        }
    }

    // shift
    let shift = ui.imgui().key_shift();
    if state.xpr.inputs.debounce(InputItem::Shift, shift) {
        if shift {
            trace!("shift down");
            state.xpr.event(&KeyDown{ key: Shift });
        } else {
            trace!("shift up");
            state.xpr.event(&KeyUp{ key: Shift });
        }
    }

    // space
    let space = ui.imgui().is_key_down(19);
    if state.xpr.inputs.debounce(InputItem::Space, space) {
        if space {
            trace!("space down");
            state.xpr.event(&KeyDown{ key: Space });
        } else {
            trace!("space up");
            state.xpr.event(&KeyUp{ key: Space });
        }
    }

    // alt
    let alt = ui.imgui().key_alt();
    if state.xpr.inputs.debounce(InputItem::Alt, alt) {
        if alt {
            trace!("alt down");
            state.xpr.event(&KeyDown{ key: Alt });
        } else {
            trace!("alt up");
            state.xpr.event(&KeyUp{ key: Alt });
        }
    }

    // z
    let key_z = ui.imgui().get_key_index(ImGuiKey::Z);
    let z = ui.imgui().is_key_down(key_z);
    if state.xpr.inputs.debounce(InputItem::Z, z) {
        if z {
            trace!("z down");
            if state.xpr.inputs.ctrl {
                state.xpr.undo();
                trace!("ctrl+z");
            }
        } else {
            trace!("z up");
        }
    }

    // y
    let key_y = ui.imgui().get_key_index(ImGuiKey::Y);
    let is_y_down = ui.imgui().is_key_down(key_y);
    if state.xpr.inputs.debounce(InputItem::Y, is_y_down) {
        if is_y_down {
            trace!("Y down");
            if state.xpr.inputs.ctrl {
                state.xpr.redo();
                trace!("ctrl+y");
            }
        } else {
            trace!("y up");
        }
    }

    // for i in 0..512 {
    //     if ui.imgui().is_key_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.xpr.update_mouse_pos(x, y);
}