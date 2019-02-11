use crate::prelude::*;
use std::rc::Rc;
use xprite::algorithms::symmetry::SymmetryMode;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    use SymmetryMode::*;
    let symm = Rc::clone(&state.xpr.toolbox.symmetry);
    let mut tool = symm.borrow_mut();
    if ui.button(im_str!("|"), (0.,0.)) {
        tool.add_symmetry(SymmetryMode::Vertical(10.));
    }
    ui.same_line(0.);
    if ui.button(im_str!("-"), (0.,0.)) {
        tool.add_symmetry(SymmetryMode::Horizontal(10.)); }
    ui.same_line(0.);
    if ui.button(im_str!("/"), (0.,0.)) {
        tool.add_symmetry(SymmetryMode::AntiDiagonal(10.)); }
    ui.same_line(0.);
    if ui.button(im_str!("\\"), (0.,0.)) {
        tool.add_symmetry(SymmetryMode::Diagonal(10.)); }
    ui.same_line(0.);
    if ui.button(im_str!("+"), (0.,0.)) {
        tool.add_symmetry(SymmetryMode::Quad(10., 10.)); }

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
        }
        ui.pop_id();
    }
}
