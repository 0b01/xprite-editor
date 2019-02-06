use crate::prelude::*;
use xprite::tools::paint_bucket;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Mode")).build(|| {
        let modes = paint_bucket::PaintBucketMode::VARIANTS;
        for (_index, mode) in modes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.paint_bucket.borrow().mode == mode;
            if ui.selectable(
                im_str!("{}", mode.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0., 0.),
            ) {
                state.xpr.set_option("mode", mode.as_str()).unwrap();
            }
        }
    });
}
