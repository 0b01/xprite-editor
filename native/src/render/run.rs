// use glium::{
//     backend::{Context, Facade},
//     Texture2d,
// };
// use imgui::*;
// use imgui_winit_support;
// use std::rc::Rc;
// use std::time::Instant;

// use glium::glutin;
// use glium::{Display, Surface};
// use imgui_glium_renderer::Renderer;

// use crate::ui::inputs::KeyCode;
// #[derive(Copy, Clone, PartialEq, Debug, Default)]
// struct MouseState {
//     pos: (i32, i32),
//     pressed: (bool, bool, bool),
//     wheel: f64,
// }

// fn set_style(imgui: &imgui::Context) {
//     let mut style = imgui.style_mut();
//     style.window_rounding = 2.0;
//     style.frame_rounding = 1.0;
//     style.grab_rounding = 2.0;
//     style.scrollbar_rounding = 0.0;
//     style.item_spacing[1] = 4.5;
//     style.frame_border_size = 1.;
//     style.child_border_size = 1.;
//     style.window_border_size = 1.;
//     style.anti_aliased_lines = false;
//     style.anti_aliased_fill = false;

//     let vals = StyleColor::VARIANTS;
//     macro_rules! find {
//         ($item: ident) => {
//             vals.iter().position(|&i| i == StyleColor::$item).unwrap()
//         };
//     }

//     let GREY = [0.169, 0.157, 0.184, 1.000];
//     // let CANVAS_BG  = [0.114, 0.106, 0.125, 1.000];
//     let LIGHT_GREY = [0.259, 0.251, 0.255, 1.000];
//     let TINT = [0.125, 0.118, 0.122, 1.000];

//     let BORDER = [0.086, 0.075, 0.102, 1.000];
//     let BORDER_SHD = [0.082, 0.071, 0.098, 1.000];

//     // style.colors[find!(Text)]                  = [0.860, 0.930, 0.890, 0.78];
//     // style.colors[find!(TextDisabled)]          = [0.860, 0.930, 0.890, 0.28];
//     style.colors[find!(WindowBg)] = GREY;
//     // style.colors[find!(PopupBg)]               = [0.200, 0.220, 0.270, 0.90];

//     style.colors[find!(Border)] = BORDER;
//     style.colors[find!(BorderShadow)] = BORDER_SHD;

//     style.colors[find!(FrameBg)] = GREY;
//     style.colors[find!(FrameBgHovered)] = LIGHT_GREY;
//     // style.colors[find!(FrameBgActive)]         = GREY;

//     style.colors[find!(TitleBg)] = LIGHT_GREY;
//     style.colors[find!(TitleBgActive)] = LIGHT_GREY;
//     style.colors[find!(TitleBgCollapsed)] = LIGHT_GREY;
//     style.colors[find!(MenuBarBg)] = LIGHT_GREY;

//     style.colors[find!(ScrollbarBg)] = TINT;
//     style.colors[find!(ScrollbarGrab)] = LIGHT_GREY;
//     style.colors[find!(ScrollbarGrabHovered)] = LIGHT_GREY;
//     style.colors[find!(ScrollbarGrabActive)] = GREY;

//     // style.colors[find!(CheckMark)]             = [0.71, 0.22, 0.27, 1.00];
//     // style.colors[find!(SliderGrab)]            = [0.47, 0.77, 0.83, 0.14];
//     // style.colors[find!(SliderGrabActive)]      = [0.71, 0.22, 0.27, 1.00];
//     style.colors[find!(Button)] = TINT;
//     style.colors[find!(ButtonHovered)] = LIGHT_GREY;
//     // style.colors[find!(ButtonActive)]          = GREY;

//     style.colors[find!(Header)] = LIGHT_GREY;
//     style.colors[find!(HeaderHovered)] = [0.455, 0.198, 0.301, 0.86];
//     style.colors[find!(HeaderActive)] = [0.455, 0.198, 0.301, 0.76];
//     // style.colors[find!(ResizeGrip)]            = [0.47, 0.77, 0.83, 0.04];
//     // style.colors[find!(ResizeGripHovered)]     = [0.455, 0.198, 0.301, 0.78];
//     // style.colors[find!(ResizeGripActive)]      = [0.455, 0.198, 0.301, 1.00];
//     // style.colors[find!(PlotLines)]             = [0.860, 0.930, 0.890, 0.63];
//     // style.colors[find!(PlotLinesHovered)]      = [0.455, 0.198, 0.301, 1.00];
//     // style.colors[find!(PlotHistogramHovered)]  = [0.455, 0.198, 0.301, 1.00];
//     // style.colors[find!(ModalWindowDarkening)]  = [0.200, 0.220, 0.270, 0.73];
//     // style.colors[find!(TextSelectedBg)]        = [0.455, 0.198, 0.301, 0.43];
// }

// pub type Textures = imgui::Textures<Texture2d>;

// pub fn run<F>(title: String, clear_color: [f32; 4], mut run_ui: F)
// where
//     F: FnMut(&Ui, &Rc<Context>, &mut Textures) -> bool,
// {
//     let mut events_loop = glutin::EventsLoop::new();
//     let context = glutin::ContextBuilder::new().with_vsync(true);
//     let builder = glutin::WindowBuilder::new()
//         .with_title(title)
//         .with_dimensions(glutin::dpi::LogicalSize::new(1024., 768.));
//     let display = Display::new(builder, context, &events_loop).unwrap();
//     let window = display.gl_window();

//     let mut imgui = imgui::Context::create();
//     imgui.set_ini_filename(None);
//     set_style(imgui);

//     // In the examples we only use integer DPI factors, because the UI can get very blurry
//     // otherwise. This might or might not be what you want in a real application.
//     let hidpi_factor = window.get_hidpi_factor().round();

//     let font_size = (13.0 * hidpi_factor) as f32;

//     imgui
//         .fonts()
//         .add_default_font_with_config(FontConfig::new().oversample_h(1).pixel_snap_h(true).size_pixels(font_size));

//     imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

//     let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

//     imgui_winit_support::configure_keys(&mut imgui);

//     let mut mouse_state = MouseState::default();
//     let mut last_frame = Instant::now();
//     let mut quit = false;

//     loop {
//         events_loop.poll_events(|event| {
//             use glium::glutin::ElementState::Pressed;
//             use glium::glutin::WindowEvent::*;
//             use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent::CloseRequested};

//             imgui_winit_support::handle_event(&mut imgui, &event, window.get_hidpi_factor(), hidpi_factor);

//             if let Event::WindowEvent { event, .. } = event {
//                 match event {
//                     CloseRequested => quit = true,
//                     KeyboardInput { input, .. } => {
//                         use glium::glutin::VirtualKeyCode as Key;
//                         let pressed = input.state == Pressed;
//                         match input.virtual_keycode {
//                             Some(Key::Tab) => { imgui.io().keys_down[KeyCode::Tab as usize] = pressed }
//                             Some(Key::Left) => { imgui.io().keys_down[KeyCode::Left as usize] = pressed }
//                             Some(Key::Right) => { imgui.io().keys_down[KeyCode::Right as usize] = pressed }
//                             Some(Key::Up) => { imgui.io().keys_down[KeyCode::Up as usize] = pressed }
//                             Some(Key::Down) => { imgui.io().keys_down[KeyCode::Down as usize] = pressed }
//                             Some(Key::PageUp) => { imgui.io().keys_down[KeyCode::PageUp as usize] = pressed }
//                             Some(Key::PageDown) => { imgui.io().keys_down[KeyCode::PageDown as usize] = pressed }
//                             Some(Key::Home) => { imgui.io().keys_down[KeyCode::Home as usize] = pressed }
//                             Some(Key::End) => { imgui.io().keys_down[KeyCode::End as usize] = pressed }
//                             Some(Key::Delete) => { imgui.io().keys_down[KeyCode::Delete as usize] = pressed }
//                             Some(Key::Back) => { imgui.io().keys_down[KeyCode::Back as usize] = pressed }
//                             Some(Key::Return) => { imgui.io().keys_down[KeyCode::Return as usize] = pressed }
//                             Some(Key::Escape) => { imgui.io().keys_down[KeyCode::Escape as usize] = pressed }
//                             Some(Key::Grave) => { imgui.io().keys_down[KeyCode::Grave as usize] = pressed }
//                             Some(Key::A) => { imgui.io().keys_down[KeyCode::A as usize] = pressed }
//                             Some(Key::B) => { imgui.io().keys_down[KeyCode::B as usize] = pressed }
//                             Some(Key::C) => { imgui.io().keys_down[KeyCode::C as usize] = pressed }
//                             Some(Key::D) => { imgui.io().keys_down[KeyCode::D as usize] = pressed }
//                             Some(Key::E) => { imgui.io().keys_down[KeyCode::E as usize] = pressed }
//                             Some(Key::F) => { imgui.io().keys_down[KeyCode::F as usize] = pressed }
//                             Some(Key::G) => { imgui.io().keys_down[KeyCode::G as usize] = pressed }
//                             Some(Key::H) => { imgui.io().keys_down[KeyCode::H as usize] = pressed }
//                             Some(Key::I) => { imgui.io().keys_down[KeyCode::I as usize] = pressed }
//                             Some(Key::J) => { imgui.io().keys_down[KeyCode::J as usize] = pressed }
//                             Some(Key::K) => { imgui.io().keys_down[KeyCode::K as usize] = pressed }
//                             Some(Key::L) => { imgui.io().keys_down[KeyCode::L as usize] = pressed }
//                             Some(Key::M) => { imgui.io().keys_down[KeyCode::M as usize] = pressed }
//                             Some(Key::N) => { imgui.io().keys_down[KeyCode::N as usize] = pressed }
//                             Some(Key::O) => { imgui.io().keys_down[KeyCode::O as usize] = pressed }
//                             Some(Key::P) => { imgui.io().keys_down[KeyCode::P as usize] = pressed }
//                             Some(Key::Q) => { imgui.io().keys_down[KeyCode::Q as usize] = pressed }
//                             Some(Key::R) => { imgui.io().keys_down[KeyCode::R as usize] = pressed }
//                             Some(Key::S) => { imgui.io().keys_down[KeyCode::S as usize] = pressed }
//                             Some(Key::T) => { imgui.io().keys_down[KeyCode::T as usize] = pressed }
//                             Some(Key::U) => { imgui.io().keys_down[KeyCode::U as usize] = pressed }
//                             Some(Key::V) => { imgui.io().keys_down[KeyCode::V as usize] = pressed }
//                             Some(Key::W) => { imgui.io().keys_down[KeyCode::W as usize] = pressed }
//                             Some(Key::X) => { imgui.io().keys_down[KeyCode::X as usize] = pressed }
//                             Some(Key::Y) => { imgui.io().keys_down[KeyCode::Y as usize] = pressed }
//                             Some(Key::Z) => { imgui.io().keys_down[KeyCode::Z as usize] = pressed }
//                             Some(Key::Key0) => { imgui.io().keys_down[KeyCode::Key0 as usize] = pressed }
//                             Some(Key::Key1) => { imgui.io().keys_down[KeyCode::Key1 as usize] = pressed }
//                             Some(Key::Key2) => { imgui.io().keys_down[KeyCode::Key2 as usize] = pressed }
//                             Some(Key::Key3) => { imgui.io().keys_down[KeyCode::Key3 as usize] = pressed }
//                             Some(Key::Key4) => { imgui.io().keys_down[KeyCode::Key4 as usize] = pressed }
//                             Some(Key::Key5) => { imgui.io().keys_down[KeyCode::Key5 as usize] = pressed }
//                             Some(Key::Key6) => { imgui.io().keys_down[KeyCode::Key6 as usize] = pressed }
//                             Some(Key::Key7) => { imgui.io().keys_down[KeyCode::Key7 as usize] = pressed }
//                             Some(Key::Key8) => { imgui.io().keys_down[KeyCode::Key8 as usize] = pressed }
//                             Some(Key::Key9) => { imgui.io().keys_down[KeyCode::Key9 as usize] = pressed }
//                             Some(Key::Space) => { imgui.io().keys_down[KeyCode::Space as usize] = pressed }
//                             Some(Key::LControl) | Some(Key::RControl) => imgui.set_key_ctrl(pressed),
//                             Some(Key::LShift) | Some(Key::RShift) => imgui.set_key_shift(pressed),
//                             Some(Key::LAlt) | Some(Key::RAlt) => imgui.set_key_alt(pressed),
//                             Some(Key::LWin) | Some(Key::RWin) => imgui.set_key_super(pressed),
//                             _ => {}
//                         }
//                     }
//                     CursorMoved { position: pos, .. } => {
//                         // Rescale position from glutin logical coordinates to our logical
//                         // coordinates
//                         mouse_state.pos = pos.to_physical(window.get_hidpi_factor()).to_logical(hidpi_factor).into();
//                     }
//                     MouseInput { state, button, .. } => match button {
//                         MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
//                         MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
//                         MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
//                         _ => {}
//                     },
//                     MouseWheel {
//                         delta: MouseScrollDelta::LineDelta(_, y),
//                         phase: TouchPhase::Moved,
//                         ..
//                     } => mouse_state.wheel = y.into(),
//                     MouseWheel {
//                         delta: MouseScrollDelta::PixelDelta(pos),
//                         phase: TouchPhase::Moved,
//                         ..
//                     } => {
//                         // Rescale pixel delta from glutin logical coordinates to our logical
//                         // coordinates
//                         mouse_state.wheel = pos.to_physical(window.get_hidpi_factor()).to_logical(hidpi_factor).y.into();
//                     }
//                     // ReceivedCharacter(c) => imgui.add_input_character(c),
//                     _ => (),
//                 }
//             }
//         });

//         let now = Instant::now();
//         let delta = now - last_frame;
//         let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
//         last_frame = now;

//         imgui_winit_support::update_mouse_cursor(&imgui, &window);

//         let frame_size = imgui_winit_support::get_frame_size(&window, hidpi_factor).unwrap();

//         let ui = imgui.frame(frame_size, delta_s);
//         if !run_ui(&ui, display.get_context(), renderer.textures()) {
//             break;
//         }

//         let mut target = display.draw();
//         target.clear_color(clear_color[0], clear_color[1], clear_color[2], clear_color[3]);
//         renderer.render(&mut target, ui).expect("Rendering failed");
//         // render_canvas(&mut target, &display);
//         target.finish().unwrap();

//         if quit {
//             break;
//         }
//     }
// }
