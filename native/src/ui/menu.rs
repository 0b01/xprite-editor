use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_menu(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(&im_str!("File"), true, || {
            if MenuItem::new(&im_str!("New")).shortcut(&im_str!("Ctrl+N")).build(&ui) {
                state.execute(Bind::NewXpr).unwrap();
            }

            if MenuItem::new(&im_str!("Load")).shortcut(&im_str!("Ctrl+O")).build(&ui) {
                state.execute(Bind::Load).unwrap();
            }

            if MenuItem::new(&im_str!("Save")).shortcut(&im_str!("Ctrl+S")).build(&ui) {
                state.execute(Bind::Save).unwrap();
            }

            if MenuItem::new(&im_str!("Settings")).shortcut(&im_str!("Ctrl+,")).build(&ui) {
                state.execute(Bind::PushTool(ToolType::Settings)).unwrap();
            }

            if MenuItem::new(&im_str!("Exporter")).shortcut(&im_str!("Ctrl+E")).build(&ui) {
                state.toggle_exporter();
            }
        });

        ui.menu(&im_str!("Edit"), true, || {
            if MenuItem::new(&im_str!("Undo")).shortcut(&im_str!("Ctrl+Z")).build(&ui) {
                state.xpr_mut().undo();
            }
            if MenuItem::new(&im_str!("Redo")).shortcut(&im_str!("Ctrl+Y")).build(&ui) {
                state.xpr_mut().redo();
            }
        });

        ui.menu(&im_str!("Panels"), true, || {
            if MenuItem::new(&im_str!("Symmetry")).shortcut(&im_str!("Ctrl+Shift+Alt+K")).build(&ui) {
                state.toggle_symmetry();
            }
            if MenuItem::new(&im_str!("Console")).build(&ui) {
                state.toggle_console();
            }
            if MenuItem::new(&im_str!("Brush")).build(&ui) {
                state.toggle_brush();
            }
        });

        ui.menu(&im_str!("Docs"), true, || {
            // if switched, set redraw dirty flg for the new xpr doc
            let mut redraw_idx = None;
            for (i, x) in state.xprs.iter_mut().enumerate() {
                let is_sel = i == state.xpr_idx;
                let pushed_id = ui.push_id(i as i32);
                if MenuItem::new(&im_str!("{}", x.name)).selected(is_sel).build(&ui) {
                    state.xpr_idx = i;
                    redraw_idx = Some(i);
                }
                pushed_id.pop(&ui);
            }
            if let Some(ridx) = redraw_idx {
                state.xprs[ridx].set_redraw(true);
            }
        });

        if cfg!(debug_assertions) {
            ui.text(&im_str!("FPS: {:.2}", ui.io().framerate));
        }
    })
}
