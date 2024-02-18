pub struct Element(pub(crate) InnerElement);

#[derive(Clone)]
pub(crate) enum InnerElement {
    Cube {
        x: u32,
        y: u32,
        z: u32,
        centered: bool,
    },
    Union {
        children: Vec<InnerElement>,
    },
    Diff {
        children: Vec<InnerElement>,
    },
    Translate {
        x: u32,
        y: u32,
        z: u32,
        child: Box<InnerElement>,
    },
}
