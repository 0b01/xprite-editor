use crate::prelude::*;

use dyon::{
    load, Dfn, Module, Runtime, Vec4,
    embed::PopVariable
};
use std::sync::{Arc, Mutex};

pub struct Scripting {
}

impl Scripting {
    pub fn new() -> Self {
        Self{}
    }

    pub fn execute(&mut self, xpr: &mut Xprite) -> Option<()> {

        use dyon::{Type, Lt};

        let mut module = Module::new();
        let ty_xpr = Type::AdHoc(Arc::new("XprState".into()), Box::new(Type::Any));
        module.add(Arc::new("xpr_new".into()), new_xpr_state, Dfn{
            lts: vec![],
            tys: vec![],
            ret: ty_xpr.clone()
        });
        match load("scripts/main.dyon", &mut module) {
            Err(msg) => { error!("{}", msg); return Some(()); }
            _ => (),
        };

        let mut runtime = Runtime::new();
        match runtime.call_str_ret("render", &Vec::new(), &Arc::new(module)) {
            Ok(msg) => {
                let i = XprState::pop_var(&runtime, &msg).unwrap();
                info!("{:#?}", i);
            }
            Err(msg) => { error!("{}", msg); return Some(()); }
        };
        Some(())
    }
}

#[derive(Debug)]
struct XprState {
    to_draw: Vec<(Vec4, Vec4)>,
}

dyon_obj! {
    XprState {
        to_draw
    }
}

dyon_fn!{fn new_xpr_state() -> XprState {
    XprState{to_draw: vec![]}
}}
