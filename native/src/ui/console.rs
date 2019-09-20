use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_console(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    if state.show_console {
        Window::new(&im_str!("Console"))
            .size([300., 200.], Condition::Appearing)
            .movable(true)
            .collapsible(true)
            .resizable(true)
            .build(&ui, || {
                ui.text(&state.xpr_mut().log.lock().unwrap().as_str());
            });
    }
}
