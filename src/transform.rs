use crate::element::{Element, InnerElement};

impl Element {
    pub fn translate(self, x: i32, y: i32, z: i32) -> Self {
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

    pub fn rotate(self, x: i32, y: i32, z: i32) -> Self {
        let inner = match self.0 {
            InnerElement::Rotate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Rotate {
                x: ix + x,
                y: iy + y,
                z: iz + z,
                child,
            },
            child => InnerElement::Rotate {
                x,
                y,
                z,
                child: Box::new(child),
            },
        };
        Element(inner)
    }
}
