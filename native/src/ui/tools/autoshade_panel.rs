use crate::prelude::*;
use std::f64;
use std::rc::Rc;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {

    let misc_flags = {
        let mut f = ImGuiColorEditFlags::empty();
        f.set(ImGuiColorEditFlags::HDR, true);
        f.set(ImGuiColorEditFlags::AlphaPreview, true);
        f.set(ImGuiColorEditFlags::NoOptions, false);
        f.set(ImGuiColorEditFlags::NoInputs, true);
        f.set(ImGuiColorEditFlags::NoLabel, true);
        f.set(ImGuiColorEditFlags::NoPicker, true);
        f
    };

    let autoshade = Rc::clone(&state.xpr_mut().toolbox.autoshade);
    let mut tool = autoshade.borrow_mut();
    let len = tool.steps.len();
    if ui.button(im_str!("+"), (0.,0.)) {
        tool.steps.push((200., 0.03, Color::red()));
        tool.finalize(&mut state.xpr_mut()).unwrap();
    }
    for i in 0..len {
        let mut corrode = tool.steps[i].0 as f32;
        let mut dist = tool.steps[i].1 as f32;

        if ui.drag_float(im_str!("corrode"), &mut corrode).build() {
            tool.steps[i].0 = corrode as f64;
            tool.finalize(&mut state.xpr_mut()).unwrap();
        }
        if ui.drag_float(im_str!("dist"), &mut dist).build() {
            tool.steps[i].1 = dist as f64;
            tool.finalize(&mut state.xpr_mut()).unwrap();
        }

        // ui.same_line(0.);
        let col = tool.steps[i].2;
        let mut sel: [f32; 4] = col.into();
        let id = im_str!("MyColor##{}", i);
        let b = ui.color_edit(id, &mut sel)
            .flags(misc_flags)
            .alpha(false);
        if b.build() {
            tool.steps[i].2 = sel.into();
            tool.finalize(&mut state.xpr_mut()).unwrap();
        }
    }
}