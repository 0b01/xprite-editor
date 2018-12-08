use crate::prelude::*;
use xprite::tools::ToolType;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum Bind {
    Redo,
    Undo,
    Tool(ToolType),
    Unmapped,
}

impl Bind {
    pub fn execute(&self, xpr: &mut Xprite) -> Option<()> {
        use self::Bind::*;
        match self {
            Redo => xpr.redo(),
            Undo => xpr.undo(),
            Tool(tool) => xpr.change_tool(&tool),
            Unmapped => (),
        }
        Some(())
    }
}

#[derive(Hash, PartialEq, Clone, Eq, Debug)]
pub enum Action {
    /// ctrl, shift, alt
    A(bool, bool, bool),
    B(bool, bool, bool),
    C(bool, bool, bool),
    D(bool, bool, bool),
    E(bool, bool, bool),
    F(bool, bool, bool),
    G(bool, bool, bool),
    H(bool, bool, bool),
    I(bool, bool, bool),
    J(bool, bool, bool),
    K(bool, bool, bool),
    L(bool, bool, bool),
    M(bool, bool, bool),
    N(bool, bool, bool),
    O(bool, bool, bool),
    P(bool, bool, bool),
    Q(bool, bool, bool),
    R(bool, bool, bool),
    S(bool, bool, bool),
    T(bool, bool, bool),
    U(bool, bool, bool),
    V(bool, bool, bool),
    W(bool, bool, bool),
    X(bool, bool, bool),
    Y(bool, bool, bool),
    Z(bool, bool, bool),
    Key0(bool, bool, bool),
    Key1(bool, bool, bool),
    Key2(bool, bool, bool),
    Key3(bool, bool, bool),
    Key4(bool, bool, bool),
    Key5(bool, bool, bool),
    Key6(bool, bool, bool),
    Key7(bool, bool, bool),
    Key8(bool, bool, bool),
    Key9(bool, bool, bool),

    Alt(bool, bool, bool),
    Shift(bool, bool, bool),
    Ctrl(bool, bool, bool),
    Space(bool, bool, bool),
    Return(bool, bool, bool),
}

pub struct HotkeyController {
    binds: HashMap<Action, Bind>
}

impl HotkeyController {
    pub fn new() -> Self {
        let mut binds = HashMap::new();

        {
            // TODO: initialize in the right place
            binds.insert( Action::Z(true, false, false), Bind::Undo );
            binds.insert( Action::Z(true, true, false),  Bind::Redo );
            binds.insert( Action::Y(true, false, false), Bind::Redo );

            // tools
            binds.insert( Action::B(false, false, false), Bind::Tool(ToolType::Pencil) );
            binds.insert( Action::G(false, false, false), Bind::Tool(ToolType::PaintBucket) );
            binds.insert( Action::L(false, false, false), Bind::Tool(ToolType::Line) );

            // alt
            binds.insert( Action::Alt(false, false, true), Bind::Tool(ToolType::ColorPicker) );
        }

        Self {
            binds,
        }
    }

    pub fn lookup(&self, action: Action) -> Bind {
        self.binds
            .get(&action)
            .cloned()
            .unwrap_or_else( || {
                error!("unmapped action: {:?}", action);
                Bind::Unmapped
            })
    }
}

