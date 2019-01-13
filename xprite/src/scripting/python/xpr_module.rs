use crate::prelude::*;
use crate::algorithms;
use pyo3::prelude::*;
use pyo3::class::{
    basic::PyObjectProtocol,
    number::PyNumberProtocol,
};
use pyo3::types::PyTuple;

type PyPoint = (i32, i32);
type PyColor = (i32, i32, i32, i32);

pub fn init_mod(py: Python) -> PyResult<&PyModule> {
    let pymod = PyModule::new(py, "xpr")?;
    pymod.add::<PyColor>("RED", Color::red().into())?;
    pymod.add::<PyColor>("GREEN", Color::green().into())?;
    pymod.add::<PyColor>("BLUE", Color::blue().into())?;
    pymod.add_class::<MyPixel>()?;
    pymod.add_class::<MyPixels>()?;

    pymod.add_function(wrap_function!(add))?;
    pymod.add_function(wrap_function!(bezier))?;
    pymod.add_function(wrap_function!(rect))?;
    pymod.add_function(wrap_function!(line))?;

    Ok(pymod)
}

#[pyclass(name=Pixels)]
#[derive(Clone)]
pub struct MyPixels {
    pub p: Pixels,
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
    pub fn sub_(&mut self, other: &MyPixels) -> PyResult<&MyPixels> {
        self.p.sub(&other.p);
        Ok(self)
    }
    pub fn sub(&self, other: &MyPixels) -> PyResult<MyPixels> {
        let mut new_self = self.clone();
        new_self.sub_(other)?;
        Ok(new_self)
    }
    pub fn intersection(&mut self, other: &MyPixels) -> PyResult<MyPixels> {
        Ok(Self{p:self.p.intersection(&other.p)})
    }
    pub fn push(&mut self, px: &MyPixel) -> PyResult<()> {
        self.p.push(px.as_pixel());
        Ok(())
    }
    pub fn contains(&mut self, px: &MyPixel) -> PyResult<bool> {
        Ok(self.p.contains(&px.as_pixel()))
    }
    pub fn clear(&mut self) -> PyResult<()> {
        self.p.clear();
        Ok(())
    }

    pub fn set_color(&mut self, pycolor: &PyTuple) -> PyResult<()> {
        let color = pycolor.into();
        self.p.set_color(&color);
        Ok(())
    }

    pub fn with_color(&mut self, pycolor: &PyTuple) -> PyResult<&MyPixels> {
        self.set_color(pycolor)?;
        Ok(self)
    }

    pub fn pixel_perfect(&mut self) -> PyResult<&MyPixels> {
        self.p.pixel_perfect();
        Ok(self)
    }

    pub fn connected_components(&mut self, w: usize, h: usize) -> PyResult<Vec<MyPixels>> {
        let ccs =self.p.connected_components(w, h);
        let ret = ccs.into_iter().map(|p|MyPixels{p}).collect();
        Ok(ret)
    }

    pub fn perimeter(&self, w: usize, h: usize) -> PyResult<MyPixels> {
        let p = self.p.perimeter(w, h);
        Ok(MyPixels{p})
    }

    pub fn as_bool_mat(&self, w: usize, h: usize) -> PyResult<Vec<Vec<bool>>> {
        Ok(self.p.as_bool_mat(w, h))
    }

    pub fn shift(&self, dist: &PyTuple) -> PyResult<MyPixels> {
        let d: Vec2D = dist.into();
        let mut p = Pixels::new();
        for pixel in self.p.iter() {
            let mut pixel = pixel.clone();
            pixel.point.x += d.x;
            pixel.point.y += d.y;
            p.push(pixel)
        }
        Ok(MyPixels{p})
    }

    pub fn shift_(&mut self, dist: &PyTuple) -> PyResult<&MyPixels> {
        self.p = self.shift(dist)?.p;
        Ok(self)
    }

}

#[pyproto]
impl PyObjectProtocol for MyPixels {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.p))
    }
}

#[pyproto]
impl PyNumberProtocol for MyPixels {
    fn __matmul__(lhs: &'p mut MyPixels, rhs: &'p MyPixels) -> PyResult<&'p MyPixels> {
        lhs.extend(rhs)?;
        Ok(lhs)
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

impl From<&PyTuple> for Vec2D {
    fn from(p: &PyTuple) -> Vec2D {
        Vec2D {
            x: p.get_item(0).extract().unwrap(),
            y: p.get_item(1).extract().unwrap(),
        }
    }
}

impl From<&PyTuple> for Color {
    fn from(p: &PyTuple) -> Color {
        (
            p.get_item(0).extract().unwrap(),
            p.get_item(1).extract().unwrap(),
            p.get_item(2).extract().unwrap(),
            p.get_item(3).extract().unwrap(),
        ).into()
    }
}

impl From<&PyTuple> for Pixel {
    fn from(p: &PyTuple) -> Pixel {
        let point = p.into();
        let color = Color::red();
        Pixel {point, color}
    }
}



impl From<&MyPixel> for Vec2D {
    fn from(p: &MyPixel) -> Vec2D {
        Vec2D {
            x: p.point.0 as f32,
            y: p.point.1 as f32,
        }
    }
}

impl From<&MyPixel> for Color {
    fn from(p: &MyPixel) -> Color {
        p.color.into()
    }
}

impl From<&MyPixel> for Pixel {
    fn from(p: &MyPixel) -> Pixel {
        let point = p.into();
        let color = p.into();
        Pixel { point, color }
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

#[pyfunction(name=bezier)]
fn bezier(
    from: &PyTuple,
    ctrl1: &PyTuple,
    ctrl2: &PyTuple,
    to: &PyTuple,
) -> PyResult<MyPixels> {
    let seg = CubicBezierSegment {
        from: from.into(),
        ctrl1: ctrl1.into(),
        ctrl2: ctrl2.into(),
        to: to.into()
    };
    let p = seg.rasterize().unwrap();
    Ok(MyPixels{ p })
}

#[pyfunction(name=rect)]
fn rect(start: &PyTuple, stop: &PyTuple, filled: bool) -> PyResult<MyPixels> {
    let p = algorithms::rect::get_rect(
        Some(start.into()),
        Some(stop.into()),
        filled
    ).unwrap();
    Ok(MyPixels {p})
}

#[pyfunction(name=line)]
fn line(start: &PyTuple, stop: &PyTuple) -> PyResult<MyPixels> {
    let p0: Pixel = start.into();
    let p1: Pixel = stop.into();
    let p = algorithms::line::bresenham(&p0.point, &p1.point);
    Ok(MyPixels {p})
}
