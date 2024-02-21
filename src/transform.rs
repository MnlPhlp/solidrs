use crate::{
    element::{Element, InnerElement},
    var::Arg,
};

impl<'a> Element<'a> {
    #[must_use]
    pub fn translate(&self, x: impl Arg<'a>, y: impl Arg<'a>, z: impl Arg<'a>) -> Self {
        let (x, y, z) = (x.val(), y.val(), z.val());
        let inner = match &self.0 {
            InnerElement::Translate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Translate {
                x: ix.clone() + x,
                y: iy.clone() + y,
                z: iz.clone() + z,
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
    pub fn rotate(&self, x: impl Arg<'a>, y: impl Arg<'a>, z: impl Arg<'a>) -> Self {
        let (x, y, z) = (x.val(), y.val(), z.val());
        let inner = match &self.0 {
            InnerElement::Rotate {
                x: ix,
                y: iy,
                z: iz,
                child,
            } => InnerElement::Rotate {
                x: ix.clone() + x,
                y: iy.clone() + y,
                z: iz.clone() + z,
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
