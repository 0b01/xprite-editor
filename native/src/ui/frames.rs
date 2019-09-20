use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_frames(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;
    Window::new(&im_str!("Frames"))
        .bring_to_front_on_focus(false)
        .position([sz[0] as f32 - RIGHT_SIDE_WIDTH, sz[1] as f32 * 3. / 4.], Condition::Always)
        .size([RIGHT_SIDE_WIDTH, (sz[1] / 4.) as f32], Condition::Always)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(&ui, || {
            macro_rules! frames {
                () => {
                    state.xpr_mut().frames_mut();
                };
            };
            let idx = frames!().current_frame_idx;

            if ui.button(&im_str!("+"), [0., 0.]) {
                frames!().add_frame_after_current();
            }

            for i in 0..frames!().count() {
                let txt = &im_str!("{}", i);
                if i % 5 != 0 {
                    ui.same_line(0.);
                }
                let color_token = if i == idx {
                    ui.push_style_colors(&[
                        (StyleColor::Button, XpriteRgba::red().into()),
                        (StyleColor::ButtonHovered, XpriteRgba::red().into()),
                        (StyleColor::ButtonActive, XpriteRgba::red().into()),
                    ])
                } else {
                    ui.push_style_colors(&[
                        (StyleColor::Button, XpriteRgba::black().into()),
                        (StyleColor::ButtonHovered, XpriteRgba::black().into()),
                        (StyleColor::ButtonActive, XpriteRgba::black().into()),
                    ])
                };
                if ui.button(txt, [0., 0.]) {
                    frames!().set_frame_index(i);
                    info!("pressed");
                    state.xpr_mut().set_redraw(true);
                }
                color_token.pop(&ui);
            }
        });
}
