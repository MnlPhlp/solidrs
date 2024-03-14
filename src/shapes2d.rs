use crate::{element::InnerElement, var::Arg, Element};

pub fn square(x: impl Arg, y: impl Arg) -> Element {
    Element(InnerElement::Square {
        x: x.val(),
        y: y.val(),
        centered: false,
    })
}
