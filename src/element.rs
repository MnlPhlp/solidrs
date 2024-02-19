use crate::Var;

#[derive(Clone)]
pub struct Element(pub(crate) InnerElement);

impl Element {
    pub fn empty() -> Self {
        Self(InnerElement::Empty)
    }
}

#[derive(Clone, strum::Display)]
pub(crate) enum InnerElement {
    Empty,
    Cube {
        x: Var,
        y: Var,
        z: Var,
        centered: bool,
    },
    Cylinder {
        h: Var,
        r: Var,
        centered: bool,
    },
    Square {
        x: Var,
        y: Var,
        centered: bool,
    },
    Union {
        children: Vec<InnerElement>,
    },
    Diff {
        children: Vec<InnerElement>,
    },
    Translate {
        x: Var,
        y: Var,
        z: Var,
        child: Box<InnerElement>,
    },
    Rotate {
        x: Var,
        y: Var,
        z: Var,
        child: Box<InnerElement>,
    },
    RotateExtrude {
        angle: Var,
        child: Box<InnerElement>,
    },
}
