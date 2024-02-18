use crate::element::{Element, InnerElement};

impl Element {
    pub fn translate(self, x: u32, y: u32, z: u32) -> Self {
        let inner = match self.0 {
            InnerElement::Translate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Translate {
                x: ix + x,
                y: iy + x,
                z: iz + z,
                child,
            },
            child => InnerElement::Translate {
                x,
                y,
                z,
                child: Box::new(child),
            },
        };
        Element(inner)
    }
}
