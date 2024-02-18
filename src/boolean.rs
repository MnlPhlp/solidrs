// InnerElement impls

use crate::element::{Element, InnerElement};

impl std::ops::Add for InnerElement {
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

impl std::ops::Sub for InnerElement {
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

impl std::ops::Add for Element {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign for Element {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.clone() + rhs.0
    }
}

impl std::ops::Sub for Element {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign for Element {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.clone() - rhs.0
    }
}
