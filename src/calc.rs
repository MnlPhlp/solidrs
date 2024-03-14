use std::{fmt::Display, sync::Arc};

use crate::Val;

#[derive(Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub enum CalcOp {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Clone)]
pub struct Calc {
    pub op: CalcOp,
    pub a: Arc<Val>,
    pub b: Arc<Val>,
}

impl Calc {
    pub fn neg(val: Val) -> Self {
        Calc {
            op: CalcOp::Neg,
            a: val.into(),
            b: Arc::new(Val::Val(0.)),
        }
    }

    pub(crate) fn add(a: Val, b: Val) -> Calc {
        Calc {
            op: CalcOp::Add,
            a: a.into(),
            b: b.into(),
        }
    }
    pub(crate) fn sub(a: Val, b: Val) -> Calc {
        Calc {
            op: CalcOp::Sub,
            a: a.into(),
            b: b.into(),
        }
    }
    pub(crate) fn mul(a: Val, b: Val) -> Calc {
        Calc {
            op: CalcOp::Mul,
            a: a.into(),
            b: b.into(),
        }
    }
    pub(crate) fn div(a: Val, b: Val) -> Calc {
        Calc {
            op: CalcOp::Div,
            a: a.into(),
            b: b.into(),
        }
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = &self.a;
        let b = &self.b;
        match self.op {
            CalcOp::Neg => write!(f, "-{a}"),
            CalcOp::Add => write!(f, "{a} + {b}"),
            CalcOp::Sub => write!(f, "{a} - {b}"),
            CalcOp::Mul => write!(f, "{a} * {b}"),
            CalcOp::Div => write!(f, "{a} / {b}"),
        }
    }
}
