// InnerElement impls

use crate::element::{Element, InnerElement};

impl std::ops::Add for InnerElement<'_> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        match self {
            InnerElement::Union { ref mut children } => {
                children.push(rhs);
                self
            }
            _ => InnerElement::Union {
                children: vec![self, rhs],
            },
        }
    }
}

impl std::ops::Sub for InnerElement<'_> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        match self {
            InnerElement::Diff { ref mut children } => {
                children.push(rhs);
                self
            }
            _ => InnerElement::Diff {
                children: vec![self, rhs],
            },
        }
    }
}

// Element impls

impl std::ops::Add for Element<'_> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, RHS: Iterator<Item = Element<'a>>> std::ops::Add<RHS> for Element<'a> {
    type Output = Self;

    fn add(mut self, rhs: RHS) -> Self::Output {
        for e in rhs {
            self += e;
        }
        self
    }
}

impl std::ops::AddAssign for Element<'_> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.clone() + rhs.0
    }
}
impl<'a, RHS: IntoIterator<Item = Element<'a>>> std::ops::AddAssign<RHS> for Element<'a> {
    fn add_assign(&mut self, rhs: RHS) {
        for e in rhs {
            *self += e;
        }
    }
}

impl std::ops::Sub for Element<'_> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign for Element<'_> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.clone() - rhs.0
    }
}
