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

    let wheel_delta = ui.imgui().mouse_wheel() as f64 / 5.;
    let (x, y) = ui.imgui().mouse_pos();
    let x = x.into();
    let y = y.into();

    if (state.xpr.last_mouse_pos.x != x || state.xpr.last_mouse_pos.y != y) && !state.inputs.space {
        handle_error!(state.xpr.mouse_move(&MouseMove {
            x: x.into(),
            y: y.into()
        }));
    }

    let left = ui.imgui().is_mouse_down(ImMouseButton::Left);
    let right = ui.imgui().is_mouse_down(ImMouseButton::Right);

    let using_window = ui.is_window_hovered() && !ui.is_item_active();

    if state.inputs.space {
        ui.imgui().set_mouse_cursor(ImGuiMouseCursor::Hand);
    }

    // middle key for scrolling
    if using_window
        && (ui.imgui().is_mouse_dragging(ImMouseButton::Middle)
            || (state.inputs.space && state.inputs.left))
    {
        // set cursor
        ui.imgui().set_mouse_cursor(ImGuiMouseCursor::Hand);
        let d = ui.imgui().mouse_delta();
        state.xpr.canvas.scroll.x += d.0 as f64;
        state.xpr.canvas.scroll.y += d.1 as f64;
    }

    if using_window {
        state.xpr.canvas.update_zoom(wheel_delta, (x, y))
    }
    if state.xpr.canvas.scale > 100. {
        state.xpr.canvas.scale = 100.;
    }
    if state.xpr.canvas.scale < 0. {
        state.xpr.canvas.scale = 1.;
    }

    // left
    if state.inputs.debounce(InputItem::Left, left) && using_window && !state.inputs.space {
        if left {
            trace!("mouse left down");
            handle_error!(state.xpr.event(&MouseDown {
                x: x.into(),
                y: y.into(),
                button: Left
            }));
        } else {
            trace!("mouse left up");
            handle_error!(state.xpr.event(&MouseUp {
                x: x.into(),
                y: y.into(),
                button: Right
            }));
        }
    }

    // right
    if state.inputs.debounce(InputItem::Right, right) && using_window {
        if right {
            let (x, y) = ui.imgui().mouse_pos();
            handle_error!(state.xpr.event(&MouseDown {
                x: x.into(),
                y: y.into(),
                button: Right
            }));
        } else {
            trace!("mouse right up");
            handle_error!(state.xpr.event(&MouseUp {
                x: x.into(),
                y: y.into(),
                button: Right
            }));
        }
    }

    macro_rules! handle_input {
        ($boolval: expr, $key_upper: ident) => {
            if state.inputs.debounce(InputItem::$key_upper, $boolval) {
                if $boolval {
                    handle_error!(state.xpr.event(&KeyDown { key: $key_upper }));
                    handle_error!({
                        let bind = state.hotkeys.lookup(Action::$key_upper(
                            state.inputs.ctrl,
                            state.inputs.shift,
                            state.inputs.alt,
                            true,
                        ));
                        state.execute(bind)
                    });
                } else {
                    handle_error!(state.xpr.event(&KeyUp { key: $key_upper }));
                    handle_error!({
                        let bind = state.hotkeys.lookup(Action::$key_upper(
                            state.inputs.ctrl,
                            state.inputs.shift,
                            state.inputs.alt,
                            false,
                        ));
                        state.execute(bind)
                    });
                }
            }
        };
    }

    let is_ctrl = ui.imgui().key_ctrl();
    handle_input!(is_ctrl, Ctrl);

    let is_shift = ui.imgui().key_shift();
    handle_input!(is_shift, Shift);

    let is_alt = ui.imgui().key_alt();
    handle_input!(is_alt, Alt);

    macro_rules! expand_handle_input {
        ($($key:ident),*) => {
            $(
                let i = ui.imgui().is_key_down(KeyCode::$key as usize);
                handle_input!(i, $key);
            )*
        };
    }

    expand_handle_input!(
        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Key0, Key1,
        Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Return, Space, Grave
    );

    // for i in 0..512 {
    //     if ui.imgui().is_key_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.xpr.update_mouse_pos(x.into(), y.into());
}
