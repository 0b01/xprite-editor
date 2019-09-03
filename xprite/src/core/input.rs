#[derive(Debug)]
pub enum InputEvent {
    MouseMove { x: f64, y: f64 },
    MouseDown { x: f64, y: f64, button: InputItem },
    MouseUp { x: f64, y: f64, button: InputItem },
    KeyUp { key: InputItem },
    KeyDown { key: InputItem },
}

macro_rules! declare_input_state {
    ($($field:ident),*) =>{

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum InputItem {
            $(
                $field,
            )*
        }

        impl InputItem {
            pub fn as_str(&self) -> &str {
                match &self {
                    $(
                        InputItem::$field => stringify!($field),
                    )*
                }
            }
        }


        pub struct InputState {
            $(
                pub $field: bool,
            )*
        }

        impl Default for InputState {
            fn default() -> Self {
                Self {
                    $(
                        $field: false,
                    )*
                }
            }
        }

        impl InputState {
            pub fn debounce(&mut self, item: InputItem, new_state: bool) -> bool {
                macro_rules! debounce {
                    ($a:ident) => {
                        if self.$a != new_state {
                            self.$a = !self.$a;
                            true
                        } else {
                            false
                        }
                    };
                }
                match item {
                    $(
                        InputItem::$field => debounce!($field),
                    )*
                }
            }
        }

    };
}

declare_input_state!(Key1,
    Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Left, Up, Right, Down, Back, Return, Space, Compose, Caret, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl, LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NavigateForward, NavigateBackward, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, PlayPause, Power, PrevTrack, RAlt, RBracket, RControl, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract, Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop, Yen, Copy, Paste, Cut
);