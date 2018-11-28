use crate::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum Bind {
    Redo,
    Undo,
    Unmapped,
}

impl Bind {
    pub fn execute(&self, xpr: &mut Xprite) -> Option<()> {
        use self::Bind::*;
        match self {
            Redo => xpr.redo(),
            Undo => xpr.undo(),
            Unmapped => error!("unmapped key bind"),
        }
        Some(())
    }
}

#[derive(Hash, PartialEq, Clone, Eq)]
pub enum Action {
    /// ctrl, shift, alt
    Z(bool, bool, bool),
    Y(bool, bool, bool),
    Alt(bool, bool, bool),
    Shift(bool, bool, bool),
    Ctrl(bool, bool, bool),
    Space(bool, bool, bool),
    Enter(bool, bool, bool),
}

pub struct HotkeyController {
    binds: HashMap<Action, Bind>
}

impl HotkeyController {
    pub fn new() -> Self {
        let mut binds = HashMap::new();
        binds.insert( Action::Z(true, false, false), Bind::Undo );
        binds.insert( Action::Z(true, true, false),  Bind::Redo );
        binds.insert( Action::Y(true, false, false), Bind::Redo );
        Self {
            binds,
        }
    }

    pub fn lookup(&self, action: Action) -> Bind {
        self.binds
            .get(&action)
            .cloned()
            .unwrap_or(Bind::Unmapped)
    }
}

