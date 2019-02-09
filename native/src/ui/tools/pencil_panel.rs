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


    let setter_fn = |state: &mut State,  brush: BrushType| match brush {
        BrushType::Pixel | BrushType::Cross => {
            state.xpr.set_option( "brush", brush.as_str()).unwrap();
        }
        BrushType::Circle | BrushType::Square => {
            state.xpr.set_option(
                "brush",
                &format!("{}{}",brush.as_str(), state.brush.sz[0]),
            ).unwrap();
        }
        BrushType::Line => {
            state.xpr.set_option(
                "brush",
                &format!("{}{},{}",brush.as_str(), state.brush.sz[0], state.brush.sz[1]),
            ).unwrap();
        }
    };

    let current_brush = state.xpr.toolbox.pencil.borrow().brush_type;

    ui.tree_node(im_str!("Brush"))
        .default_open(true)
    .build(|| {
        let brushes = BrushType::VARIANTS;
        for (_index, brush) in brushes.iter().enumerate() {
            let is_sel = &current_brush == brush;
            if ui.selectable(
                im_str!("{}", brush.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0., 0.),
            ) {
                setter_fn(state, *brush);
            }
        }
    });


    ui.tree_node(im_str!("Brush"))
        .default_open(true)
        .build(|| {
            if ui.input_int(im_str!("size"), &mut state.brush.sz[0]).build() { setter_fn(state, current_brush); }
            if ui.input_int(im_str!("angle"), &mut state.brush.sz[1]).build() { setter_fn(state, current_brush); }
            macro_rules! angle_btn {
                ($angle: literal) => {
                    if ui.button(
                        im_str!("{}", stringify!($angle)),
                        (0., 0.)
                    ) {
                        state.brush.sz[1] = $angle;
                        setter_fn(state, current_brush);
                    }
                }
            }

            angle_btn!(30);
            ui.same_line(0.);
            angle_btn!(45);
            ui.same_line(0.);
            angle_btn!(60);
            ui.same_line(0.);
            angle_btn!(90);
            ui.same_line(0.);
            angle_btn!(120);
            ui.same_line(0.);
            angle_btn!(135);
            ui.same_line(0.);

        });
}