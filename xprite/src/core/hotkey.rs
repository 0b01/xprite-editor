use crate::tools::ToolType;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum Bind {
    Redo,
    Undo,
    PushTool(ToolType),
    PopTool,
    ToggleConsole,
    RunScript,
    Save,
    Load,
    NewXpr,
    CloseXpr(usize),
    Unmapped,
}

macro_rules! declare_actions {
    ($($field:ident),*) =>{

        #[derive(Hash, PartialEq, Clone, Eq, Debug)]
        pub enum Action {
            $(
                /// ctrl, shift, alt, is_down
                $field(bool, bool, bool, bool),
            )*
        }
    }
}

declare_actions!(
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

pub struct HotkeyController {
    binds: HashMap<Action, Bind>,
    pub enabled: bool,
}

impl HotkeyController {
    pub fn new() -> Self {
        let mut binds = HashMap::new();

        {
            // TODO: initialize in the right place
            binds.insert(Action::Z(true, false, false, true), Bind::Undo);
            binds.insert(Action::Z(true, true, false, true), Bind::Redo);
            binds.insert(Action::Y(true, false, false, true), Bind::Redo);

            binds.insert(Action::Grave(false, false, false, true), Bind::ToggleConsole);

            // tools
            binds.insert(Action::B(false, false, false, true), Bind::PushTool(ToolType::Pencil));
            binds.insert(Action::G(false, false, false, true), Bind::PushTool(ToolType::PaintBucket));
            binds.insert(Action::L(false, false, false, true), Bind::PushTool(ToolType::Line));
            binds.insert(Action::E(false, false, false, true), Bind::PushTool(ToolType::Eraser));
            binds.insert(Action::V(false, false, false, true), Bind::PushTool(ToolType::Vector));
            binds.insert(Action::R(false, false, false, true), Bind::PushTool(ToolType::Rect));
            binds.insert(Action::U(false, false, false, true), Bind::PushTool(ToolType::Ellipse));
            binds.insert(Action::T(false, false, false, true), Bind::PushTool(ToolType::Texture));
            binds.insert(Action::M(false, false, false, true), Bind::PushTool(ToolType::Marquee));

            // alt
            binds.insert(Action::LAlt(false, false, true, true), Bind::PushTool(ToolType::ColorPicker));
            binds.insert(Action::LAlt(false, false, false, false), Bind::PopTool);

            binds.insert(Action::Return(true, false, false, true), Bind::RunScript);
            binds.insert(Action::N(true, false, false, true), Bind::NewXpr);
            // ctrl-s
            binds.insert(Action::S(true, false, false, true), Bind::Save);
            binds.insert(Action::O(true, false, false, true), Bind::Load);
            binds.insert(Action::S(true, true, false, true), Bind::Save);
            binds.insert(Action::O(true, true, false, true), Bind::Load);
        }

        Self { binds, enabled: true }
    }

    pub fn lookup(&self, action: Action) -> Bind {
        if !self.enabled {
            return Bind::Unmapped;
        }
        self.binds.get(&action).cloned().unwrap_or_else(|| {
            trace!("unmapped action: {:?}", action);
            Bind::Unmapped
        })
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}
