use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_frames(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;
    ui.window(&im_str!("Frames"))
        .no_bring_to_front_on_focus(true)
        .position([sz[0] as f32 - RIGHT_SIDE_WIDTH, sz[1] as f32 * 3. / 4. ], Condition::Always)
        .size([RIGHT_SIDE_WIDTH, (sz[1] / 4.) as f32], Condition::Always)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {});
}
