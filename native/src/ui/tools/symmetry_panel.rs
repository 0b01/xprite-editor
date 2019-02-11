use crate::prelude::*;
use std::rc::Rc;
use xprite::algorithms::symmetry::SymmetryMode;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    use SymmetryMode::*;
    let symm = Rc::clone(&state.xpr.toolbox.symmetry);
    let mut tool = symm.borrow_mut();
    for symm in &SymmetryMode::VARIANTS {
        ui.same_line(0.);
        if ui.button(im_str!("{}", symm.symbol()), (0.,0.)) {
            tool.add_symmetry(symm.clone()); }
        ui.tooltip(|| {
            ui.text(symm.as_str());
        })
    }

    let len = tool.symms.len();
    'out: for i in 0..len {
        ui.push_id(i as i32);
        if ui.button(im_str!("-"), (0., 0.)) {
            tool.remove_symmetry(i);
            ui.pop_id();
            break 'out;
        }
        ui.same_line(0.);
        match &mut tool.symms[i] {
            Horizontal(m) => {
                ui.text("horizontal");
                let mut i = *m as i32;
                if ui.drag_int(im_str!("row"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            Vertical(m) => {
                ui.text("vertical");
                let mut i = *m as i32;
                if ui.drag_int(im_str!("row"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            AntiDiagonal(m) => {
                ui.text("antidiagonal");
                let mut i = *m as i32;
                if ui.drag_int(im_str!("row"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            Diagonal(m) => {
                ui.text("diagonal");
                let mut i = *m as i32;
                if ui.drag_int(im_str!("row"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            Quad(m, n) => {
                ui.text("quad");
                let mut i = [*m as i32, *n as i32];
                if ui.drag_int2(im_str!("row"), &mut i).build() {
                    *m = i[0].into();
                    *n = i[1].into();
                    tool.dirty = true;
                }
            }
            Rotational(pivot, deg, maxn) =>  {
                // ...
            }
        }
        ui.pop_id();
    }
}
