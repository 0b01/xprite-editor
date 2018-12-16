use dyon::{load, Dfn, Module, Runtime};
use std::sync::Arc;

pub struct Scripting {
    module: Arc<Module>,
    runtime: dyon::Runtime,
}

impl Scripting {
    pub fn new() -> Self {
        let mut module = Module::new();
        module.add(Arc::new("say_hello".to_owned()), say_hello, Dfn{ lts: vec![], tys:vec![], ret: dyon::Type::Void});

        load("scripts/main.dyon", &mut module).unwrap();

        let runtime = Runtime::new();

        let module = Arc::new(module);

        Self {
            runtime,
            module,
        }
    }

    pub fn execute(&mut self) -> () {
        self.runtime.run(&self.module).unwrap();
    }
}


dyon_fn!{fn say_hello() {
    println!("hi!");
}}