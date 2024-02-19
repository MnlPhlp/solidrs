use crate::{
    element::{Element, InnerElement},
    var::Arg,
};

impl Element {
    pub fn translate(self, x: impl Arg, y: impl Arg, z: impl Arg) -> Self {
        let (x, y, z) = (x.var(), y.var(), z.var());
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

    pub fn rotate(self, x: impl Arg, y: impl Arg, z: impl Arg) -> Self {
        let (x, y, z) = (x.var(), y.var(), z.var());
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
