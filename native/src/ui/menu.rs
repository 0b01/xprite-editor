use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_menu(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(&im_str!("File")).build(|| {
            if ui.menu_item(&im_str!("New")).shortcut(&im_str!("Ctrl+N")).build() {
                state.execute(Bind::NewXpr).unwrap();
            }

            if ui.menu_item(&im_str!("Load")).shortcut(&im_str!("Ctrl+O")).build() {
                state.execute(Bind::Load).unwrap();
            }

            if ui.menu_item(&im_str!("Save")).shortcut(&im_str!("Ctrl+S")).build() {
                state.execute(Bind::Save).unwrap();
            }

            if ui.menu_item(&im_str!("Settings")).build() {
                state.execute(Bind::PushTool(ToolType::Settings)).unwrap();
            }

            if ui.menu_item(&im_str!("Exporter")).build() {
                state.toggle_exporter();
            }
        });

        ui.menu(&im_str!("Edit")).build(|| {
            if ui.menu_item(&im_str!("Undo")).shortcut(&im_str!("Ctrl+Z")).build() {
                state.xpr_mut().undo();
            }
            if ui.menu_item(&im_str!("Redo")).shortcut(&im_str!("Ctrl+y")).build() {
                state.xpr_mut().redo();
            }
        });

        ui.menu(&im_str!("Panels")).build(|| {
            if ui.menu_item(&im_str!("Symmetry")).build() {
                state.toggle_symmetry();
            }
            if ui.menu_item(&im_str!("Console")).build() {
                state.toggle_console();
            }
            if ui.menu_item(&im_str!("Brush")).build() {
                state.toggle_brush();
            }
        });

        ui.menu(&im_str!("Docs")).build(|| {
            // if switched, set redraw dirty flg for the new xpr doc
            let mut redraw_idx = None;
            for (i, x) in state.xprs.iter_mut().enumerate() {
                let mut is_sel = i == state.xpr_idx;
                ui.push_id(i as i32);
                if ui.menu_item(&im_str!("{}", x.name)).selected(&mut is_sel).build() {
                    state.xpr_idx = i;
                    redraw_idx = Some(i);
                }
                ui.pop_id();
            }
            if let Some(ridx) = redraw_idx {
                state.xprs[ridx].redraw = true;
            }
        });

        if cfg!(debug_assertions) {
            ui.text(&im_str!("FPS: {:.2}", ui.io().framerate));
        }
    })
}
