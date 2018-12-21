use crate::prelude::*;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type PyPixel = ((i32, i32),(u8,u8,u8,u8));

pub fn python(fname: &str, xpr: &mut Xprite) -> PyResult<()> {
    let mut f = File::open(fname).unwrap();
    let mut txt = String::new();
    f.read_to_string(&mut txt).unwrap();


    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    locals.set_item("os", py.import("os")?).unwrap();
    locals.set_item("pixels", PyDict::new(py)).unwrap();
    py.run(&txt, None, Some(&locals)).unwrap();

    let res = locals.get_item("pixels").unwrap();
    let v: Vec<PyPixel> = res.extract().unwrap();

    let mut buf = Pixels::new();
    for &((x,y), (r,g,b,a)) in v.iter().rev() {
        buf.push(pixel!(x, y, Color{r,g,b,a}));
    }
    xpr.history.enter().unwrap();
    let layer = xpr.current_layer_mut().unwrap();
    layer.content.clear();
    layer.content.extend(&buf);

    Ok(())
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