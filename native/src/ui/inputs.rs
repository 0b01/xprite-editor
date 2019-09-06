#![allow(non_snake_case)]

use crate::prelude::*;

#[allow(unused)]
#[repr(usize)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
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

    if (state.xpr().last_mouse_pos.x != x || state.xpr().last_mouse_pos.y != y) && !state.inputs.Space {
        handle_error!(state.xpr_mut().mouse_move(&MouseMove { x: x.into(), y: y.into() }));
    }

    let left = ui.is_mouse_down(MouseButton::Left);
    let right = ui.is_mouse_down(MouseButton::Right);

    let using_window = ui.is_window_hovered() && !ui.is_item_active();

    if state.inputs.Space {
        ui.set_mouse_cursor(Some(MouseCursor::Hand));
    }

    let is_wheel_dragging = ui.is_mouse_dragging(MouseButton::Middle) || state.inputs.Space;
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
        if state.inputs.LAlt || state.inputs.RAlt {
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
    if state.inputs.debounce(InputItem::LMB, left) && using_window && !state.inputs.Space {
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
    if state.inputs.debounce(InputItem::RMB, right) && using_window {
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
                            .lookup(Action::$key_upper(state.inputs.LControl, state.inputs.LShift, state.inputs.LAlt, true));
                        state.execute(bind)
                    });
                } else {
                    handle_error!(state.xpr_mut().event(&KeyUp { key: $key_upper }));
                    handle_error!({
                        let bind = state
                            .hotkeys
                            .lookup(Action::$key_upper(state.inputs.LControl, state.inputs.LShift, state.inputs.LAlt, false));
                        state.execute(bind)
                    });
                }
            }
        };
    }

    let is_ctrl = ui.io().key_ctrl;
    handle_input!(is_ctrl, LControl);

    let is_shift = ui.io().key_shift;
    handle_input!(is_shift, LShift);

    let is_alt = ui.io().key_alt;
    handle_input!(is_alt, LAlt);

    macro_rules! expand_handle_input {
        ($($key:ident),*) => {
            $(
                let i = ui.is_key_down(KeyCode::$key as u32);
                handle_input!(i, $key);
            )*
        };
    }

    expand_handle_input!(
        Key1,
        Key2,
        Key3,
        Key4,
        Key5,
        Key6,
        Key7,
        Key8,
        Key9,
        Key0,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Escape,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        Snapshot,
        Scroll,
        Pause,
        Insert,
        Home,
        Delete,
        End,
        PageDown,
        PageUp,
        Left,
        Up,
        Right,
        Down,
        Back,
        Return,
        Space,
        Compose,
        Caret,
        Numlock,
        Numpad0,
        Numpad1,
        Numpad2,
        Numpad3,
        Numpad4,
        Numpad5,
        Numpad6,
        Numpad7,
        Numpad8,
        Numpad9,
        AbntC1,
        AbntC2,
        Add,
        Apostrophe,
        Apps,
        At,
        Ax,
        Backslash,
        Calculator,
        Capital,
        Colon,
        Comma,
        Convert,
        Decimal,
        Divide,
        Equals,
        Grave,
        Kana,
        Kanji,
        LAlt,
        LBracket,
        LControl,
        LShift,
        LWin,
        Mail,
        MediaSelect,
        MediaStop,
        Minus,
        Multiply,
        Mute,
        MyComputer,
        NavigateForward,
        NavigateBackward,
        NextTrack,
        NoConvert,
        NumpadComma,
        NumpadEnter,
        NumpadEquals,
        OEM102,
        Period,
        PlayPause,
        Power,
        PrevTrack,
        RAlt,
        RBracket,
        RControl,
        RShift,
        RWin,
        Semicolon,
        Slash,
        Sleep,
        Stop,
        Subtract,
        Sysrq,
        Tab,
        Underline,
        Unlabeled,
        VolumeDown,
        VolumeUp,
        Wake,
        WebBack,
        WebFavorites,
        WebForward,
        WebHome,
        WebRefresh,
        WebSearch,
        WebStop,
        Yen,
        Copy,
        Paste,
        Cut
    );

    // for i in 0..512 {
    //     if ui.is_keys_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.xpr_mut().update_mouse_pos(x.into(), y.into());
}
