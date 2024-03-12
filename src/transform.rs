use crate::{
    element::{Element, InnerElement},
    var::Arg,
};

impl Element {
    #[must_use]
    pub fn translate(&self, x: impl Arg, y: impl Arg, z: impl Arg) -> Self {
        let (x, y, z) = (x.val(), y.val(), z.val());
        let inner = match &self.0 {
            InnerElement::Translate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Translate {
                x: *ix + x,
                y: *iy + y,
                z: *iz + z,
                child: child.clone(),
            },
            child => InnerElement::Translate {
                x,
                y,
                z,
                child: Box::new(child.clone()),
            },
        };
        Element(inner)
    }

    #[must_use]
    pub fn rotate(&self, x: impl Arg, y: impl Arg, z: impl Arg) -> Self {
        let (x, y, z) = (x.val(), y.val(), z.val());
        let inner = match &self.0 {
            InnerElement::Rotate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Rotate {
                x: *ix + x,
                y: *iy + y,
                z: *iz + z,
                child: child.clone(),
            },
            child => InnerElement::Rotate {
                x,
                y,
                z,
                child: Box::new(child.clone()),
            },
        };
        Element(inner)
    }
}
