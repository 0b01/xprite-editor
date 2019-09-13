use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_frames(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;
    ui.window(&im_str!("Frames"))
        .no_bring_to_front_on_focus(true)
        .position([sz[0] as f32 - RIGHT_SIDE_WIDTH, sz[1] as f32 * 3. / 4. ], Condition::Always)
        .size([RIGHT_SIDE_WIDTH, (sz[1] / 4.) as f32], Condition::Always)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(|| {
            macro_rules! frames {
                () => {
                    state.xpr_mut().frames_mut();
                }
            };
            let idx = frames!().current_frame_idx;

            if ui.button(&im_str!("+"), [0., 0.]) {
                frames!().add_frame_after_current();
            }

            for i in 0..frames!().count() {
                let txt = &im_str!("{}", i);
                if i == idx {
                    ui.text(txt);
                } else {
                    if ui.button(txt, [0., 0.]) {
                        frames!().set_frame_index(i);
                        info!("pressed");
                        state.xpr_mut().set_redraw(true);
                    }
                }
            }
        });
}
