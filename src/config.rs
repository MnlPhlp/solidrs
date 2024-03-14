use crate::{element::InnerElement, Arg, Element};

impl Element {
    #[must_use]
    pub fn set_fa(&self, fa: impl Arg) -> Self {
        Element(InnerElement::Fa {
            fa: fa.val(),
            child: Box::new(self.0.clone()),
        })
    }
    #[must_use]
    pub fn set_fs(&self, fs: impl Arg) -> Self {
        Element(InnerElement::Fs {
            fs: fs.val(),
            child: Box::new(self.0.clone()),
        })
    }
    #[must_use]
    pub fn set_fn(&self, f_n: impl Arg) -> Self {
        Element(InnerElement::Fn {
            f_n: f_n.val(),
            child: Box::new(self.0.clone()),
        })
    }
}
