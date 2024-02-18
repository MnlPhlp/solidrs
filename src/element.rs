pub struct Element(pub(crate) InnerElement);

#[derive(Clone)]
pub(crate) enum InnerElement {
    Cube {
        x: i32,
        y: i32,
        z: i32,
        centered: bool,
    },
    Square {
        x: i32,
        y: i32,
        centered: bool,
    },
    Union {
        children: Vec<InnerElement>,
    },
    Diff {
        children: Vec<InnerElement>,
    },
    Translate {
        x: i32,
        y: i32,
        z: i32,
        child: Box<InnerElement>,
    },
    Rotate {
        x: i32,
        y: i32,
        z: i32,
        child: Box<InnerElement>,
    },
    RotateExtrude {
        angle: i32,
        child: Box<InnerElement>,
    },
}
