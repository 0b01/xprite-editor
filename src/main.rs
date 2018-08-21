#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate lyon_geom;


mod xprite;

use xprite::Xprite;

use stdweb::traits::*;
use stdweb::web::IEventTarget;
use stdweb::web::event::{
    KeyDownEvent,
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

    let xprite_clone = xprite.clone();
    doc.add_event_listener({
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "=" => xprite_clone.borrow_mut().zoom_in(),
                "-" => xprite_clone.borrow_mut().zoom_out(),
                "z" => if event.ctrl_key() { xprite_clone.borrow_mut().undo() },
                "Z" => if event.ctrl_key() { xprite_clone.borrow_mut().redo() },
                "y" => if event.ctrl_key() { xprite_clone.borrow_mut().redo() },
                _ => (),
            };
        }
    });

    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseUpEvent| {
        xprite_clone.borrow_mut().mouse_up(
            event.client_x(),
            event.client_y(),
        );
    });

    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseMoveEvent| {
        xprite_clone.borrow_mut().mouse_move(
            event.client_x(),
            event.client_y(),
        );
    });


    let xprite_clone = xprite.clone();
    doc.add_event_listener(move |event: MouseDownEvent| {
        xprite_clone.borrow_mut().mouse_down(
            event.client_x(),
            event.client_y(),
            event.button(),
        );
    });

    xprite.borrow().draw();

    init_js_bindings(&xprite);

    stdweb::event_loop();

}

fn init_js_bindings(xprite: &Rc<RefCell<Xprite>>) {
    let xpr = xprite.clone();
    let fn_draw = move || {xpr.borrow().draw()};
    let xpr = xprite.clone();
    let fn_draw_pixel = move |x:u32, y:u32| {xpr.borrow_mut().draw_pixel(x, y, None)};
    let xpr = xprite.clone();
    let fn_get_height = move || {xpr.borrow().get_height()};
    let xpr = xprite.clone();
    let fn_get_width = move || {xpr.borrow().get_width()};
    let xpr = xprite.clone();
    let fn_set_color = move |r:u8, g:u8, b:u8| {xpr.borrow_mut().set_color(r,g,b)};
    let xpr = xprite.clone();
    let fn_set_option = move |opt:String, val:String| {xpr.borrow_mut().set_option(&opt, &val)};
    let xpr = xprite.clone();
    let fn_set_option_for_tool = move |name: String, opt:String, val:String|
        {xpr.borrow_mut().set_option_for_tool(&name, &opt, &val)};
    let xpr = xprite.clone();
    let fn_change_tool = move |name: String|
        {xpr.borrow_mut().change_tool(&name)};

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
    };
}
