use crate::element::{Element, InnerElement};

pub fn cube(x: i32, y: i32, z: i32, centered: bool) -> Element {
    Element(InnerElement::Cube { x, y, z, centered })
}

impl Element {
    pub fn rotate_extrude(self, angle: i32) -> Self {
        Element(InnerElement::RotateExtrude {
            angle,
            child: Box::new(self.0),
        })
    }
}
