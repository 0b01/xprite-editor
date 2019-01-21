mod xpr_module;

use crate::prelude::*;
use self::xpr_module::*;
use pyo3::types::{PyDict, PyList};
use std::fs::File;
use std::io::Read;

pub fn python(fname: &str) -> Result<Xprite, String> {
    let mut f = File::open(fname).map_err(|_| "Cannot open file")?;
    let mut txt = String::new();
    f.read_to_string(&mut txt)
        .expect("Unable to read to string");

    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    let mptr = unsafe { pyo3::ffi::PyImport_AddModule("__main__\0".as_ptr() as *const _) };
    let main_globals = unsafe { pyo3::ffi::PyModule_GetDict(mptr) };
    let obj: PyObject = unsafe { PyObject::from_borrowed_ptr(py, main_globals) };
    let globals: &PyDict = obj.cast_as::<PyDict>(py).unwrap();

    let xpr = xpr_module::init_mod(py).unwrap();
    globals
        .set_item("xpr", xpr)
        .map_err(|_| "Failed to set xpr".to_owned())?;

    locals
        .set_item("PIXELS", PyList::empty(py))
        .map_err(|_| "Failed to set PIXELS".to_owned())?;
    locals
        .set_item("WIDTH", 100)
        .map_err(|_| "Failed to set WIDTH".to_owned())?;
    locals
        .set_item("HEIGHT", 100)
        .map_err(|_| "Failed to set HEIGHT".to_owned())?;

    py.run(&txt, Some(&globals), Some(&locals)).map_err(|e| {
        e.print(py);
        "script execution failed".to_owned()
    })?;

    let width: f64 = locals
        .get_item("WIDTH")
        .ok_or_else(|| "WIDTH is undefined".to_owned())?
        .extract()
        .map_err(|e| {
            e.print(py);
            "Cannot extract WIDTH".to_owned()
        })?;
    let height: f64 = locals
        .get_item("HEIGHT")
        .ok_or_else(|| "HEIGHT is undefined".to_owned())?
        .extract()
        .map_err(|e| {
            e.print(py);
            "Cannot extract HEIGHT".to_owned()
        })?;
    let my_pixels: &MyPixels = locals
        .get_item("PIXELS")
        .ok_or_else(|| "PIXELS is undefined".to_owned())?
        .extract()
        .map_err(|e| {
            e.print(py);
            "Cannot extract PIXELS".to_owned()
        })?;

    let mut xpr = Xprite::new(width, height);

    xpr.history.enter().unwrap();
    let layer = xpr.current_layer_mut().unwrap();
    layer.content.clear();
    layer.content.extend(&my_pixels.p);

    Ok(xpr)
}
