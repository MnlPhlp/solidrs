use crate::{
    element::InnerElement,
    var::{Arg, Val},
    Element,
};

impl<'a> Element<'a> {
    #[must_use]
    /// # Panics
    //  panics for types that don't allow centering
    pub fn center(&self) -> Self {
        let inner = match &self.0 {
            InnerElement::Cube { x, y, z, .. } => InnerElement::Cube {
                x: *x,
                y: *y,
                z: *z,
                centered: true,
            },
            InnerElement::Cylinder { h, r, .. } => InnerElement::Cylinder {
                h: *h,
                r: *r,
                centered: true,
            },
            // ToDo fix this with types to not allow invalid calls
            other => panic!("center() is not allowed for type {other}"),
        };
        Element(inner)
    }

    #[must_use]
    pub fn margin(&self, margin: &impl Arg<'a>) -> Self {
        Element(self.0.margin(margin.f32()))
    }
}

impl InnerElement<'_> {
    #[must_use]
    fn margin(&self, margin: f32) -> Self {
        match self {
            InnerElement::Cube { x, y, z, centered } => margin_cube(*x, *y, *z, *centered, margin),
            Self::Translate { x, y, z, child } => Self::Translate {
                x: *x,
                y: *y,
                z: *z,
                child: Box::new(child.margin(margin)),
            },

            // ToDo fix this with types to not allow invalid calls
            other => panic!("margin() is not allowed for type {other}"),
        }
    }
}

fn margin_cube(x: Val, y: Val, z: Val, centered: bool, margin: f32) -> InnerElement<'static> {
    let cube = InnerElement::Cube {
        x: x + margin * 2.,
        y: y + margin * 2.,
        z: z + margin * 2.,
        centered,
    };
    if centered {
        cube
    } else {
        InnerElement::Translate {
            x: -margin.val(),
            y: -margin.val(),
            z: -margin.val(),
            child: Box::new(cube),
        }
    }
}
