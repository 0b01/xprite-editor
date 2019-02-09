use crate::prelude::*;
use xprite::tools::pencil;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Mode"))
        .default_open(true)
    .build(|| {
        let modes = pencil::PencilMode::VARIANTS;
        for (_index, mode) in modes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.pencil.borrow().mode == mode;
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

    ui.tree_node(im_str!("Brush"))
        .default_open(true)
    .build(|| {
        let brushes = BrushType::VARIANTS;
        for (_index, brush) in brushes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.pencil.borrow().brush_type == brush;
            match brush {
                BrushType::Pixel | BrushType::Cross => {
                    if ui.selectable(
                        im_str!("{}", brush.as_str()),
                        is_sel,
                        ImGuiSelectableFlags::empty(),
                        (0., 0.),
                    ) {
                        state.xpr.set_option("brush", brush.as_str()).unwrap();
                    }
                }

                BrushType::Circle | BrushType::Square => {
                    let popup_name = im_str!("brush{}", brush.as_str());
                    if ui.selectable(
                        im_str!("{}", brush.as_str()),
                        is_sel,
                        ImGuiSelectableFlags::empty(),
                        (0., 0.),
                    ) {
                        ui.open_popup(popup_name);
                    }

                    ui.popup(popup_name, ||{
                        if ui.input_int(im_str!("size"), &mut state.brush_sz).build() {
                            state.xpr.set_option(
                                "brush",
                                &format!("{}{}",brush.as_str(), state.brush_sz),
                            ).unwrap();
                        }
                    });
                }
            }
        }
    });
}
