use crate::{
    element::{Element, InnerElement},
    var::Arg,
};

#[must_use]
pub fn cube(x: impl Arg, y: impl Arg, z: impl Arg) -> Element {
    Element(InnerElement::Cube {
        x: x.val(),
        y: y.val(),
        z: z.val(),
        centered: false,
    })
}

#[must_use]
pub fn cylinder(h: impl Arg, r: impl Arg) -> Element {
    Element(InnerElement::Cylinder {
        h: h.val(),
        r: r.val(),
        centered: false,
    })
}

impl Element {
    #[must_use]
    pub fn rotate_extrude(&self, angle: impl Arg) -> Self {
        Element(InnerElement::RotateExtrude {
            angle: angle.val(),
            child: Box::new(self.0.clone()),
        })
    }
}
