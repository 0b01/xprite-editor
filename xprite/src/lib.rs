#![allow(clippy::too_many_arguments)]
#![allow(clippy::float_cmp)]
#![feature(vec_remove_item)]
#![feature(specialization)]

extern crate image as img;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

#[cfg(feature = "dyon-scripting")]
#[macro_use] extern crate dyon;
#[cfg(feature = "python-scripting")]
#[macro_use] extern crate pyo3;

extern crate indexmap as imap;
extern crate libtexsyn;

extern crate bincode as bc;

pub mod bincode {
    pub use bc::*;
}

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

pub mod indexmap {
    pub use imap::*;
}

