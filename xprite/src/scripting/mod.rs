use crate::prelude::*;

use dyon::{load, Dfn, Module, Runtime, Vec4};
use std::sync::{Arc};

pub struct Scripting {
    runtime: dyon::Runtime,
}

impl Scripting {
    pub fn new() -> Self {
        let runtime = Runtime::new();
        Self {
            runtime,
        }
    }

    pub fn execute(&mut self, xpr: &mut Xprite) -> Option<()> {

        use dyon::{Type, Lt};

        let mut module = Module::new();
        module.add(Arc::new("say_hello".to_owned()), say_hello, Dfn{ lts: vec![], tys:vec![], ret: Type::Void});
        module.add(Arc::new("pixel".to_owned()), pixel, Dfn{
            lts: vec![Lt::Default, Lt::Default],
            tys:vec![Type::Vec4, Type::Vec4],
            ret: Type::Void
        });

        match load("scripts/main.dyon", &mut module) {
            Err(msg) => { error!("{}", msg); return Some(()); }
            _ => (),
        };

        match self.runtime.run(&Arc::new(module)) {
            Err(msg) => { error!("{}", msg); return Some(()); }
            _ => (),
        };
        Some(())
    }
}

dyon_fn!{fn pixel(v: Vec4, col: Vec4) {
    println!("{:?} {:?}", v, col);
}}


dyon_fn!{fn say_hello() {
    println!("hi!");
}}
