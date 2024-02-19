use crate::{
    element::{Element, InnerElement},
    var::Arg,
};

pub fn cube(x: impl Arg, y: impl Arg, z: impl Arg) -> Element {
    Element(InnerElement::Cube {
        x: x.var(),
        y: y.var(),
        z: z.var(),
        centered: false,
    })
}

pub fn cylinder(h: impl Arg, r: impl Arg) -> Element {
    Element(InnerElement::Cylinder {
        h: h.var(),
        r: r.var(),
        centered: false,
    })
}

impl Element {
    pub fn rotate_extrude(self, angle: impl Arg) -> Self {
        Element(InnerElement::RotateExtrude {
            angle: angle.var(),
            child: Box::new(self.0),
        })
    }
}
