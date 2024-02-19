use crate::{element::InnerElement, Arg, Element};

impl<'a> Element<'a> {
    #[must_use]
    pub fn set_fa(self, fa: impl Arg<'a>) -> Self {
        Element(InnerElement::Fa {
            fa: fa.val(),
            child: Box::new(self.0),
        })
    }
    #[must_use]
    pub fn set_fs(self, fs: impl Arg<'a>) -> Self {
        Element(InnerElement::Fs {
            fs: fs.val(),
            child: Box::new(self.0),
        })
    }
}
