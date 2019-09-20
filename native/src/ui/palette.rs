use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_palette(rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;

    let color = ui.push_style_colors(&vec![(StyleColor::WindowBg, BLACK)]);
    Window::new(&im_str!("Palette"))
        .bring_to_front_on_focus(false)
        .position([0., TOOLBOX_H], Condition::Always)
        .size([LEFT_SIDE_WIDTH, sz[1] as f32 - TOOLBOX_H - COLOR_PICKER_H], Condition::Always)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(&ui, || {
            // ui.slider_int(&im_str!("Colors per row"), &mut state.cols_per_row, 2, 16).build();
            let styles = vec![
                StyleVar::ChildBorderSize(0.),
                StyleVar::FrameBorderSize(0.),
                StyleVar::WindowBorderSize(0.),
                StyleVar::PopupBorderSize(0.),
                StyleVar::FrameRounding(0.),
            ];
            let style = ui.push_style_vars(&styles);
            draw_cells(rdr, state, ui);
            style.pop(&ui);
        });
    color.pop(&ui);
}

pub fn draw_color_picker(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;

    Window::new(&im_str!("Color Picker"))
        .bring_to_front_on_focus(false)
        .position([0., sz[1] as f32 - COLOR_PICKER_H], Condition::Always)
        .size([LEFT_SIDE_WIDTH, COLOR_PICKER_H], Condition::Always)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(&ui, || {
            let misc_flags = {
                let mut f = ColorEditFlags::empty();
                f.set(ColorEditFlags::ALPHA_BAR, true);
                f.set(ColorEditFlags::ALPHA_PREVIEW, true);
                f.set(ColorEditFlags::ALPHA_PREVIEW_HALF, true);
                f.set(ColorEditFlags::NO_LABEL, true);
                f.set(ColorEditFlags::DISPLAY_HEX, true);

                f.set(ColorEditFlags::PICKER_HUE_BAR, true);
                // f.set(ColorEditFlags::PickerHueWheel, true);
                f
            };

            let mut sel: [f32; 4] = {
                if let Some(col) = state.xpr().color_picker_color {
                    col.to_rgba(Some(state.xpr())).unwrap().into()
                } else {
                    unsafe { state.xpr().palette.current_color().1.as_rgba().into() }
                }
            };
            let b = ColorPicker::new(&im_str!("##4"), &mut sel)
                .flags(misc_flags)
                .alpha(true)
                .alpha_bar(true)
                .side_preview(true)
                .display_rgb(true);

            if b.build(&ui) {
                let ret: Color = sel.into();
                let pal = state.xpr_mut().palette.current_palette();
                let idx = pal.idx;
                let mut pal_ = pal.colors.borrow_mut();
                *(pal_.get_index_mut(idx).unwrap().1) = ret;
                drop(pal_);
                state.xpr_mut().set_redraw(true);
            };
        });
}

fn draw_cells(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let items: Vec<_> = state.xpr_mut().palette.palettes.keys().cloned().map(ImString::new).collect();
    let refs: Vec<_> = items.iter().collect();
    let pal_idx: i32 = state.xpr_mut().palette.selected_palette_idx as i32;
    if ComboBox::new(&im_str!("Palette")).build_simple_string(&ui, &mut (pal_idx as usize), &refs) {
        state.xpr_mut().palette.selected_palette_idx = pal_idx as usize;
        state.xpr_mut().set_redraw(true);
    }
    ui.text(&im_str!("Color: {}", state.xpr().palette.current_color().0));

    let temp = ui.cursor_screen_pos();
    let mut MARGIN = temp[0];
    let mut PALETTE_BEGIN_Y = temp[1];
    MARGIN += 1.5;
    PALETTE_BEGIN_Y += 1.5;
    let PALETTE_W = LEFT_SIDE_WIDTH - 2. * MARGIN;
    let BLOCK_SZ = PALETTE_W / state.cols_per_row as f32;
    // let PALETTE_H = 400.;

    // let draw_list = ui.get_window_draw_list();
    // draw_list.add_rect(
    //     (MARGIN - 5., PALETTE_BEGIN_Y - 5.),
    //     (RIGHT_SIDE_WIDTH - MARGIN + 5., PALETTE_BEGIN_Y + PALETTE_H + 5.),
    //     LIGHT_GREY
    // ).filled(false).build(&ui);
    let mut modified = false;
    let cols_per_row = state.cols_per_row as usize;
    let xpr = state.xpr_mut();
    let pal = xpr.palette.current_palette_mut();
    let color_idx = pal.idx;
    let mut pal_ = pal.colors.borrow_mut();
    for (i, (_col_name, col)) in pal_.iter_mut().enumerate() {
        //        let is_sel = col == &xpr.selected_color;
        let is_sel = i == color_idx;
        let x = MARGIN + BLOCK_SZ * ((i % cols_per_row) as f32);
        let y = PALETTE_BEGIN_Y + BLOCK_SZ * ((i / cols_per_row) as f32);

        ui.set_cursor_screen_pos([x, y]);
        let misc_flags = {
            let mut f = ColorEditFlags::empty();
            f.set(ColorEditFlags::HDR, true);
            f.set(ColorEditFlags::ALPHA_PREVIEW, true);
            f.set(ColorEditFlags::NO_OPTIONS, false);
            f.set(ColorEditFlags::NO_INPUTS, true);
            f.set(ColorEditFlags::NO_LABEL, true);
            f.set(ColorEditFlags::NO_PICKER, false);
            f
        };
        let mut sel: [f32; 4] = unsafe { (*col).as_rgba().into() };
        if ColorEdit::new(&im_str!("##{}", i), &mut sel).flags(misc_flags).alpha(false).build(&ui) {
            // if color is mutated
            *col = sel.into();
            modified = true;
        }

        if ui.is_item_hovered() && ui.io().mouse_down[0] {
            // if clicked
            pal.idx = i;
        }

        if is_sel {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_triangle(
                    [x - MARGIN / 8., y - MARGIN / 4. + BLOCK_SZ / 2.],
                    [x - MARGIN / 8., y + BLOCK_SZ - MARGIN / 4.],
                    [x + BLOCK_SZ - MARGIN / 4. - BLOCK_SZ / 2., y + BLOCK_SZ - MARGIN / 4.],
                    LIGHT_GREY,
                )
                .filled(true)
                .build();
        }
    }
    drop(pal_);
    if modified {
        xpr.set_redraw(true);
    }

    if ui.small_button(&im_str!("+")) {
        let pal = state.xpr_mut().palette.current_palette();
        let key = format!("my_color##{}", pal.colors.borrow().len());
        pal.colors.borrow_mut().insert(key, Color::black());
    }
}
