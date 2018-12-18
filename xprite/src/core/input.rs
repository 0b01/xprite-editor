#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InputItem {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
    W, X, Y, Z, Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    Ctrl,
    Shift,
    Space,
    Alt,
    Left,
    Right,
    Return,
    Grave,
}

impl InputItem {
    pub fn as_str(&self) -> &str {
        match &self {
            InputItem::Ctrl => "ctrl", InputItem::Shift => "shift",
            InputItem::Space => "space",
            InputItem::Alt => "alt",
            InputItem::Left => "left",
            InputItem::Right => "right",
            InputItem::Return => "return",
            InputItem::A => "a",
            InputItem::B => "b",
            InputItem::C => "c",
            InputItem::D => "d",
            InputItem::E => "e",
            InputItem::F => "f",
            InputItem::G => "g",
            InputItem::H => "h",
            InputItem::I => "i",
            InputItem::J => "j",
            InputItem::K => "k",
            InputItem::L => "l",
            InputItem::M => "m",
            InputItem::N => "n",
            InputItem::O => "o",
            InputItem::P => "p",
            InputItem::Q => "q",
            InputItem::R => "r",
            InputItem::S => "s",
            InputItem::T => "t",
            InputItem::U => "u",
            InputItem::V => "v",
            InputItem::W => "w",
            InputItem::X => "x",
            InputItem::Y => "y",
            InputItem::Z => "z",
            InputItem::Key0 => "0",
            InputItem::Key1 => "1",
            InputItem::Key2 => "2",
            InputItem::Key3 => "3",
            InputItem::Key4 => "4",
            InputItem::Key5 => "5",
            InputItem::Key6 => "6",
            InputItem::Key7 => "7",
            InputItem::Key8 => "8",
            InputItem::Key9 => "9",

            InputItem::Grave => "`",
        }
    }
}


#[derive(Debug)]
pub enum InputEvent {
    MouseMove {
        x: f32,
        y: f32,
    },
    MouseDown {
        x: f32,
        y: f32,
        button: InputItem,
    },
    MouseUp {
        x: f32,
        y: f32,
        button: InputItem,
    },
    KeyUp {
        key: InputItem,
    },
    KeyDown {
        key: InputItem,
    }
}


pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub space: bool,
    pub alt: bool,
    pub enter: bool,

    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
    pub h: bool,
    pub i: bool,
    pub j: bool,
    pub k: bool,
    pub l: bool,
    pub m: bool,
    pub n: bool,
    pub o: bool,
    pub p: bool,
    pub q: bool,
    pub r: bool,
    pub s: bool,
    pub t: bool,
    pub u: bool,
    pub v: bool,
    pub w: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub key0: bool,
    pub key1: bool,
    pub key2: bool,
    pub key3: bool,
    pub key4: bool,
    pub key5: bool,
    pub key6: bool,
    pub key7: bool,
    pub key8: bool,
    pub key9: bool,

    pub grave: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            ctrl: false,
            shift: false,
            space: false,
            alt: false,
            enter: false,
            a: false,
            b: false,
            c: false,
            d: false,
            e: false,
            f: false,
            g: false,
            h: false,
            i: false,
            j: false,
            k: false,
            l: false,
            m: false,
            n: false,
            o: false,
            p: false,
            q: false,
            r: false,
            s: false,
            t: false,
            u: false,
            v: false,
            w: false,
            x: false,
            y: false,
            z: false,
            key0: false,
            key1: false,
            key2: false,
            key3: false,
            key4: false,
            key5: false,
            key6: false,
            key7: false,
            key8: false,
            key9: false,

            grave: false,
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
            InputItem::Ctrl => debounce!(ctrl),
            InputItem::Shift => debounce!(shift),
            InputItem::Space => debounce!(space),
            InputItem::Alt => debounce!(alt),
            InputItem::Left => debounce!(left),
            InputItem::Right => debounce!(right),
            InputItem::Return => debounce!(enter),

            InputItem::A => debounce!(a),
            InputItem::B => debounce!(b),
            InputItem::C => debounce!(c),
            InputItem::D => debounce!(d),
            InputItem::E => debounce!(e),
            InputItem::F => debounce!(f),
            InputItem::G => debounce!(g),
            InputItem::H => debounce!(h),
            InputItem::I => debounce!(i),
            InputItem::J => debounce!(j),
            InputItem::K => debounce!(k),
            InputItem::L => debounce!(l),
            InputItem::M => debounce!(m),
            InputItem::N => debounce!(n),
            InputItem::O => debounce!(o),
            InputItem::P => debounce!(p),
            InputItem::Q => debounce!(q),
            InputItem::R => debounce!(r),
            InputItem::S => debounce!(s),
            InputItem::T => debounce!(t),
            InputItem::U => debounce!(u),
            InputItem::V => debounce!(v),
            InputItem::W => debounce!(w),
            InputItem::X => debounce!(x),
            InputItem::Y => debounce!(y),
            InputItem::Z => debounce!(z),
            InputItem::Key0 => debounce!(key0),
            InputItem::Key1 => debounce!(key1),
            InputItem::Key2 => debounce!(key2),
            InputItem::Key3 => debounce!(key3),
            InputItem::Key4 => debounce!(key4),
            InputItem::Key5 => debounce!(key5),
            InputItem::Key6 => debounce!(key6),
            InputItem::Key7 => debounce!(key7),
            InputItem::Key8 => debounce!(key8),
            InputItem::Key9 => debounce!(key9),

            InputItem::Grave => debounce!(grave),
        }
    }
}
