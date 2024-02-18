use crate::element::{Element, InnerElement};

pub fn cube(x: u32, y: u32, z: u32, centered: bool) -> Element {
    Element(InnerElement::Cube { x, y, z, centered })
}
