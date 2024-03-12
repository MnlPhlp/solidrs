use std::fmt::Display;

use crate::{Val, Var};

#[allow(clippy::from_over_into)]
impl From<&Val0> for Val {
    fn from(val: &Val0) -> Self {
        Self::from(*val)
    }
}
impl Display for Val0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: Val = self.into();
        val.fmt(f)
    }
}

#[derive(Clone, Copy)]
pub struct Calc {
    pub op: CalcOp,
    pub a: Val0,
    pub b: Val0,
}
#[derive(Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub enum CalcOp {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

impl Calc {
    pub fn neg(val: Val) -> Val {
        Val::Calc(Calc {
            op: CalcOp::Neg,
            a: val.into(),
            b: Val0::Val(0.),
        })
    }

    pub(crate) fn add(a: Val, b: Val) -> Val {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a + float_b);
            }
        }
        Val::Calc(Calc {
            op: CalcOp::Add,
            a: a.into(),
            b: b.into(),
        })
    }
    pub(crate) fn sub(a: Val, b: Val) -> Val {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a - float_b);
            }
        }
        Val::Calc(Calc {
            op: CalcOp::Sub,
            a: a.into(),
            b: b.into(),
        })
    }
    pub(crate) fn mul(a: Val, b: Val) -> Val {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a * float_b);
            }
        }
        Val::Calc(Calc {
            op: CalcOp::Mul,
            a: a.into(),
            b: b.into(),
        })
    }
    pub(crate) fn div(a: Val, b: Val) -> Val {
        if let Val::Val(float_a) = a {
            if let Val::Val(float_b) = b {
                return Val::Val(float_a / float_b);
            }
        }
        Val::Calc(Calc {
            op: CalcOp::Div,
            a: a.into(),
            b: b.into(),
        })
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = self.a;
        let b = self.b;
        match self.op {
            CalcOp::Neg => write!(f, "-{a}"),
            CalcOp::Add => write!(f, "{a} + {b}"),
            CalcOp::Sub => write!(f, "{a} - {b}"),
            CalcOp::Mul => write!(f, "{a} * {b}"),
            CalcOp::Div => write!(f, "{a} / {b}"),
        }
    }
}

// no nesting
#[derive(Clone, Copy)]
pub enum Val0 {
    Val(f32),
    Var(Var),
    Calc(SubCalc1),
}
impl From<Val> for Val0 {
    fn from(value: Val) -> Self {
        match value {
            Val::Val(v) => Val0::Val(v),
            Val::Var(v) => Val0::Var(v),
            Val::Calc(c) => Val0::Calc(SubCalc1 {
                op: c.op,
                a: c.a.into(),
                b: c.b.into(),
            }),
        }
    }
}
impl From<Val0> for Val {
    fn from(val: Val0) -> Self {
        match val {
            Val0::Val(v) => Val::Val(v),
            Val0::Var(v) => Val::Var(v),
            Val0::Calc(SubCalc1 { op, a, b }) => Val::Calc(Calc {
                op,
                a: a.into(),
                b: b.into(),
            }),
        }
    }
}

// first level of nesting
#[derive(Clone, Copy)]
pub struct SubCalc1 {
    op: CalcOp,
    a: Val1,
    b: Val1,
}
#[derive(Clone, Copy)]
pub enum Val1 {
    Val(f32),
    Var(Var),
}
impl From<Val0> for Val1 {
    fn from(value: Val0) -> Self {
        match value {
            Val0::Val(v) => Val1::Val(v),
            Val0::Var(v) => Val1::Var(v),
            Val0::Calc(_) => panic!("calculations nested to deep"),
        }
    }
}
impl From<Val1> for Val0 {
    fn from(value: Val1) -> Self {
        match value {
            Val1::Val(v) => Val0::Val(v),
            Val1::Var(v) => Val0::Var(v),
        }
    }
}
