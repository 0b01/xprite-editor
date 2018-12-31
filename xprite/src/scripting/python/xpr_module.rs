use crate::prelude::*;
use pyo3::prelude::*;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::types::PyTuple;

type PyPoint = (i32, i32);
type PyColor = (i32, i32, i32, i32);

pub fn init_mod(py: Python) -> PyResult<&PyModule> {
    let pymod = PyModule::new(py, "xpr")?;
    pymod.add("RED", Color::red().as_tuple())?;
    pymod.add("GREEN", Color::green().as_tuple())?;
    pymod.add("BLUE", Color::blue().as_tuple())?;
    pymod.add_class::<MyPixel>()?;
    pymod.add_class::<MyPixels>()?;
    pymod.add_function(wrap_function!(add))?;

    Ok(pymod)
}
#[pyclass(name=Pixels)]
pub struct MyPixels {
    p: Pixels,
}
#[pymethods]
impl MyPixels {
    #[new]
    #[args(args="*")]
    fn __new__(obj: &PyRawObject, args: &PyTuple) -> PyResult<()> {
        let mut p = Pixels::new();
        for i in args.iter() {
            let i: &MyPixel = i.try_into()?;
            p.push(i.as_pixel());
        }
        obj.init(|_| {
            MyPixels {
                p
            }
        })
    }
    pub fn extend(&mut self, other: &MyPixels) -> PyResult<()> {
        self.p.extend(&other.p);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for MyPixels {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("{:?}", self.p))
    }
}


#[pyclass(name=Pixel)]
pub struct MyPixel {
   point: PyPoint,
   color: PyColor,
}

#[pymethods]
impl MyPixel {
    #[new]
    fn __new__(obj: &PyRawObject, point: PyPoint, color: PyColor) -> PyResult<()> {
        obj.init(|_| {
            MyPixel {
                point, color
            }
        })
    }
}

impl MyPixel {
    pub fn as_pixel(&self) -> Pixel {
        let (x, y) = self.point;
        let (r, g, b, a) = self.color;
        pixel!(x, y, Color {
            r:r as u8,
            g:g as u8,
            b:b as u8,
            a:a as u8
        })
    }
}

#[pyproto]
impl PyObjectProtocol for MyPixel {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("Pixel<({:?}), ({:?})>", self.point, self.color))
    }
}

#[pyfunction(name=add)]
fn add(a: i64, b: i64) -> i64 {
    a + b
}