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
        x: f32,
        y: f32,
        z: f32,
        centered: bool,
    },
    Cylinder {
        h: f32,
        r: f32,
        centered: bool,
    },
    Square {
        x: f32,
        y: f32,
        centered: bool,
    },
    Union {
        children: Vec<InnerElement>,
    },
    Diff {
        children: Vec<InnerElement>,
    },
    Translate {
        x: f32,
        y: f32,
        z: f32,
        child: Box<InnerElement>,
    },
    Rotate {
        x: f32,
        y: f32,
        z: f32,
        child: Box<InnerElement>,
    },
    RotateExtrude {
        angle: f32,
        child: Box<InnerElement>,
    },
}
