use std::fmt::Display;

use crate::var::Val;

#[derive(Clone)]
pub struct Element<'a>(pub(crate) InnerElement<'a>);

impl Element<'_> {
    #[must_use]
    pub fn empty() -> Self {
        Self(InnerElement::Empty)
    }
}

#[derive(Clone)]
pub(crate) enum InnerElement<'a> {
    Empty,
    Cube {
        x: Val<'a>,
        y: Val<'a>,
        z: Val<'a>,
        centered: bool,
    },
    Cylinder {
        h: Val<'a>,
        r: Val<'a>,
        centered: bool,
    },
    Square {
        x: Val<'a>,
        y: Val<'a>,
        centered: bool,
    },
    Union {
        children: Vec<InnerElement<'a>>,
    },
    Diff {
        children: Vec<InnerElement<'a>>,
    },
    Translate {
        x: Val<'a>,
        y: Val<'a>,
        z: Val<'a>,
        child: Box<InnerElement<'a>>,
    },
    Rotate {
        x: Val<'a>,
        y: Val<'a>,
        z: Val<'a>,
        child: Box<InnerElement<'a>>,
    },
    RotateExtrude {
        angle: Val<'a>,
        child: Box<InnerElement<'a>>,
    },
    Fa {
        fa: Val<'a>,
        child: Box<InnerElement<'a>>,
    },
    Fs {
        fs: Val<'a>,
        child: Box<InnerElement<'a>>,
    },
}

impl Display for InnerElement<'_> {
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
        };
        write!(f, "{}", name)
    }
}
