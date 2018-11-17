pub mod tool_panel;
pub mod toolbar;
pub mod menu;
pub mod canvas;
pub mod settings;
pub mod tools;

use crate::prelude::*;
use xprite::rendering::Renderer;

/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method which takes in a renderer
pub fn draw(rdr: &Renderer, state: &mut State, ui: &Ui) -> bool {
    self::menu::draw_menu(rdr, state, ui);
    self::toolbar::draw_toolbar(state, ui);
    self::canvas::draw_canvas(rdr, state, ui);
    self::settings::draw_settings(rdr, state, ui);
    self::tool_panel::tool_panel(rdr, state, ui);
    true
}
