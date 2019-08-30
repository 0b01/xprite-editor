use glium::glutin::{self, Event, WindowEvent};
use glium::{Display, Surface};
use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use glium::backend::Facade;

use std::rc::Rc;

use crate::render::clipboard;
use crate::render::glium_renderer_support::Renderer;

pub type Textures = imgui::Textures<Rc<glium::Texture2d>>;

#[derive(Copy, Clone, PartialEq, Debug)]
struct MouseState {
    pos: glutin::dpi::LogicalPosition,
    pressed: (bool, bool, bool),
    wheel: f64,
}

impl MouseState {
    fn new() -> Self {
        Self {
            pos: glutin::dpi::LogicalPosition { x: 0., y: 0. },
            pressed: (false, false, false),
            wheel: 0.,
        }
    }
}

fn set_style(imgui: &mut imgui::Context) {
    let mut style = imgui.style_mut();
    style.window_rounding = 2.0;
    style.frame_rounding = 1.0;
    style.grab_rounding = 2.0;
    style.scrollbar_rounding = 0.0;
    style.item_spacing[1] = 4.5;
    style.frame_border_size = 1.;
    style.child_border_size = 1.;
    style.window_border_size = 1.;
    style.anti_aliased_lines = false;
    style.anti_aliased_fill = false;

    let vals = StyleColor::VARIANTS;
    macro_rules! find {
        ($item: ident) => {
            vals.iter().position(|&i| i == StyleColor::$item).unwrap()
        };
    }

    let GREY = [0.169, 0.157, 0.184, 1.000];
    // let CANVAS_BG  = [0.114, 0.106, 0.125, 1.000];
    let LIGHT_GREY = [0.259, 0.251, 0.255, 1.000];
    let TINT = [0.125, 0.118, 0.122, 1.000];

    let BORDER = [0.086, 0.075, 0.102, 1.000];
    let BORDER_SHD = [0.082, 0.071, 0.098, 1.000];

    // style.colors[find!(Text)]                  = [0.860, 0.930, 0.890, 0.78];
    // style.colors[find!(TextDisabled)]          = [0.860, 0.930, 0.890, 0.28];
    style.colors[find!(WindowBg)] = GREY;
    // style.colors[find!(PopupBg)]               = [0.200, 0.220, 0.270, 0.90];

    style.colors[find!(Border)] = BORDER;
    style.colors[find!(BorderShadow)] = BORDER_SHD;

    style.colors[find!(FrameBg)] = GREY;
    style.colors[find!(FrameBgHovered)] = LIGHT_GREY;
    // style.colors[find!(FrameBgActive)]         = GREY;

    style.colors[find!(TitleBg)] = LIGHT_GREY;
    style.colors[find!(TitleBgActive)] = LIGHT_GREY;
    style.colors[find!(TitleBgCollapsed)] = LIGHT_GREY;
    style.colors[find!(MenuBarBg)] = LIGHT_GREY;

    style.colors[find!(ScrollbarBg)] = TINT;
    style.colors[find!(ScrollbarGrab)] = LIGHT_GREY;
    style.colors[find!(ScrollbarGrabHovered)] = LIGHT_GREY;
    style.colors[find!(ScrollbarGrabActive)] = GREY;

    // style.colors[find!(CheckMark)]             = [0.71, 0.22, 0.27, 1.00];
    // style.colors[find!(SliderGrab)]            = [0.47, 0.77, 0.83, 0.14];
    // style.colors[find!(SliderGrabActive)]      = [0.71, 0.22, 0.27, 1.00];
    style.colors[find!(Button)] = TINT;
    style.colors[find!(ButtonHovered)] = LIGHT_GREY;
    // style.colors[find!(ButtonActive)]          = GREY;

    style.colors[find!(Header)] = LIGHT_GREY;
    style.colors[find!(HeaderHovered)] = [0.455, 0.198, 0.301, 0.86];
    style.colors[find!(HeaderActive)] = [0.455, 0.198, 0.301, 0.76];
    // style.colors[find!(ResizeGrip)]            = [0.47, 0.77, 0.83, 0.04];
    // style.colors[find!(ResizeGripHovered)]     = [0.455, 0.198, 0.301, 0.78];
    // style.colors[find!(ResizeGripActive)]      = [0.455, 0.198, 0.301, 1.00];
    // style.colors[find!(PlotLines)]             = [0.860, 0.930, 0.890, 0.63];
    // style.colors[find!(PlotLinesHovered)]      = [0.455, 0.198, 0.301, 1.00];
    // style.colors[find!(PlotHistogramHovered)]  = [0.455, 0.198, 0.301, 1.00];
    // style.colors[find!(ModalWindowDarkening)]  = [0.200, 0.220, 0.270, 0.73];
    // style.colors[find!(TextSelectedBg)]        = [0.455, 0.198, 0.301, 0.43];
}

pub struct System {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {
    let title = match title.rfind('/') {
        Some(idx) => title.split_at(idx + 1).1,
        None => title,
    };
    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(title.to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display =
        Display::new(builder, context, &events_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    set_style(&mut imgui);

    if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(Box::new(backend));
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        // FontSource::TtfData {
        //     data: include_bytes!("../../../resources/mplus-1p-regular.ttf"),
        //     size_pixels: font_size,
        //     config: Some(FontConfig {
        //         rasterizer_multiply: 1.75,
        //         glyph_ranges: FontGlyphRanges::japanese(),
        //         ..FontConfig::default()
        //     }),
        // },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System {
        events_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui, &Rc<glium::backend::Context>, &mut Textures)>(self, mut run_ui: F) {
        let System {
            mut events_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let mut mouse_state = MouseState::new();
        let mut last_frame = Instant::now();
        let mut run = true;


        while run {

            use glium::glutin::ElementState::Pressed;
            use glium::glutin::WindowEvent::*;
            use glium::glutin::{MouseButton, MouseScrollDelta, TouchPhase, WindowEvent::CloseRequested};
            use crate::ui::inputs::KeyCode;

            events_loop.poll_events(|event| {
                platform.handle_event(imgui.io_mut(), &window, &event);

                if let Event::WindowEvent { event, .. } = event {

                    match event {
                        CloseRequested => run = false,
                        KeyboardInput { input, .. } => {
                            use glium::glutin::VirtualKeyCode as Key;
                            let pressed = input.state == Pressed;

                            macro_rules! handle_key {
                                ($($key:ident),*) => {
                                    match input.virtual_keycode {
                                        $(
                                            Some(glium::glutin::VirtualKeyCode::$key) => imgui.io_mut().keys_down[KeyCode::$key as usize] = pressed,
                                        )*
                                        Some(Key::LControl) | Some(Key::RControl) => imgui.io_mut().key_ctrl = pressed,
                                        Some(Key::LShift) | Some(Key::RShift) => imgui.io_mut().key_shift = pressed,
                                        Some(Key::LAlt) | Some(Key::RAlt) => imgui.io_mut().key_alt = pressed,
                                        Some(Key::LWin) | Some(Key::RWin) => imgui.io_mut().key_super = pressed,
                                        _ => (),
                                    }
                                };
                            }

                            handle_key!(
                                Tab, Left, Right, Up, Down, PageUp, PageDown, Home, End, Delete, Back,
                                Return, Escape, Grave, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
                                Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Space
                            )
                        }
                        CursorMoved { position: pos, .. } => {
                            // Rescale position from glutin logical coordinates to our logical
                            // coordinates
                            mouse_state.pos = platform.scale_pos_from_winit(&window, pos);
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
                        } => mouse_state.wheel = y.into(),
                        MouseWheel {
                            delta: MouseScrollDelta::PixelDelta(pos),
                            phase: TouchPhase::Moved,
                            ..
                        } => {
                            // Rescale pixel delta from glutin logical coordinates to our logical
                            // coordinates
                            mouse_state.wheel = platform.scale_pos_from_winit(&window, pos).y;
                        }
                        ReceivedCharacter(c) => imgui.io_mut().add_input_character(c),
                        _ => (),
                    }
                }
            });

            let io = imgui.io_mut();
            platform
                .prepare_frame(io, &window)
                .expect("Failed to start frame");
            last_frame = io.update_delta_time(last_frame);
            let mut ui = imgui.frame();
            run_ui(&mut run, &mut ui, display.get_context(), renderer.textures());

            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(&ui, &window);
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
    }
}