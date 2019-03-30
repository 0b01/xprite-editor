#![allow(clippy::too_many_arguments)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::useless_let_if_seq)]
#![allow(clippy::single_match)]
#![allow(clippy::explicit_counter_loop)]
#![allow(clippy::float_cmp)]

#[macro_use]
extern crate bitflags;
pub extern crate ase;
extern crate fnv;
extern crate hex;
extern crate palette;
extern crate image as img;
extern crate imageproc;
#[macro_use]
extern crate log;

#[cfg(feature = "python-scripting")]
#[macro_use]
extern crate pyo3;

extern crate indexmap as imap;
extern crate natord;
extern crate wfc_image;

#[macro_use]
pub mod core;
pub mod algorithms;
pub mod layer;
pub mod prelude;
pub mod rendering;
pub mod scripting;
pub mod tools;
pub mod image {
    pub use img::*;
}
pub mod indexmap {
    pub use imap::*;
}
