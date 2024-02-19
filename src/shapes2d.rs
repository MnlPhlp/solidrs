use crate::{element::InnerElement, var::Arg, Element};

pub fn square<'a>(x: impl Arg<'a>, y: impl Arg<'a>) -> Element<'a> {
    Element(InnerElement::Square {
        x: x.val(),
        y: y.val(),
        centered: false,
    })
}
