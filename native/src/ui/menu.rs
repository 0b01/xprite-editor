use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_menu(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            if ui
                .menu_item(im_str!("Load"))
                .shortcut(im_str!("Ctrl+O"))
                .build()
            {
                state.load_xpr("1.xpr");
            }
            if ui
                .menu_item(im_str!("Save"))
                .shortcut(im_str!("Ctrl+S"))
                .build()
            {
                state.save_xpr("1.xpr");
            }
            if ui.menu_item(im_str!("Settings")).build() {
                state.show_settings = true;
            }
        });

        ui.menu(im_str!("Edit")).build(|| {
            if ui
                .menu_item(im_str!("Undo"))
                .shortcut(im_str!("Ctrl+Z"))
                .build()
            {
                state.xpr.undo();
            }
            if ui
                .menu_item(im_str!("Redo"))
                .shortcut(im_str!("Ctrl+y"))
                .build()
            {
                state.xpr.redo();
            }
        });
    })
}
