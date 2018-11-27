use imgui::{ImGui, FontGlyphRange, FrameSize, ImFontConfig, ImGuiMouseCursor, Ui, ImGuiCol, ImVec4};
use std::time::Instant;

use crate::ui::inputs::KeyCode;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

fn set_style(imgui: &mut ImGui) {
    let mut style = imgui.style_mut();
    style.window_rounding = 0.0;
    style.frame_rounding  = 0.0;
    style.grab_rounding   = 0.0;
    style.scrollbar_rounding = 5.0;
    style.frame_border_size = 1.0;
    style.item_spacing.y = 6.5;
    style.frame_border_size = 2.;
    style.child_border_size = 2.;
    style.window_border_size = 2.;

    let vals = ImGuiCol::values();
    macro_rules! find {
        ($item: ident) => {
            vals.iter().position(|&i|i==ImGuiCol::$item).unwrap()
        };
    }

    style.colors[find!(Text)]                  = ImVec4::new(0.860, 0.930, 0.890, 0.78);
    style.colors[find!(TextDisabled)]          = ImVec4::new(0.860, 0.930, 0.890, 0.28);
    style.colors[find!(WindowBg)]              = ImVec4::new(0.000, 0.000, 0.000, 1.00);
    style.colors[find!(PopupBg)]               = ImVec4::new(0.200, 0.220, 0.270, 0.90);
    style.colors[find!(Border)]                = ImVec4::new(1.000, 0.200, 0.000, 1.00);
    style.colors[find!(BorderShadow)]          = ImVec4::new(0.000, 0.000, 0.000, 0.00);
    style.colors[find!(FrameBg)]               = ImVec4::new(0.200, 0.220, 0.270, 1.00);
    style.colors[find!(FrameBgHovered)]        = ImVec4::new(0.455, 0.198, 0.301, 0.78);
    style.colors[find!(FrameBgActive)]         = ImVec4::new(0.455, 0.198, 0.301, 1.00);
    style.colors[find!(TitleBg)]               = ImVec4::new(0.232, 0.201, 0.271, 1.00);
    style.colors[find!(TitleBgActive)]         = ImVec4::new(1.000, 0.200, 0.000, 1.00);
    style.colors[find!(TitleBgCollapsed)]      = ImVec4::new(0.200, 0.220, 0.270, 0.75);
    style.colors[find!(MenuBarBg)]             = ImVec4::new(0.200, 0.220, 0.270, 0.47);
    style.colors[find!(ScrollbarBg)]           = ImVec4::new(0.200, 0.220, 0.270, 1.00);
    style.colors[find!(ScrollbarGrab)]         = ImVec4::new(0.09, 0.15, 0.16, 1.00);
    style.colors[find!(ScrollbarGrabHovered)]  = ImVec4::new(0.455, 0.198, 0.301, 0.78);
    style.colors[find!(ScrollbarGrabActive)]   = ImVec4::new(0.455, 0.198, 0.301, 1.00);
    style.colors[find!(CheckMark)]             = ImVec4::new(0.71, 0.22, 0.27, 1.00);
    style.colors[find!(SliderGrab)]            = ImVec4::new(0.47, 0.77, 0.83, 0.14);
    style.colors[find!(SliderGrabActive)]      = ImVec4::new(0.71, 0.22, 0.27, 1.00);
    style.colors[find!(Button)]                = ImVec4::new(0.47, 0.77, 0.83, 0.14);
    style.colors[find!(ButtonHovered)]         = ImVec4::new(0.455, 0.198, 0.301, 0.86);
    style.colors[find!(ButtonActive)]          = ImVec4::new(0.455, 0.198, 0.301, 1.00);
    style.colors[find!(Header)]                = ImVec4::new(0.455, 0.198, 0.301, 0.76);
    style.colors[find!(HeaderHovered)]         = ImVec4::new(0.455, 0.198, 0.301, 0.86);
    style.colors[find!(HeaderActive)]          = ImVec4::new(1.000, 0.200, 0.000, 1.00);
    style.colors[find!(ResizeGrip)]            = ImVec4::new(0.47, 0.77, 0.83, 0.04);
    style.colors[find!(ResizeGripHovered)]     = ImVec4::new(0.455, 0.198, 0.301, 0.78);
    style.colors[find!(ResizeGripActive)]      = ImVec4::new(0.455, 0.198, 0.301, 1.00);
    style.colors[find!(PlotLines)]             = ImVec4::new(0.860, 0.930, 0.890, 0.63);
    style.colors[find!(PlotLinesHovered)]      = ImVec4::new(0.455, 0.198, 0.301, 1.00);
    style.colors[find!(PlotHistogram)]         = ImVec4::new(0.860, 0.930, 0.890, 0.63);
    style.colors[find!(PlotHistogramHovered)]  = ImVec4::new(0.455, 0.198, 0.301, 1.00);
    style.colors[find!(TextSelectedBg)]        = ImVec4::new(0.455, 0.198, 0.301, 0.43);
    style.colors[find!(ModalWindowDarkening)]  = ImVec4::new(0.200, 0.220, 0.270, 0.73);
}

pub fn run<F: FnMut(&Ui) -> bool>(title: &str, clear_color: [f32; 4], mut run_ui: F) {
    use glium::glutin;
    use glium::{Display, Surface};
    use imgui_glium_renderer::Renderer;

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(title.to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display = Display::new(builder, context, &events_loop).unwrap();
    let window = display.gl_window();

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    set_style(&mut imgui);

    // In the examples we only use integer DPI factors, because the UI can get very blurry
    // otherwise. This might or might not be what you want in a real application.
    let hidpi_factor = window.get_hidpi_factor().round();

    let font_size = (13.0 * hidpi_factor) as f32;

    imgui.fonts().add_default_font_with_config(
        ImFontConfig::new()
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size),
    );

    imgui.fonts().add_font_with_config(
        include_bytes!("../Roboto-Regular.ttf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            .rasterizer_multiply(1.75),
        &FontGlyphRange::japanese(),
    );

    imgui.set_font_global_scale((1.0 / hidpi_factor) as f32);

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    configure_keys(&mut imgui);

    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();
    let mut quit = false;

    loop {
        events_loop.poll_events(|event| {
            use glium::glutin::ElementState::Pressed;
            use glium::glutin::WindowEvent::*;
            use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => quit = true,
                    KeyboardInput { input, .. } => {
                        use glium::glutin::VirtualKeyCode as Key;

                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(Key::Tab) =>       imgui.set_key(KeyCode::Tab, pressed),
                            Some(Key::Left) =>      imgui.set_key(KeyCode::Left, pressed),
                            Some(Key::Right) =>     imgui.set_key(KeyCode::Right, pressed),
                            Some(Key::Up) =>        imgui.set_key(KeyCode::Up, pressed),
                            Some(Key::Down) =>      imgui.set_key(KeyCode::Down, pressed),
                            Some(Key::PageUp) =>    imgui.set_key(KeyCode::PageUp, pressed),
                            Some(Key::PageDown) =>  imgui.set_key(KeyCode::PageDown, pressed),
                            Some(Key::Home) =>      imgui.set_key(KeyCode::Home, pressed),
                            Some(Key::End) =>       imgui.set_key(KeyCode::End, pressed),
                            Some(Key::Delete) =>    imgui.set_key(KeyCode::Delete, pressed),
                            Some(Key::Back) =>      imgui.set_key(KeyCode::Back, pressed),
                            Some(Key::Return) =>    imgui.set_key(KeyCode::Return, pressed),
                            Some(Key::Escape) =>    imgui.set_key(KeyCode::Escape, pressed),

                            Some(Key::A) => imgui.set_key(KeyCode::A, pressed),
                            Some(Key::B) => imgui.set_key(KeyCode::B, pressed),
                            Some(Key::C) => imgui.set_key(KeyCode::C, pressed),
                            Some(Key::D) => imgui.set_key(KeyCode::D, pressed),
                            Some(Key::E) => imgui.set_key(KeyCode::E, pressed),
                            Some(Key::F) => imgui.set_key(KeyCode::F, pressed),
                            Some(Key::G) => imgui.set_key(KeyCode::G, pressed),
                            Some(Key::H) => imgui.set_key(KeyCode::H, pressed),
                            Some(Key::I) => imgui.set_key(KeyCode::I, pressed),
                            Some(Key::J) => imgui.set_key(KeyCode::J, pressed),
                            Some(Key::K) => imgui.set_key(KeyCode::K, pressed),
                            Some(Key::L) => imgui.set_key(KeyCode::L, pressed),
                            Some(Key::M) => imgui.set_key(KeyCode::M, pressed),
                            Some(Key::N) => imgui.set_key(KeyCode::N, pressed),
                            Some(Key::O) => imgui.set_key(KeyCode::O, pressed),
                            Some(Key::P) => imgui.set_key(KeyCode::P, pressed),
                            Some(Key::Q) => imgui.set_key(KeyCode::Q, pressed),
                            Some(Key::R) => imgui.set_key(KeyCode::R, pressed),
                            Some(Key::S) => imgui.set_key(KeyCode::S, pressed),
                            Some(Key::T) => imgui.set_key(KeyCode::T, pressed),
                            Some(Key::U) => imgui.set_key(KeyCode::U, pressed),
                            Some(Key::V) => imgui.set_key(KeyCode::V, pressed),
                            Some(Key::W) => imgui.set_key(KeyCode::W, pressed),
                            Some(Key::X) => imgui.set_key(KeyCode::X, pressed),
                            Some(Key::Y) => imgui.set_key(KeyCode::Y, pressed),
                            Some(Key::Z) => imgui.set_key(KeyCode::Z, pressed),
                            Some(Key::Key0) => imgui.set_key(KeyCode::Key0, pressed),
                            Some(Key::Key1) => imgui.set_key(KeyCode::Key1, pressed),
                            Some(Key::Key2) => imgui.set_key(KeyCode::Key2, pressed),
                            Some(Key::Key3) => imgui.set_key(KeyCode::Key3, pressed),
                            Some(Key::Key4) => imgui.set_key(KeyCode::Key4, pressed),
                            Some(Key::Key5) => imgui.set_key(KeyCode::Key5, pressed),
                            Some(Key::Key6) => imgui.set_key(KeyCode::Key6, pressed),
                            Some(Key::Key7) => imgui.set_key(KeyCode::Key7, pressed),
                            Some(Key::Key8) => imgui.set_key(KeyCode::Key8, pressed),
                            Some(Key::Key9) => imgui.set_key(KeyCode::Key9, pressed),

                            Some(Key::Space) => imgui.set_key(KeyCode::Space, pressed),

                            Some(Key::LControl) | Some(Key::RControl) => {
                                imgui.set_key_ctrl(pressed)
                            }
                            Some(Key::LShift) | Some(Key::RShift) => imgui.set_key_shift(pressed),
                            Some(Key::LAlt) | Some(Key::RAlt) => imgui.set_key_alt(pressed),
                            Some(Key::LWin) | Some(Key::RWin) => imgui.set_key_super(pressed),
                            _ => {}
                        }
                    }
                    CursorMoved { position: pos, .. } => {
                        // Rescale position from glutin logical coordinates to our logical
                        // coordinates
                        mouse_state.pos = pos
                            .to_physical(window.get_hidpi_factor())
                            .to_logical(hidpi_factor)
                            .into();
                    }
                    MouseInput { state, button, .. } => match button {
                        MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                        MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                        MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                        _ => {}
                    },
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(pos),
                        phase: TouchPhase::Moved,
                        ..
                    } => {
                        // Rescale pixel delta from glutin logical coordinates to our logical
                        // coordinates
                        mouse_state.wheel = pos
                            .to_physical(window.get_hidpi_factor())
                            .to_logical(hidpi_factor)
                            .y as f32;
                    }
                    ReceivedCharacter(c) => imgui.add_input_character(c),
                    _ => (),
                }
            }
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);

        let mouse_cursor = imgui.mouse_cursor();
        if imgui.mouse_draw_cursor() || mouse_cursor == ImGuiMouseCursor::None {
            // Hide OS cursor
            window.hide_cursor(true);
        } else {
            // Set OS cursor
            window.hide_cursor(false);
            window.set_cursor(match mouse_cursor {
                ImGuiMouseCursor::None => unreachable!("mouse_cursor was None!"),
                ImGuiMouseCursor::Arrow => glutin::MouseCursor::Arrow,
                ImGuiMouseCursor::TextInput => glutin::MouseCursor::Text,
                ImGuiMouseCursor::ResizeNS => glutin::MouseCursor::NsResize,
                ImGuiMouseCursor::ResizeEW => glutin::MouseCursor::EwResize,
                ImGuiMouseCursor::ResizeNESW => glutin::MouseCursor::NeswResize,
                ImGuiMouseCursor::ResizeNWSE => glutin::MouseCursor::NwseResize,
                ImGuiMouseCursor::Move => glutin::MouseCursor::Move,
            });
        }

        // Rescale window size from glutin logical size to our logical size
        let physical_size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());
        let logical_size = physical_size.to_logical(hidpi_factor);

        let frame_size = FrameSize {
            logical_size: logical_size.into(),
            hidpi_factor,
        };

        ::std::thread::sleep(::std::time::Duration::from_millis(15));

        let ui = imgui.frame(frame_size, delta_s);
        if !run_ui(&ui) {
            break;
        }

        let mut target = display.draw();
        target.clear_color(
            clear_color[0],
            clear_color[1],
            clear_color[2],
            clear_color[3],
        );
        renderer.render(&mut target, ui).expect("Rendering failed");
        target.finish().unwrap();

        if quit {
            break;
        }
    }
}

fn configure_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey;

    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    imgui.set_mouse_pos(mouse_state.pos.0 as f32, mouse_state.pos.1 as f32);
    imgui.set_mouse_down([
        mouse_state.pressed.0,
        mouse_state.pressed.1,
        mouse_state.pressed.2,
        false,
        false,
    ]);
    imgui.set_mouse_wheel(mouse_state.wheel);
    mouse_state.wheel = 0.0;
}
