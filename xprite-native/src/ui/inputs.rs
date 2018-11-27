use crate::prelude::*;

#[repr(usize)]
pub enum KeyCode {
    Tab = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
    PageUp = 5,
    PageDown = 6,
    Home = 7,
    End = 8,
    Delete = 9,
    Back = 10,
    Return = 11,
    Escape = 12,
    A = 13,
    B = 14,
    C = 15,
    D = 16,
    E = 17,
    F = 18,
    G = 19,
    H = 20,
    I = 21,
    J = 22,
    K = 23,
    L = 24,
    M = 25,
    N = 26,
    O = 27,
    P = 29,
    Q = 30,
    R = 31,
    S = 32,
    T = 33,
    U = 34,
    V = 35,
    W = 36,
    X = 37,
    Y = 38,
    Z = 39,
    Key0 = 40,
    Key1 = 41,
    Key2 = 42,
    Key3 = 43,
    Key4 = 44,
    Key5 = 45,
    Key6 = 46,
    Key7 = 47,
    Key8 = 48,
    Key9 = 49,
    Space = 50,
}

pub fn bind_input(state: &mut State, ui: &Ui) {
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

    let is_space = ui.imgui().is_key_down(KeyCode::Space as usize);
    handle_input!(is_space, Space);

    let is_alt = ui.imgui().key_alt();
    handle_input!(is_alt, Alt);

    let is_z = ui.imgui().is_key_down(KeyCode::Z as usize);
    handle_input!(is_z, Z, {
        if state.xpr.inputs.ctrl {
            state.xpr.undo();
        }
    }, {});

    let is_y = ui.imgui().is_key_down(KeyCode::Y as usize);
    handle_input!(is_y, Y, {
        if state.xpr.inputs.ctrl {
            state.xpr.redo();
        }
    }, {});

    let is_enter = ui.imgui().is_key_down(KeyCode::Return as usize);
    handle_input!(is_enter, Enter);


    // for i in 0..512 {
    //     if ui.imgui().is_key_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.xpr.update_mouse_pos(x, y);
}