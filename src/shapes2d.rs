use crate::{element::InnerElement, Element};

pub fn square(x: i32, y: i32, centered: bool) -> Element {
    Element(InnerElement::Square { x, y, centered })
}
