use crate::{element::InnerElement, Element, Num};

pub fn square(x: impl Num, y: impl Num, centered: bool) -> Element {
    Element(InnerElement::Square {
        x: x.f32(),
        y: y.f32(),
        centered,
    })
}
