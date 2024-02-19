use crate::{
    element::{Element, InnerElement},
    Num,
};

impl Element {
    pub fn translate(self, x: impl Num, y: impl Num, z: impl Num) -> Self {
        let (x, y, z) = (x.f32(), y.f32(), z.f32());
        let inner = match self.0 {
            InnerElement::Translate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Translate {
                x: ix + x,
                y: iy + y,
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

    pub fn rotate(self, x: impl Num, y: impl Num, z: impl Num) -> Self {
        let (x, y, z) = (x.f32(), y.f32(), z.f32());
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
