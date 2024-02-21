use std::fmt::Display;

use crate::Val;

#[derive(Clone)]
pub enum Calc<'a> {
    Neg(Val<'a>),
    Add(Val<'a>, Val<'a>),
    Sub(Val<'a>, Val<'a>),
    Mul(Val<'a>, Val<'a>),
    Div(Val<'a>, Val<'a>),
}

impl<'a> Calc<'a> {
    pub fn neg(val: Val<'a>) -> Val<'a> {
        if let Val::Val(float) = val {
            Val::Val(-float)
        } else {
            Val::Calc(Box::new(Calc::Neg(val)))
        }
    }

    pub(crate) fn add(a: Val<'a>, b: Val<'a>) -> Val<'a> {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a + float_b);
            }
        }
        Val::Calc(Box::new(Calc::Add(a, b)))
    }
    pub(crate) fn sub(a: Val<'a>, b: Val<'a>) -> Val<'a> {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a - float_b);
            }
        }
        Val::Calc(Box::new(Calc::Sub(a, b)))
    }
    pub(crate) fn mul(a: Val<'a>, b: Val<'a>) -> Val<'a> {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a * float_b);
            }
        }
        Val::Calc(Box::new(Calc::Mul(a, b)))
    }
    pub(crate) fn div(a: Val<'a>, b: Val<'a>) -> Val<'a> {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a / float_b);
            }
        }
        Val::Calc(Box::new(Calc::Div(a, b)))
    }
}

impl Display for Calc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calc::Neg(val) => write!(f, "-{val}"),
            Calc::Add(a, b) => write!(f, "{a} + {b}"),
            Calc::Sub(a, b) => write!(f, "{a} - {b}"),
            Calc::Mul(a, b) => write!(f, "{a} * {b}"),
            Calc::Div(a, b) => write!(f, "{a} / {b}"),
        }
    }
}
