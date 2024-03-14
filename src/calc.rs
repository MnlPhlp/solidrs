use std::{fmt::Display, sync::Arc};

use crate::{Val, Var};

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
#[allow(clippy::module_name_repetitions)]
pub enum CalcOp {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

impl Calc {
    pub fn neg(val: Val) -> Self {
        Calc {
            op: CalcOp::Neg,
            a: val.into(),
            b: Val0::Val(0.),
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

// macro to build nested calculations
macro_rules! calc_struct {
    ($calc:ident,$sub:ident,$par_val:ident,$val:ident) => {
        #[derive(Clone, Copy)]
        pub struct $calc {
            pub op: CalcOp,
            pub a: $val,
            pub b: $val,
        }
        #[derive(Clone, Copy)]
        pub enum $val {
            Val(f32),
            Var(Var),
            Calc($sub),
        }
        impl From<$par_val> for $val {
            fn from(value: $par_val) -> Self {
                match value {
                    $par_val::Val(v) => $val::Val(v),
                    $par_val::Var(v) => $val::Var(v),
                    $par_val::Calc(c) => $val::Calc($sub {
                        op: c.op,
                        a: c.a.into(),
                        b: c.b.into(),
                    }),
                }
            }
        }
        impl From<$val> for $par_val {
            fn from(val: $val) -> Self {
                match val {
                    $val::Val(v) => $par_val::Val(v),
                    $val::Var(v) => $par_val::Var(v),
                    $val::Calc($sub { op, a, b }) => $par_val::Calc($calc {
                        op,
                        a: a.into(),
                        b: b.into(),
                    }),
                }
            }
        }
    };
    ($calc:ident,$par_val:ident,$val:ident) => {
        #[derive(Clone, Copy)]
        pub struct $calc {
            pub op: CalcOp,
            pub a: $val,
            pub b: $val,
        }
        #[derive(Clone, Copy)]
        pub enum $val {
            Val(f32),
            Var(Var),
        }
        impl From<$par_val> for $val {
            fn from(value: $par_val) -> Self {
                match value {
                    $par_val::Val(v) => $val::Val(v),
                    $par_val::Var(v) => $val::Var(v),
                    $par_val::Calc(_) => panic!("calculations nested to deep"),
                }
            }
        }
        impl From<$val> for $par_val {
            fn from(val: $val) -> Self {
                match val {
                    $val::Val(v) => $par_val::Val(v),
                    $val::Var(v) => $par_val::Var(v),
                }
            }
        }
    };
}

// first level using normal Val
#[derive(Clone, Copy)]
pub struct Calc {
    pub op: CalcOp,
    pub a: Val0,
    pub b: Val0,
}
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
            Val0::Calc(c) => Val::Calc(Arc::new(Calc {
                op: c.op,
                a: c.a.into(),
                b: c.b.into(),
            })),
        }
    }
}
// nested calculations
calc_struct!(SubCalc1, SubCalc2, Val0, Val1);
// calc_struct!(SubCalc2, SubCalc3, Val1, Val2);

// final nesting that produces panic when trying to nest further
calc_struct!(SubCalc2, Val1, Val2);
