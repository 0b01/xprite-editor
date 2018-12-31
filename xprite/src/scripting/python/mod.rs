mod xpr_module;
use self::xpr_module::*;

use crate::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::fs::File;
use std::io::Read;


pub fn python(fname: &str) -> Result<Xprite, String> {
    let mut f = File::open(fname).map_err(|_| "Cannot open file")?;
    let mut txt = String::new();
    f.read_to_string(&mut txt).expect("Unable to read to string");

    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    let globals = PyDict::new(py);

    let xpr = xpr_module::init_mod(py).unwrap();
    globals.set_item("xpr", xpr)
        .map_err(|_| "Failed to set xpr".to_owned())?;

    locals.set_item("PIXELS", PyList::empty(py))
        .map_err(|_| "Failed to set PIXELS".to_owned())?;
    locals.set_item("WIDTH", 100)
        .map_err(|_| "Failed to set WIDTH".to_owned())?;
    locals.set_item("HEIGHT", 100)
        .map_err(|_| "Failed to set HEIGHT".to_owned())?;

    py.run(&txt, Some(&globals), Some(&locals))
        .map_err(|e|
            {e.print(py); "script execution failed".to_owned()}
        )?;

    let width: f32 = locals.get_item("WIDTH")
        .ok_or("WIDTH is undefined".to_owned())?
        .extract()
        .map_err(|e| {e.print(py); "Cannot extract WIDTH".to_owned()})?;
    let height: f32 = locals.get_item("HEIGHT")
        .ok_or("HEIGHT is undefined".to_owned())?
        .extract()
        .map_err(|e| {e.print(py); "Cannot extract HEIGHT".to_owned()})?;
    let my_pixels: &MyPixels = locals.get_item("PIXELS")
        .ok_or("PIXELS is undefined".to_owned())?
        .extract()
        .map_err(|e| {e.print(py); "Cannot extract PIXELS".to_owned()})?;

    let buf = my_pixels.p.clone();

    let mut xpr = Xprite::new(width, height);

    xpr.history.enter().unwrap();
    let layer = xpr.current_layer_mut().unwrap();
    layer.content.clear();
    layer.content.extend(&buf);

    Ok(xpr)
}