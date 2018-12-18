#![feature(vec_remove_item)]

extern crate image as img;
#[macro_use] extern crate log;
#[macro_use] extern crate dyon;
extern crate indexmap;

#[macro_use] pub mod core;
pub mod algorithms;
pub mod scripting;


pub mod tools;
pub mod prelude;

pub mod layer;
pub mod rendering;


pub mod image {
    pub use img::*;
}
