#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
}

pub enum MouseEvent {
    MouseMove {
        x: f32,
        y: f32,
    },
    MouseDown {
        x: f32,
        y: f32,
        button: MouseButton,
    },
    MouseUp {
        x: f32,
        y: f32,
    },
}
