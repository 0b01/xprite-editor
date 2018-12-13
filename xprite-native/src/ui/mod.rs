pub mod inputs;
pub mod tool_panel;
pub mod toolbar;
pub mod menu;
pub mod canvas;
pub mod palette;
pub mod settings;
pub mod tools;
pub mod layers;


use crate::prelude::*;
use xprite::rendering::Renderer;

/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method which takes in a renderer
pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) -> bool {
    self::menu::draw_menu(rdr, state, ui);
    self::toolbar::draw_toolbar(state, ui);
    self::canvas::draw_canvas(rdr, state, ui);
    self::settings::draw_settings(rdr, state, ui);
    self::tool_panel::draw_tool_panel(rdr, state, ui);
    self::palette::draw_palette(rdr, state, ui);
    self::layers::draw_layers(rdr, state, ui);
    true
}
