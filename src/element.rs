use crate::var::Val;

#[derive(Clone)]
pub struct Element<'a>(pub(crate) InnerElement<'a>);

impl Element<'_> {
    #[must_use]
    pub fn empty() -> Self {
        Self(InnerElement::Empty)
    }
}

#[derive(Clone, strum::Display)]
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
}
