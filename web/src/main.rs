#![recursion_limit="128"]
#![feature(try_from)]

// #[macro_use]
// extern crate itertools;
#[macro_use] extern crate stdweb;
extern crate xprite;
mod stdweb_renderer;

use self::stdweb_renderer::StdwebRenderer;
use xprite::prelude::*;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{IEventTarget, IHtmlElement};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::{
    KeyDownEvent,
    KeyUpEvent,
    MouseDownEvent,
    MouseMoveEvent,
    MouseUpEvent,
};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();

    let xprite = Rc::new(RefCell::new(Xprite::new(200., 200.)));
    let rdr = StdwebRenderer::new("#canvas");
    // xprite.borrow_mut().init(renderer);

    xprite.borrow_mut().draw();
    xprite.borrow_mut().render(&rdr);

    let doc = stdweb::web::document();

    let xpr = xprite.clone();
    doc.add_event_listener({
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                // "=" => xpr.borrow_mut().zoom_in().unwrap(),
                // "-" => xpr.borrow_mut().zoom_out().unwrap(),
                // "p" => xpr.borrow().print_cursor_location(),
                // "l" => xpr.borrow_mut().change_tool("line"),
                // "f" => xpr.borrow_mut().change_tool("pencil"),
                "z" => if event.ctrl_key() { xpr.borrow_mut().undo() },
                "Z" => if event.ctrl_key() { xpr.borrow_mut().redo() },
                "y" => if event.ctrl_key() { xpr.borrow_mut().redo() },
                "Control" => xpr.borrow_mut().set_option("ctrl", "true").unwrap(),
                "Shift" => xpr.borrow_mut().set_option("shift", "true").unwrap(),
                _ => (),
            };
        }
    });

    let xpr = xprite.clone();
    doc.add_event_listener({
        move |event: KeyUpEvent| {
            match event.key().as_ref() {
                "Control" => xpr.borrow_mut().set_option("ctrl", "false").unwrap(),
                "Shift" => xpr.borrow_mut().set_option("shift", "false").unwrap(),
                _ => (),
            };
        }
    });

    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseUpEvent| {
        let canvas: CanvasElement = stdweb::web::document().query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
        let rect = canvas.get_bounding_client_rect();
        xprite_clone.borrow_mut().mouse_up(
            &InputEvent::MouseUp{
                x: event.client_x() as f32 - rect.get_x() as f32,
                y: event.client_y() as f32 - rect.get_y() as f32,
            }
        );
    });

    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseMoveEvent| {
        let canvas: CanvasElement = stdweb::web::document().query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
        let rect = canvas.get_bounding_client_rect();
        xprite_clone.borrow_mut().mouse_move(
            &InputEvent::MouseMove{
                x: event.client_x() as f32 - rect.get_x() as f32,
                y: event.client_y() as f32 - rect.get_y() as f32,
            }
        );
    });


    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseDownEvent| {
        let canvas: CanvasElement = stdweb::web::document().query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
        let rect = canvas.get_bounding_client_rect();
        let button = match event.button() {
            stdweb::web::event::MouseButton::Left => InputItem::Left,
            stdweb::web::event::MouseButton::Right => InputItem::Right,
            _ => unimplemented!(),
        };
        xprite_clone.borrow_mut().mouse_down(
            &InputEvent::MouseDown {
                x: event.client_x() as f32 - rect.get_x() as f32,
                y: event.client_y() as f32 - rect.get_y() as f32,
                button,
            }
        );
    });

    // init_js_bindings(&xprite);

    stdweb::event_loop();

}

/*
fn init_js_bindings(xprite: &Rc<RefCell<Xprite>>) {
    let xpr = xprite.clone();
    let fn_draw = move ||
        {xpr.borrow().draw()};
    let xpr = xprite.clone();
    let fn_draw_pixel = move |x:u32, y:u32|
        {xpr.borrow_mut().draw_pixel(x, y, ColorOption::Unset)};
    let xpr = xprite.clone();
    let fn_get_height = move ||
        {xpr.borrow().get_height()};
    let xpr = xprite.clone();
    let fn_get_width = move ||
        {xpr.borrow().get_width()};
    let xpr = xprite.clone();
    let fn_set_color = move |r:u8, g:u8, b:u8|
        {xpr.borrow_mut().set_color(r,g,b)};
    let xpr = xprite.clone();
    let fn_set_option = move |opt:String, val:String|
        {xpr.borrow_mut().set_option(&opt, &val)};
    let xpr = xprite.clone();
    let fn_set_option_for_tool = move |name: String, opt:String, val:String|
        {xpr.borrow_mut().set_option_for_tool(&name, &opt, &val)};
    let xpr = xprite.clone();
    let fn_change_tool = move |name: String|
        {xpr.borrow_mut().change_tool(&name)};
    let xpr = xprite.clone();
    let fn_enter = move ||
        {xpr.borrow_mut().history.enter();};

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
*/