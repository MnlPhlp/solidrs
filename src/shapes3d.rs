use crate::{
    element::{Element, InnerElement},
    var::Arg,
};

pub fn cube<'a>(x: impl Arg<'a>, y: impl Arg<'a>, z: impl Arg<'a>) -> Element<'a> {
    Element(InnerElement::Cube {
        x: x.val(),
        y: y.val(),
        z: z.val(),
        centered: false,
    })
}

pub fn cylinder<'a>(h: impl Arg<'a>, r: impl Arg<'a>) -> Element<'a> {
    Element(InnerElement::Cylinder {
        h: h.val(),
        r: r.val(),
        centered: false,
    })
}

impl<'a> Element<'a> {
    pub fn rotate_extrude(self, angle: impl Arg<'a>) -> Self {
        Element(InnerElement::RotateExtrude {
            angle: angle.val(),
            child: Box::new(self.0),
        })
    }
}
