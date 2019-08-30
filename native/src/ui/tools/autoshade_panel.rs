use xprite::algorithms::autoshade::{AutoshadeStepParam, AutoshadeBlendingMode};
use crate::prelude::*;
use std::f64;
use std::rc::Rc;

pub fn draw(_rdr: &mut dyn Renderer, state: &mut State, ui: &Ui) {

    let autoshade = Rc::clone(&state.xpr_mut().toolbox.autoshade);
    let mut tool = autoshade.borrow_mut();
    let len = tool.steps.len();
    if ui.button(&im_str!("+"), [0., 0.]) {
        tool.steps.push(
            AutoshadeStepParam {
                erode: 200.,
                shift: Vec2f{x: -10., y: -10.},
                mode: AutoshadeBlendingMode::Lighten(10),
            }
        );
        tool.finalize(&mut state.xpr_mut()).unwrap();
    }
    for i in 0..len {
        ui.push_id(i as i32);
        let mut erode = tool.steps[i].erode as f32;
        let mut dist_x = tool.steps[i].shift.x as f32;
        let mut dist_y = tool.steps[i].shift.y as f32;

        if ui.small_button(&im_str!("-")) {
            tool.steps.remove(i);
            ui.pop_id();
            return;
        }
        if ui.drag_float(&im_str!("erode"), &mut erode).build() {
            tool.steps[i].erode = erode as f64;
            tool.finalize(&mut state.xpr_mut()).unwrap();
        }
        if ui.drag_float(&im_str!("dist_x"), &mut dist_x).build() {
            tool.steps[i].shift.x = dist_x as f64;
            tool.finalize(&mut state.xpr_mut()).unwrap();
        }
        if ui.drag_float(&im_str!("dist_y"), &mut dist_y).build() {
            tool.steps[i].shift.y = dist_y as f64;
            tool.finalize(&mut state.xpr_mut()).unwrap();
        }

        // // ui.same_line(0.);
        // let col = tool.steps[i].mode;
        // let mut sel: [f32; 4] = col.into();
        // let id = im_str!("MyColor##{}", i);
        // let b = ui.color_edit(id, &mut sel).flags(misc_flags).alpha(false);
        // if b.build() {
        //     tool.steps[i].color = sel.into();
        //     tool.finalize(&mut state.xpr_mut()).unwrap();
        // }

        ui.pop_id();
    }
}
