#![allow(clippy::too_many_arguments)]
#![allow(clippy::float_cmp)]
#![feature(vec_remove_item)]
#![feature(specialization)]

extern crate fnv;
extern crate ase;
extern crate hex;
extern crate image as img;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

#[cfg(feature = "python-scripting")]
#[macro_use]
extern crate pyo3;

extern crate bincode as bc;
extern crate indexmap as imap;
extern crate libtexsyn;
extern crate natord;

pub mod bincode {
    pub use bc::*;
}
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
