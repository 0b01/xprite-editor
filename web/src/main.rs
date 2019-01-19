#![recursion_limit = "128"]
#![feature(try_from)]

// #[macro_use]
#[macro_use]
extern crate stdweb;
extern crate fern;
extern crate xprite;
mod stdweb_renderer;

use self::stdweb_renderer::StdwebRenderer;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{KeyDownEvent, KeyUpEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{IEventTarget, IHtmlElement};
use xprite::prelude::*;

fn main() {
    init_logger();
    stdweb::initialize();

    let xpr = Rc::new(RefCell::new(Xprite::new(200., 200.)));
    let mut rdr = StdwebRenderer::new("#canvas");
    // xprite.borrow_mut().init(renderer);

    // --------------------------- register anim callback ----------------------
    let xpr_ = xpr.clone();
    let callback = move |_t: f64| {
        xpr_.borrow_mut().draw().unwrap();
        xpr_.borrow_mut().update().unwrap();
        xpr_.borrow_mut().render_canvas(&mut rdr);
        xpr_.borrow_mut().render_cursor(&mut rdr);
        xpr_.borrow_mut().render_bezier(&mut rdr);
    };
    js!(
        let callback = @{callback};
        function mainloop(t) {
            callback(t);
            requestAnimationFrame(mainloop);
        }
        requestAnimationFrame(mainloop);
    );

    let doc = stdweb::web::document();

    let xpr_ = xpr.clone();
    doc.add_event_listener({
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                // "=" => xpr.borrow_mut().zoom_in().unwrap(),
                // "-" => xpr.borrow_mut().zoom_out().unwrap(),
                // "p" => xpr.borrow().print_cursor_location(),
                // "l" => xpr.borrow_mut().change_tool("line"),
                "r" => xpr_.borrow_mut().change_tool(ToolType::Rect).unwrap(),
                "v" => xpr_.borrow_mut().change_tool(ToolType::Vector).unwrap(),
                "b" => xpr_.borrow_mut().change_tool(ToolType::Pencil).unwrap(),
                "g" => xpr_
                    .borrow_mut()
                    .change_tool(ToolType::PaintBucket)
                    .unwrap(),
                "z" => {
                    if event.ctrl_key() {
                        xpr_.borrow_mut().undo()
                    }
                }
                "Z" => {
                    if event.ctrl_key() {
                        xpr_.borrow_mut().redo()
                    }
                }
                "y" => {
                    if event.ctrl_key() {
                        xpr_.borrow_mut().redo()
                    }
                }
                "Control" => xpr_.borrow_mut().set_option("ctrl", "true").unwrap(),
                "Shift" => xpr_.borrow_mut().set_option("shift", "true").unwrap(),
                _ => (),
            };
        }
    });

    let xpr_ = xpr.clone();
    doc.add_event_listener({
        move |event: KeyUpEvent| {
            match event.key().as_ref() {
                "Control" => xpr_.borrow_mut().set_option("ctrl", "false").unwrap(),
                "Shift" => xpr_.borrow_mut().set_option("shift", "false").unwrap(),
                _ => (),
            };
        }
    });

    let xpr_ = xpr.clone();
    doc.add_event_listener(move |event: MouseUpEvent| {
        let canvas: CanvasElement = stdweb::web::document()
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let rect = canvas.get_bounding_client_rect();
        let x = event.client_x() as f32 - rect.get_x() as f32;
        let y = event.client_y() as f32 - rect.get_y() as f32;
        let w = rect.get_width() as f32;
        let h = rect.get_height() as f32;
        if oob(x, y, w, h) {
            return;
        }
        xpr_.borrow_mut()
            .mouse_up(&InputEvent::MouseUp {
                x,
                y,
                button: InputItem::Left,
            })
            .unwrap();
    });

    let xpr_ = xpr.clone();
    doc.add_event_listener(move |event: MouseMoveEvent| {
        let canvas: CanvasElement = stdweb::web::document()
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let rect = canvas.get_bounding_client_rect();
        xpr_.borrow_mut()
            .event(&InputEvent::MouseMove {
                x: event.client_x() as f32 - rect.get_x() as f32,
                y: event.client_y() as f32 - rect.get_y() as f32,
            })
            .unwrap();
    });

    let xpr_ = xpr.clone();
    doc.add_event_listener(move |event: MouseDownEvent| {
        let canvas: CanvasElement = stdweb::web::document()
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let rect = canvas.get_bounding_client_rect();
        let button = match event.button() {
            stdweb::web::event::MouseButton::Left => InputItem::Left,
            stdweb::web::event::MouseButton::Right => InputItem::Right,
            _ => unimplemented!(),
        };
        let x = event.client_x() as f32 - rect.get_x() as f32;
        let y = event.client_y() as f32 - rect.get_y() as f32;
        let w = rect.get_width() as f32;
        let h = rect.get_height() as f32;
        if oob(x, y, w, h) {
            return;
        }
        xpr_.borrow_mut()
            .mouse_down(&InputEvent::MouseDown { x, y, button })
            .unwrap();
    });

    init_js_bindings(&xpr);

    stdweb::event_loop();
}

fn init_js_bindings(xpr: &Rc<RefCell<Xprite>>) {
    let xpr_ = xpr.clone();
    let fn_draw = move || {
        xpr_.borrow_mut().draw().unwrap();
        ()
    };
    let xpr_ = xpr.clone();
    let fn_draw_pixel = move |x: u32, y: u32| {
        xpr_.borrow_mut()
            .current_layer_mut()
            .unwrap()
            .content
            .push(Pixel {
                point: Vec2f {
                    x: x as f32,
                    y: y as f32,
                },
                color: Color::red(),
            })
    };
    let xpr_ = xpr.clone();
    let fn_get_height = move || xpr_.borrow().canvas.art_w;
    let xpr_ = xpr.clone();
    let fn_get_width = move || xpr_.borrow().canvas.art_h;
    let xpr_ = xpr.clone();
    let fn_set_color = move |r: u8, g: u8, b: u8| {
        xpr_.borrow_mut().set_color(&Color::new(r, g, b));
        ()
    };
    let xpr_ = xpr.clone();
    let fn_set_option = move |opt: String, val: String| {
        xpr_.borrow_mut().set_option(&opt, &val).unwrap();
        ()
    };
    let xpr_ = xpr.clone();
    let fn_set_option_for_tool = move |name: String, opt: String, val: String| {
        let tool = ToolType::from_str(&name).unwrap();
        xpr_.borrow_mut()
            .set_option_for_tool(&tool, &opt, &val)
            .unwrap();
        ()
    };
    let xpr_ = xpr.clone();
    let fn_change_tool = move |name: String| {
        let tool = ToolType::from_str(&name).unwrap();
        xpr_.borrow_mut().change_tool(tool).unwrap();
        ()
    };
    let xpr_ = xpr.clone();
    let fn_enter = move || {
        xpr_.borrow_mut().history.enter().unwrap();
        ()
    };

    js! {
        window.xprite = {};
        window.xprite.draw = @{fn_draw};
        window.xprite.draw_pixel = @{fn_draw_pixel};
        window.xprite.get_height = @{fn_get_height};
        window.xprite.get_width = @{fn_get_width};
        window.xprite.set_color = @{fn_set_color};
        window.xprite.set_option = @{fn_set_option};
        window.xprite.set_option_for_tool = @{fn_set_option_for_tool};
        window.xprite.change_tool = @{fn_change_tool};
        window.xprite.enter = @{fn_enter}
    };
}

fn init_logger() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // .chain(std::io::stdout())
        .chain(fern::Output::call(move |record| {
            console!(log, format!("{}", record.args()));
        }))
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply()
        .unwrap();
}
