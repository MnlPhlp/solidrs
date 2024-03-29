mod boolean;
mod calc;
mod config;
mod element;
mod modify;
mod scad;
mod shapes2d;
mod shapes3d;
#[cfg(test)]
mod tests;
mod transform;
mod var;

pub use element::Element;
pub use scad::Export;
pub use shapes2d::*;
pub use shapes3d::*;
pub use var::{Arg, Val, Var};

// include examples for testing
#[cfg(test)]
#[doc(hidden)]
#[path = "../examples"]
mod examples {
    pub mod car;
    pub mod simple;
    pub mod variables;
}
