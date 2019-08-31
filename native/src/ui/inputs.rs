use crate::prelude::*;

#[allow(unused)]
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
    Grave = 51,
}

macro_rules! handle_error {
    ($e: expr) => {
        if let Err(e) = $e {
            error!("{}", e);
        }
    };
}

pub fn bind_input(state: &mut State, ui: &Ui) {
    use self::InputEvent::*;
    use self::InputItem::*;

    let wheel_delta = ui.io().mouse_wheel as f64 / 5.;
    let pos = ui.io().mouse_pos;
    let x = pos[0].into();
    let y = pos[1].into();

    if (state.xpr().last_mouse_pos.x != x || state.xpr().last_mouse_pos.y != y) && !state.inputs.space {
        handle_error!(state.xpr_mut().mouse_move(&MouseMove { x: x.into(), y: y.into() }));
    }

    let left = ui.is_mouse_down(MouseButton::Left);
    let right = ui.is_mouse_down(MouseButton::Right);

    let using_window = ui.is_window_hovered() && !ui.is_item_active();

    if state.inputs.space {
        ui.set_mouse_cursor(Some(MouseCursor::Hand));
    }

    let is_wheel_dragging = ui.is_mouse_dragging(MouseButton::Middle) || state.inputs.space;
    // middle key for scrolling
    if using_window && is_wheel_dragging {
        // set cursor
        ui.set_mouse_cursor(Some(MouseCursor::Hand));
        let d = ui.io().mouse_delta;
        state.xpr_mut().canvas.scroll.x += d[0] as f64;
        state.xpr_mut().canvas.scroll.y += d[1] as f64;
    }

    if using_window && wheel_delta != 0. {
        // if alt pressed, change brush size
        if state.inputs.alt {
            let current_tool = state.xpr().last_tool();
            if let Some(brush) = state.xpr().get_brush_for_tool(current_tool) {
                state.brush.sz[0] += wheel_delta.signum() as i32;
                state.set_brush_for_tool(brush.brush_type, current_tool);
            }
        } else {
            state.xpr_mut().canvas.update_zoom(wheel_delta, (x, y))
        }
    }
    if state.xpr().canvas.scale > 100. {
        state.xpr_mut().canvas.scale = 100.;
    }
    if state.xpr().canvas.scale < 0. {
        state.xpr_mut().canvas.scale = 1.;
    }

    // left
    if state.inputs.debounce(InputItem::Left, left) && using_window && !state.inputs.space {
        if left {
            trace!("mouse left down");
            handle_error!(state.xpr_mut().event(&MouseDown {
                x: x.into(),
                y: y.into(),
                button: Left
            }));
        } else {
            trace!("mouse left up");
            handle_error!(state.xpr_mut().event(&MouseUp {
                x: x.into(),
                y: y.into(),
                button: Right
            }));
        }
    }

    // right
    if state.inputs.debounce(InputItem::Right, right) && using_window {
        if right {
            let pos = ui.io().mouse_pos;
            let x = pos[0].into();
            let y = pos[1].into();
            handle_error!(state.xpr_mut().event(&MouseDown { x: x, y: y, button: Right }));
        } else {
            trace!("mouse right up");
            handle_error!(state.xpr_mut().event(&MouseUp { x: x, y: y, button: Right }));
        }
    }

    macro_rules! handle_input {
        ($boolval: expr, $key_upper: ident) => {
            if state.inputs.debounce(InputItem::$key_upper, $boolval) {
                if $boolval {
                    handle_error!(state.xpr_mut().event(&KeyDown { key: $key_upper }));
                    handle_error!({
                        let bind = state
                            .hotkeys
                            .lookup(Action::$key_upper(state.inputs.ctrl, state.inputs.shift, state.inputs.alt, true));
                        state.execute(bind)
                    });
                } else {
                    handle_error!(state.xpr_mut().event(&KeyUp { key: $key_upper }));
                    handle_error!({
                        let bind = state
                            .hotkeys
                            .lookup(Action::$key_upper(state.inputs.ctrl, state.inputs.shift, state.inputs.alt, false));
                        state.execute(bind)
                    });
                }
            }
        };
    }

    let is_ctrl = ui.io().key_ctrl;
    handle_input!(is_ctrl, Ctrl);

    let is_shift = ui.io().key_shift;
    handle_input!(is_shift, Shift);

    let is_alt = ui.io().key_alt;
    handle_input!(is_alt, Alt);

    macro_rules! expand_handle_input {
        ($($key:ident),*) => {
            $(
                let i = ui.is_key_down(KeyCode::$key as u32);
                handle_input!(i, $key);
            )*
        };
    }

    expand_handle_input!(
        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Return,
        Space, Grave
    );

    // for i in 0..512 {
    //     if ui.is_keys_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.xpr_mut().update_mouse_pos(x.into(), y.into());
}
