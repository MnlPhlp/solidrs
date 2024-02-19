use crate::{element::InnerElement, Element, Num};

impl Element {
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

    pub fn margin(&self, margin: impl Num) -> Self {
        Element(self.0.margin(margin.f32()))
    }
}

impl InnerElement {
    pub fn margin(&self, margin: f32) -> Self {
        match self {
            InnerElement::Cube { x, y, z, centered } => {
                margin_cube(*x, *y, *z, *centered, margin.f32())
            }
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

fn margin_cube(x: f32, y: f32, z: f32, centered: bool, margin: f32) -> InnerElement {
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
            x: -margin,
            y: -margin,
            z: -margin,
            child: Box::new(cube),
        }
    }
}
