use crate::{
    element::{Element, InnerElement},
    Num,
};

pub fn cube(x: impl Num, y: impl Num, z: impl Num) -> Element {
    Element(InnerElement::Cube {
        x: x.f32(),
        y: y.f32(),
        z: z.f32(),
        centered: false,
    })
}

pub fn cylinder(h: i32, r: i32) -> Element {
    Element(InnerElement::Cylinder {
        h: h.f32(),
        r: r.f32(),
        centered: false,
    })
}

impl Element {
    pub fn rotate_extrude(self, angle: i32) -> Self {
        Element(InnerElement::RotateExtrude {
            angle: angle.f32(),
            child: Box::new(self.0),
        })
    }
}
