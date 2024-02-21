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
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
                centered: true,
            },
            InnerElement::Cylinder { h, r, .. } => InnerElement::Cylinder {
                h: h.clone(),
                r: r.clone(),
                centered: true,
            },
            // ToDo fix this with types to not allow invalid calls
            other => panic!("center() is not allowed for type {other}"),
        };
        Element(inner)
    }

    #[must_use]
    pub fn margin(&self, margin: impl Arg<'a>) -> Self {
        Element(self.0.clone().margin(margin.val()))
    }
}

impl<'a> InnerElement<'a> {
    #[must_use]
    fn margin(self, margin: Val<'a>) -> Self {
        match self {
            InnerElement::Cube { x, y, z, centered } => margin_cube(x, y, z, centered, margin),
            Self::Translate { x, y, z, child } => Self::Translate {
                x,
                y,
                z,
                child: Box::new(child.margin(margin)),
            },

            // ToDo fix this with types to not allow invalid calls
            other => panic!("margin() is not allowed for type {other}"),
        }
    }
}

fn margin_cube<'a>(
    x: Val<'a>,
    y: Val<'a>,
    z: Val<'a>,
    centered: bool,
    margin: Val<'a>,
) -> InnerElement<'a> {
    let cube = InnerElement::Cube {
        x: x + margin.clone() * 2,
        y: y + margin.clone() * 2,
        z: z + margin.clone() * 2,
        centered,
    };
    if centered {
        cube
    } else {
        InnerElement::Translate {
            x: -margin.clone(),
            y: -margin.clone(),
            z: -margin,
            child: Box::new(cube),
        }
    }
}
