#[macro_use]
extern crate stdweb;


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
                "+" => xprite_clone.borrow_mut().zoom_in(),
                "-" => xprite_clone.borrow_mut().zoom_out(),
                "z" => if event.ctrl_key() { xprite_clone.borrow_mut().undo() },
                "Z" => if event.ctrl_key() { xprite_clone.borrow_mut().redo() },
                "y" => if event.ctrl_key() { xprite_clone.borrow_mut().redo() },
                key => console!(log, key)
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

    let xprclone = xprite.clone();
    let fn_draw = move || {xprclone.borrow().draw()};
    let xprclone = xprite.clone();
    let fn_draw_pixel = move |x:u32, y:u32| {xprclone.borrow_mut().draw_pixel(x, y)};

    js! {
        window.xprite = {};
        window.xprite.draw = @{fn_draw};
        window.xprite.draw_pixel = @{fn_draw_pixel};
    };


    stdweb::event_loop();

}
