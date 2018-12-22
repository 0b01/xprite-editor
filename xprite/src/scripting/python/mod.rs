use crate::prelude::*;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::fs::File;
use std::io::Read;

type PyPixel = ((i32, i32),(i32,i32,i32,i32));

pub fn python(fname: &str) -> Result<Xprite, String> {
    let mut f = File::open(fname).map_err(|_| "Cannot open file")?;
    let mut txt = String::new();
    f.read_to_string(&mut txt).expect("Unable to read to string");


    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    locals.set_item("PIXELS", PyDict::new(py))
        .map_err(|_| "Failed to set PIXELS".to_owned())?;
    py.run(&txt, None, Some(&locals))
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
    let pixels: Vec<PyPixel> = locals.get_item("PIXELS")
        .ok_or("PIXELS is undefined".to_owned())?
        .extract()
        .map_err(|e| {e.print(py); "Cannot extract PIXELS".to_owned()})?;

    let mut buf = Pixels::new();
    for &((x,y), (r,g,b,a)) in pixels.iter().rev() {
        buf.push(pixel!(x, y, Color{
            r:r as u8,
            g:g as u8,
            b:b as u8,
            a:a as u8
        }));
    }

    let mut xpr = Xprite::new(width, height);

    xpr.history.enter().unwrap();
    let layer = xpr.current_layer_mut().unwrap();
    layer.content.clear();
    layer.content.extend(&buf);

    Ok(xpr)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_python_run() {
        // use super::*;
        // let mut xpr = Xprite::new(100., 100.);
        // let fname = "scripts/render.py";
        // python(fname, &mut xpr).unwrap();
    }
}