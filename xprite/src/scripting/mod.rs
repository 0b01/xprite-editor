use crate::prelude::*;

use dyon::{
    load, load_str, Dfn, Module, Runtime, Vec4,
    Type,
    embed::PopVariable
};
use std::sync::Arc;

#[derive(Default)]
pub struct Scripting {
    pub fname: Option<String>,
}

impl Scripting {
    pub fn new() -> Self {
        Self {
            fname: None,
        }
    }

    pub fn execute(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        let mut module = Module::new();
        let ty_xpr = Type::AdHoc(Arc::new("XprDrawList".into()), Box::new(Type::Any));
        module.add(Arc::new("xpr_new".into()), new_xpr_state, Dfn{
            lts: vec![],
            tys: vec![],
            ret: ty_xpr.clone()
        });

        // load stdlib
        if let Err(msg) = load_str(
            "xpr.dyon",
            Arc::new(include_str!("./xpr.dyon").to_owned()),
            &mut module
        ) {
            return Err(msg);
        }

        if self.fname.is_none() {
            return Err("Fname is not supplied".to_owned());
        }
        let fname = self.fname.as_ref().unwrap();

        if let Err(msg) = load(fname, &mut module) {
            return Err(msg);
        }

        let mut runtime = Runtime::new();
        match runtime.call_str_ret("render", &Vec::new(), &Arc::new(module)) {
            Ok(msg) => {
                let mut buf = Pixels::new();
                let draw_list = XprDrawList::pop_var(&runtime, &msg).unwrap();
                for &(pos, color) in draw_list.to_draw.iter().rev() {
                    let pos : [f32; 4] = pos.into();
                    let color : [f32; 4] = color.into();
                    buf.push(pixel!(pos[0], pos[1], color.into()));
                }
                xpr.history.enter()?;
                let layer = xpr.current_layer_mut().unwrap();
                layer.content.clear();
                layer.content.extend(&buf);

            },
            Err(msg) => return Err(msg),
        };
        Ok(())
    }
}

#[derive(Debug)]
struct XprDrawList {
    to_draw: Vec<(Vec4, Vec4)>,
}

dyon_obj! {
    XprDrawList {
        to_draw
    }
}

dyon_fn!{fn new_xpr_state() -> XprDrawList {
    XprDrawList{
        to_draw: vec![]
    }
}}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_script_execute() {
        let mut xpr = Xprite::new(100., 100.);
        let mut sc = Scripting::new();
        sc.fname = Some("/home/g/Desktop/xprite/scripts/render.dyon".to_owned());
        sc.execute(&mut xpr).unwrap();
    }
}