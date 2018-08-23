#![recursion_limit="128"]

// #[macro_use]
// extern crate itertools;
#[macro_use]
extern crate stdweb;


mod xprite;

use xprite::{Xprite, Event};

use stdweb::traits::*;
use stdweb::web::IEventTarget;
use stdweb::web::event::{
    KeyDownEvent,
    KeyUpEvent,
    MouseDownEvent,
    MouseMoveEvent,
    MouseUpEvent
};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();

    let xprite = Rc::new(RefCell::new(Xprite::new("#canvas", 200, 200)));

    xprite.borrow().draw();

    let doc = stdweb::web::document();

    let xpr = xprite.clone();
    doc.add_event_listener({
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "=" => xpr.borrow_mut().zoom_in(),
                "-" => xpr.borrow_mut().zoom_out(),
                "p" => xpr.borrow().print_cursor_location(),
                "l" => xpr.borrow_mut().change_tool("line"),
                "f" => xpr.borrow_mut().change_tool("pencil"),
                "z" => if event.ctrl_key() { xpr.borrow_mut().undo() },
                "Z" => if event.ctrl_key() { xpr.borrow_mut().redo() },
                "y" => if event.ctrl_key() { xpr.borrow_mut().redo() },
                "Control" => xpr.borrow_mut().set_option("ctrl", "true"),
                "Shift" => xpr.borrow_mut().set_option("shift", "true"),
                _ => (),
            };
        }
    });

    let xpr = xprite.clone();
    doc.add_event_listener({
        move |event: KeyUpEvent| {
            match event.key().as_ref() {
                "Control" => xpr.borrow_mut().set_option("ctrl", "false"),
                "Shift" => xpr.borrow_mut().set_option("shift", "false"),
                _ => (),
            };
        }
    });

    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseUpEvent| {
        xprite_clone.borrow_mut().mouse_up(
            &Event::MouseUp{
                x: event.client_x(),
                y: event.client_y(),
            }
        );
    });

    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseMoveEvent| {
        xprite_clone.borrow_mut().mouse_move(
            &Event::MouseMove{
                x: event.client_x(),
                y: event.client_y(),
            }
        );
    });


    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseDownEvent| {
        xprite_clone.borrow_mut().mouse_down(
            &Event::MouseDown{
                x: event.client_x(),
                y: event.client_y(),
                button: event.button(),
            }
        );
    });

    xprite.borrow().draw();

    init_js_bindings(&xprite);

    stdweb::event_loop();

}

fn init_js_bindings(xprite: &Rc<RefCell<Xprite>>) {
    let xpr = xprite.clone();
    let fn_draw = move ||
        {xpr.borrow().draw()};
    let xpr = xprite.clone();
    let fn_draw_pixel = move |x:u32, y:u32|
        {xpr.borrow_mut().draw_pixel(x, y, None)};
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
    let fn_new_history_frame = move ||
        {xpr.borrow_mut().history.new_history_frame();};

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
        window.xprite.new_history_frame = @{fn_new_history_frame}
    };
}
