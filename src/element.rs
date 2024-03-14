use std::fmt::Display;

use crate::var::Val;

#[derive(Clone)]
pub struct Element(pub(crate) InnerElement);

impl Element {
    #[must_use]
    pub fn empty() -> Self {
        Self(InnerElement::Empty)
    }
}

#[derive(Clone)]
pub(crate) enum InnerElement {
    Empty,
    Cube {
        x: Val,
        y: Val,
        z: Val,
        centered: bool,
    },
    Cylinder {
        h: Val,
        r: Val,
        centered: bool,
    },
    Square {
        x: Val,
        y: Val,
        centered: bool,
    },
    Union {
        children: Vec<InnerElement>,
    },
    Diff {
        children: Vec<InnerElement>,
    },
    Translate {
        x: Val,
        y: Val,
        z: Val,
        child: Box<InnerElement>,
    },
    Rotate {
        x: Val,
        y: Val,
        z: Val,
        child: Box<InnerElement>,
    },
    RotateExtrude {
        angle: Val,
        child: Box<InnerElement>,
    },
    Fa {
        fa: Val,
        child: Box<InnerElement>,
    },
    Fs {
        fs: Val,
        child: Box<InnerElement>,
    },
    Fn {
        f_n: Val,
        child: Box<InnerElement>,
    },
}

impl Display for InnerElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            InnerElement::Empty => "empty",
            InnerElement::Cube { .. } => "cube",
            InnerElement::Cylinder { .. } => "cylinder",
            InnerElement::Square { .. } => "square",
            InnerElement::Union { .. } => "union",
            InnerElement::Diff { .. } => "difference",
            InnerElement::Translate { .. } => "translate",
            InnerElement::Rotate { .. } => "rotate",
            InnerElement::RotateExtrude { .. } => "rotate_extrude",
            InnerElement::Fa { .. } => "fa",
            InnerElement::Fs { .. } => "fs",
            InnerElement::Fn { .. } => "fn",
        };
        write!(f, "{name}")
    }
}
