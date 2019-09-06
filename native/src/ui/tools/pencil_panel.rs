use crate::prelude::*;
use std::rc::Rc;

pub fn draw(state: &mut State, ui: &Ui) {
    // ui.tree_node(&im_str!("Mode")).default_open(true).build(|| {
    //     for (_index, mode) in PencilMode::VARIANTS.iter().enumerate() {
    //         let is_sel = &state.xpr_mut().toolbox.pencil.borrow().mode == mode;
    //         if ui.selectable(&im_str!("{}", mode.as_str()), is_sel, ImGuiSelectableFlags::empty(), [0., 0.]) {
    //             state.xpr_mut().set_option("mode", mode.as_str()).unwrap();
    //         }
    //     }
    // });
    let pencil = Rc::clone(&state.xpr_mut().toolbox.pencil);
    let p = &mut pencil.borrow_mut().processor;

    let mut pp = p.run_pixel_perfect == Some(true);
    let mut pap = p.run_pixel_perfect == Some(false);


    let disabled =  p.sorted_monotonic || p.selective_anti_aliasing;
    if ui.checkbox(&im_str!("Pixel Perfect"), &mut pp) {
        if !disabled {
            if !pp && !pap {
                p.run_pixel_perfect = None;
            } else {
                p.run_pixel_perfect = Some(true);
            }
        }
    }
    if ui.checkbox(&im_str!("Pixel AntiPerfect"), &mut pap) {
        if !disabled {
            if !pp && !pap {
                p.run_pixel_perfect = None;
            } else {
                p.run_pixel_perfect = Some(false);
            }
        }
    }


    if ui.checkbox(&im_str!("Sorted Monotonic"), &mut p.sorted_monotonic) {
        if p.sorted_monotonic {
            p.run_pixel_perfect = Some(true);
        }
    }

    if ui.checkbox(&im_str!("Selective Anti-Aliasing"), &mut p.selective_anti_aliasing) {
        if p.selective_anti_aliasing {
            p.run_pixel_perfect = Some(true);
        }
    }

    let mut sel: [f32; 4] = unsafe { p.aa_alt_color.as_rgba().into() };
    let id = im_str!("MyColor##{}", "background");
    let misc_flags = {
        let mut f = ImGuiColorEditFlags::empty();
        f.set(ImGuiColorEditFlags::HDR, true);
        f.set(ImGuiColorEditFlags::AlphaPreview, true);
        f.set(ImGuiColorEditFlags::NoOptions, false);
        f.set(ImGuiColorEditFlags::NoInputs, true);
        f.set(ImGuiColorEditFlags::NoLabel, true);
        f.set(ImGuiColorEditFlags::NoPicker, false);
        f
    };
    let b = ui.color_edit(&id, &mut sel).flags(misc_flags).alpha(false);
    if b.build() {
        p.aa_alt_color = sel.into();
    }

    if ui.button(&im_str!("toggle brush"), [0., 0.]) {
        state.toggle_brush();
    }
}
