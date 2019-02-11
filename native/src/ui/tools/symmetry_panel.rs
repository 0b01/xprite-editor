use crate::prelude::*;
use std::rc::Rc;
use xprite::algorithms::symmetry::SymmetryMode;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    use SymmetryMode::*;
    let symm = Rc::clone(&state.xpr.toolbox.symmetry);
    let mut tool = symm.borrow_mut();
    for (i, symm) in SymmetryMode::VARIANTS.iter().enumerate() {
        ui.push_id(i as i32);
        ui.same_line(0.);
        if ui.button(im_str!("{}", symm.symbol()), (0.,0.)) {
            tool.add_symmetry(symm.clone()); }
        if ui.is_item_hovered() {
            ui.tooltip_text(symm.as_str());
        }
        ui.pop_id();
    }

    let len = tool.symms.len();
    'out: for i in 0..len {
        ui.push_id(1 + i as i32);
        if ui.button(im_str!("-"), (0., 0.)) {
            info!("removing symmetry {}", i);
            tool.remove_symmetry(i);
            ui.pop_id();
            break 'out;
        }
        ui.same_line(0.);

        let mut show = tool.symms[i].0;
        if ui.checkbox(im_str!("enable"), &mut show) {
            tool.symms[i].0 = show;
            tool.dirty = true
        }

        ui.same_line(0.);
        match &mut tool.symms[i].1 {
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
                if ui.drag_int(im_str!("col"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            AntiDiagonal(m) => {
                ui.text("antidiagonal");
                let mut i = *m as i32;
                if ui.drag_int(im_str!("y-intercept"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            Diagonal(m) => {
                ui.text("diagonal");
                let mut i = *m as i32;
                if ui.drag_int(im_str!("y-intercept"), &mut i).build() {
                    *m = i.into();
                    tool.dirty = true;
                }
            }
            Quad(m, n) => {
                ui.text("quad");
                let mut i = [*m as i32, *n as i32];
                if ui.drag_int2(im_str!("row, col"), &mut i).build() {
                    *m = i[0].into();
                    *n = i[1].into();
                    tool.dirty = true;
                }
            }
            Rotational(pivot, deg, maxn) =>  {
                let mut dirty = false;

                ui.text("rotational");
                let mut i = [pivot.y as i32, pivot.x as i32];
                if ui.drag_int2(im_str!("pivot"), &mut i).build() {
                    pivot.y = i[0].into();
                    pivot.x = i[1].into();
                    dirty = true;
                }
                let mut deg_ = *deg as f32;
                let mut maxn_ = *maxn as i32;
                if ui.drag_float(im_str!("degree"), &mut deg_).build() {
                    *deg = deg_.into();
                    dirty = true;
                }
                if ui.drag_int(im_str!("max"), &mut maxn_).build() {
                    *maxn = maxn_ as u8;
                    dirty = true;
                }

                tool.dirty = dirty;
            }
        }
        ui.pop_id();
    }
}
