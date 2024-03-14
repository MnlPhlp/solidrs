use crate::{
    element::{Element, InnerElement},
    var::Arg,
    Val,
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
                x: Val::Calc(ix.clone() + x),
                y: Val::Calc(iy.clone() + y),
                z: Val::Calc(iz.clone() + z),
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
                x: Val::Calc(ix.clone() + x),
                y: Val::Calc(iy.clone() + y),
                z: Val::Calc(iz.clone() + z),
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
