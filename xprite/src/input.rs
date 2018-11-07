#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
}

pub enum MouseEvent {
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseDown {
        x: i32,
        y: i32,
        button: MouseButton,
    },
    MouseUp {
        x: i32,
        y: i32,
    },
}
