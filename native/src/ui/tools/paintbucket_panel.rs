use crate::prelude::*;
use xprite::algorithms::floodfill;
use xprite::tools::paint_bucket;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.tree_node(&im_str!("Degrees")).default_open(true).build(|| {
        let degrees = floodfill::FloodFillDegrees::VARIANTS;
        for (_index, degrees) in degrees.iter().enumerate() {
            let is_sel = &state.xpr_mut().toolbox.paint_bucket.borrow().degrees == degrees;
            if ui.selectable(&im_str!("{}", degrees.as_str()), is_sel, ImGuiSelectableFlags::empty(), [0., 0.]) {
                state.xpr_mut().set_option("degrees", degrees.as_str()).unwrap();
            }
            if ui.is_item_hovered() {
                ui.tooltip_text(degrees.description());
            }
        }
    });

    ui.tree_node(&im_str!("Mode")).default_open(true).build(|| {
        let modes = paint_bucket::PaintBucketMode::VARIANTS;
        for (_index, mode) in modes.iter().enumerate() {
            let is_sel = &state.xpr_mut().toolbox.paint_bucket.borrow().mode == mode;
            if ui.selectable(&im_str!("{}", mode.as_str()), is_sel, ImGuiSelectableFlags::empty(), [0., 0.]) {
                state.xpr_mut().set_option("mode", mode.as_str()).unwrap();
            }

        }
    });
}
