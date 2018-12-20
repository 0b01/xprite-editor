#![allow(clippy::too_many_arguments)]
#![allow(clippy::float_cmp)]
#![feature(vec_remove_item)]


extern crate image as img;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate dyon;
extern crate indexmap;
extern crate libtexsyn;
extern crate bincode;

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
