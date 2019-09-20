use crate::prelude::*;
// use xprite::prelude::*;
// use xprite::rendering::Renderer;

pub fn draw_toolbar(state: &mut State, ui: &Ui) {
    ui.window(&im_str!("Sprite 3 v{}", env!("CARGO_PKG_VERSION")))
        .no_bring_to_front_on_focus(true)
        .position([0., 20.], Condition::Appearing)
        .size([LEFT_SIDE_WIDTH, 200.], Condition::Appearing)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(|| {
            let selected = state.xpr_mut().toolbox.selected.clone();
            let tools = ToolType::VARIANTS;
            let orig = ui.get_cursor_screen_pos();
            for (idx, tool_type) in tools.iter().enumerate() {
                let is_sel = selected == *tool_type;

                let x = (idx % 5) as f32;
                let y = (idx / 5) as f32;
                let mut pos = [orig[0] + x * 33., orig[1] + y * 35.];

                // macro_rules! draw_down {
                //     ($t: expr) => {
                //         ui.set_cursor_screen_pos(pos);
                //         ui.image(TextureId::from(state.icons["button_down"]), [30., 30.]).build();
                //         ui.set_cursor_screen_pos(pos);
                //         ui.image(TextureId::from(state.icons[$t.as_str()]), [30., 30.]).build();
                //     }
                // }

                macro_rules! draw_up {
                    ($t: expr) => {
                        ui.set_cursor_screen_pos(pos);
                        ui.image(TextureId::from(state.icons["button_up"]), [30., 30.]).build();
                        ui.set_cursor_screen_pos(pos);
                        ui.image(TextureId::from(state.icons[$t.as_str()]), [30., 30.]).build();
                    }
                }
                macro_rules! draw_hold {
                    ($t: expr) => {
                        ui.set_cursor_screen_pos(pos);
                        ui.image(TextureId::from(state.icons["button_hold"]), [30., 30.]).build();
                        pos[1] += 5.;
                        ui.set_cursor_screen_pos(pos);
                        ui.image(TextureId::from(state.icons[$t.as_str()]), [30., 30.]).build();
                    }
                }

                let within_bound = {
                    let mouse_pos = ui.io().mouse_pos;
                    (
                        mouse_pos[0] > pos[0]
                     && mouse_pos[1] > pos[1]
                     && mouse_pos[0] < pos[0] + 30.
                     && mouse_pos[1] < pos[1] + 30.
                    )
                };

                if is_sel {
                    draw_hold!(tool_type);
                } else {
                    if ui.is_mouse_down(MouseButton::Left) && within_bound {
                        draw_hold!(tool_type);
                        state.xpr_mut().change_tool(*tool_type).unwrap();
                    } else {
                        draw_up!(tool_type);
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text(
                            &im_str!("{} ({})",
                            tool_type.as_str(),
                            state.hotkeys.lookup_reverse_str(&Bind::PushTool(*tool_type)).unwrap(),
                        ));
                    }
                }
            }

        })
}
