use xprite::rendering::Renderer;
use crate::prelude::*;

pub fn draw_menu(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            ui.menu_item(im_str!("Load")).shortcut(im_str!("Ctrl+O")).build();
            ui.menu_item(im_str!("Save")).shortcut(im_str!("Ctrl+S")).build();
            if ui.menu_item(im_str!("Settings")).build() {
                state.show_settings = true;
            }
        });
        ui.menu(im_str!("Edit")).build(|| {
            if ui.menu_item(im_str!("Undo")).shortcut(im_str!("Ctrl+Z")).build() {
                state.xpr.undo();
            }
            if ui.menu_item(im_str!("Redo")).shortcut(im_str!("Ctrl+y")).build() {
                state.xpr.redo();
            }
        });
    })
}
