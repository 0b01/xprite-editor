use crate::prelude::*;
use std::borrow::Cow;
use xprite::rendering::Renderer;

pub fn draw_palette(rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;

    let colors = vec![(ImGuiCol::WindowBg, BLACK)];
    ui.with_color_vars(&colors, || {
        ui.window(im_str!("Palette"))
            .no_bring_to_front_on_focus(true)
            .position((0., TOOLBOX_H), ImGuiCond::Always)
            .size(
                (LEFT_SIDE_WIDTH, sz.1 as f32 - TOOLBOX_H - COLOR_PICKER_H),
                ImGuiCond::Always,
            )
            .movable(false)
            .collapsible(false)
            .resizable(false)
            .build(|| {
                // ui.slider_int(im_str!("Colors per row"), &mut state.cols_per_row, 2, 16).build();
                let styles = vec![
                    StyleVar::ChildBorderSize(0.),
                    StyleVar::FrameBorderSize(0.),
                    StyleVar::WindowBorderSize(0.),
                    StyleVar::PopupBorderSize(0.),
                    StyleVar::FrameRounding(0.),
                ];
                ui.with_style_vars(&styles, || {
                    draw_cells(rdr, state, ui);
                })
            });
    })
}

pub fn draw_color_picker(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;

    ui.window(im_str!("Color Picker"))
        .no_bring_to_front_on_focus(true)
        .position((0., sz.1 as f32 - COLOR_PICKER_H), ImGuiCond::Always)
        .size((LEFT_SIDE_WIDTH, COLOR_PICKER_H), ImGuiCond::Always)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(|| {
            let misc_flags = {
                let mut f = ImGuiColorEditFlags::empty();
                f.set(ImGuiColorEditFlags::AlphaBar, true);
                f.set(ImGuiColorEditFlags::AlphaPreview, true);
                f.set(ImGuiColorEditFlags::AlphaPreviewHalf, true);
                f.set(ImGuiColorEditFlags::NoLabel, true);
                f.set(ImGuiColorEditFlags::HEX, true);

                f.set(ImGuiColorEditFlags::PickerHueBar, true);
                // f.set(ImGuiColorEditFlags::PickerHueWheel, true);
                f
            };

            let mut sel: [f32; 4] = {
                if let Some(col) = &state.xpr_mut().color_picker_color {
                    (*col).into()
                } else {
                    state.xpr_mut().selected_color.into()
                }
            };
            let b = ui
                .color_picker(im_str!("MyColor##4"), &mut sel)
                .flags(misc_flags)
                .alpha(true)
                .alpha_bar(true)
                .side_preview(true)
                .rgb(true);

            if b.build() {
                let ret = sel.into();
                state.xpr_mut().selected_color = ret;
            };
        });
}

fn draw_cells(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let items: Vec<_> = state.xpr_mut()
        .palette_man
        .palettes
        .keys()
        .cloned()
        .map(ImString::new)
        .collect();
    let refs: Vec<_> = items.iter().map(|s| s.as_ref()).collect();
    ui.combo(
        im_str!("Palette"),
        &mut state.palette_window.palette_idx,
        &refs[..],
        -1,
    );
    ui.text(im_str!(
        "Color: {}",
        state
            .palette_window
            .palette_color_name
            .as_ref()
            .unwrap_or(&Cow::Borrowed("None"))
    ));

    let (mut MARGIN, mut PALETTE_BEGIN_Y) = ui.get_cursor_screen_pos();
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
    // ).filled(false).build();
    let idx = state.palette_window.palette_idx as usize;
    let cols_per_row = state.cols_per_row as usize;
    let mut xpr = state.xpr_mut();
    let pal = xpr
        .palette_man
        .palettes
        .get_index_mut(idx)
        .unwrap()
        .1;
    for (i, (col_name, col)) in pal.iter_mut().enumerate() {
        let is_sel = col == &xpr.selected_color;
        let x = MARGIN + BLOCK_SZ * ((i % cols_per_row) as f32);
        let y = PALETTE_BEGIN_Y
            + BLOCK_SZ * ((i / cols_per_row) as f32);

        ui.set_cursor_screen_pos((x, y));
        if ui
            .invisible_button(im_str!("colorcell##{}", i), (BLOCK_SZ, BLOCK_SZ))
        {
            xpr.selected_color = *col;
        }

        // if the color block is selected
        if is_sel {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect(
                    (x - MARGIN / 4., y - MARGIN / 4.),
                    (x + BLOCK_SZ - MARGIN / 4., y + BLOCK_SZ - MARGIN / 4.),
                    LIGHT_GREY,
                )
                .filled(true)
                .build();
        }

        ui.set_cursor_screen_pos((x, y));
        let misc_flags = {
            let mut f = ImGuiColorEditFlags::empty();
            f.set(ImGuiColorEditFlags::HDR, true);
            f.set(ImGuiColorEditFlags::AlphaPreview, true);
            f.set(ImGuiColorEditFlags::NoOptions, false);
            f.set(ImGuiColorEditFlags::NoInputs, true);
            f.set(ImGuiColorEditFlags::NoLabel, true);
            f.set(ImGuiColorEditFlags::NoPicker, true);
            f
        };
        let mut sel: [f32; 4] = (*col).into();
        let id = im_str!("MyColor##{}", i);
        let b = ui.color_edit(id, &mut sel).flags(misc_flags).alpha(false);

        // // show color name on hover
        // if ui.is_item_hovered() {
        //     state.palette_window.palette_color_name =
        //         Some(Cow::Owned(col_name.to_owned()));
        //     ui.tooltip(|| {
        //         ui.text(col_name.to_owned());
        //     });
        // }

        if b.build() {
            *col = sel.into();
        }
    }

    if ui.small_button(im_str!("+")) {
        let pal = state.xpr_mut()
            .palette_man
            .palettes
            .get_index_mut(idx)
            .unwrap()
            .1;
        pal.insert(format!("my_color##{}", pal.len()), Color::black());
    }
}
