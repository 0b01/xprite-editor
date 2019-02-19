pub mod canvas;
pub mod console;
pub mod file_manager;
pub mod inputs;
pub mod layers;
pub mod menu;
pub mod palette;
pub mod preview;
pub mod tool_panel;
pub mod toolbar;
pub mod exporter;
pub mod tools;
pub mod symmetry;
pub mod brush;

use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;

/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method
pub fn draw(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) -> bool {
    state.xpr_mut().draw().unwrap();
    state.redraw_pixels(rdr).unwrap();
    state.xpr_mut().update().unwrap();

    self::file_manager::draw_file_manager(rdr, state, ui);
    self::menu::draw_menu(rdr, state, ui);
    self::toolbar::draw_toolbar(state, ui);
    self::canvas::draw_canvas(rdr, state, ui);
    self::tool_panel::draw_tool_panel(rdr, state, ui);
    self::palette::draw_palette(rdr, state, ui);
    self::palette::draw_color_picker(rdr, state, ui);
    self::layers::draw_layers(rdr, state, ui);

    self::symmetry::draw_symmetry(rdr, state, ui);
    self::brush::draw_brush(rdr, state, ui);
    self::console::draw_console(rdr, state, ui);

    self::preview::draw_preview(rdr, state, ui);
    self::exporter::draw_exporter(rdr, state, ui);
    true
}
