mod boolean;
mod element;
mod modify;
mod scad;
mod shapes2d;
mod shapes3d;
mod transform;

pub use element::Element;
pub use scad::ExportScad;
pub use shapes2d::*;
pub use shapes3d::*;

pub trait Num {
    fn f32(&self) -> f32;
}

impl Num for f32 {
    fn f32(&self) -> f32 {
        *self
    }
}
impl Num for i32 {
    fn f32(&self) -> f32 {
        *self as f32
    }
}
