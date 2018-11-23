use crate::prelude::*;
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

    macro_rules! handle_input {
        ($boolval: expr, $key_upper: ident) => {
            if state.xpr.inputs.debounce(InputItem::$key_upper, $boolval) {
                if $boolval {
                    state.xpr.event(&KeyDown{ key: $key_upper });
                } else {
                    state.xpr.event(&KeyUp{ key: $key_upper });
                }
            }
        };

        ($boolval: expr, $key_upper: ident, $tblock: block, $fblock: block) => {
            if state.xpr.inputs.debounce(InputItem::$key_upper, $boolval) {
                if $boolval {
                    state.xpr.event(&KeyDown{ key: $key_upper });
                    $tblock
                } else {
                    state.xpr.event(&KeyUp{ key: $key_upper });
                    $fblock
                }
            }
        };
    }


    let is_ctrl = ui.imgui().key_ctrl();
    handle_input!(is_ctrl, Ctrl);

    let is_shift = ui.imgui().key_shift();
    handle_input!(is_shift, Shift);

    let is_space = ui.imgui().is_key_down(19);
    handle_input!(is_space, Space);

    let is_alt = ui.imgui().key_alt();
    handle_input!(is_alt, Alt);

    let key_z = ui.imgui().get_key_index(ImGuiKey::Z);
    let is_z = ui.imgui().is_key_down(
        key_z);
    handle_input!(is_z, Z, {
        if state.xpr.inputs.ctrl {
            state.xpr.undo();
        }
    }, {});

    let key_y = ui.imgui().get_key_index(ImGuiKey::Y);
    let is_y = ui.imgui().is_key_down(key_y);
    handle_input!(is_y, Y, {
        if state.xpr.inputs.ctrl {
            state.xpr.redo();
        }
    }, {});

    let is_enter = ui.imgui().is_key_down(11);
    handle_input!(is_enter, Enter);


    // for i in 0..512 {
    //     if ui.imgui().is_key_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.xpr.update_mouse_pos(x, y);
}