use std::sync::{atomic::AtomicU32, Mutex};

use crate::calc::Calc;

pub static VAR_ID: AtomicU32 = AtomicU32::new(0);
#[macro_export]
macro_rules! calc {
    ($var:ident,$val:expr) => {
        let $var = Var::new(
            stringify!($var),
            $val,
            $crate::VAR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        );
        let $var = &$var;
    };
    ($var:ident,$val:expr,$comment:literal) => {
        let $var = Var::commented(
            stringify!($var),
            $comment,
            $val,
            $crate::VAR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        );
        let $var = &$var;
    };
}

pub trait Arg: Sized {
    fn val(self) -> Val;
}
impl Arg for &'static Var {
    fn val(self) -> Val {
        Val::Var(self)
    }
}
impl Arg for Val {
    fn val(self) -> Val {
        self
    }
}

impl Arg for f32 {
    fn val(self) -> Val {
        Val::Val(self)
    }
}
impl Arg for i32 {
    #[allow(clippy::cast_precision_loss)]
    fn val(self) -> Val {
        Val::Val(self as f32)
    }
}

pub struct Var {
    name: &'static str,
    comment: &'static str,
    val: Mutex<Val>,
    id: u32,
}
impl Var {
    #[must_use]
    pub const fn new(name: &'static str, val: Val, id: u32) -> Var {
        Self {
            name,
            comment: "",
            val: Mutex::new(val),
            id,
        }
    }
    #[must_use]
    pub const fn commented(name: &'static str, comment: &'static str, val: Val, id: u32) -> Var {
        Self {
            name,
            comment,
            val: Mutex::new(val),
            id,
        }
    }
    pub fn set(&self, val: impl Arg) {
        *self.val.lock().unwrap() = val.val();
    }
    pub(crate) fn get_comment(&self) -> &'static str {
        self.comment
    }
    pub(crate) fn get_name(&self) -> &'static str {
        self.name
    }
    pub(crate) fn get_val(&self) -> Val {
        self.val.lock().unwrap().clone()
    }
    pub(crate) fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Copy)]
pub enum Val {
    Val(f32),
    Var(&'static Var),
    Calc(Calc),
}
impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Val(val) => f.write_str(val.to_string().as_str()),
            Val::Var(var) => f.write_str(var.name),
            Val::Calc(calc) => calc.fmt(f),
        }
    }
}

impl std::ops::Neg for Val {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Calc::neg(self)
    }
}
impl std::ops::Neg for &'static Var {
    type Output = Val;

    fn neg(self) -> Self::Output {
        Calc::neg(Val::Var(self))
    }
}

macro_rules! impl_op {
    ($op:ident,$func:ident,$t:ty) => {
        impl<RHS: Arg> std::ops::$op<RHS> for $t {
            type Output = Val;

            fn $func(self, rhs: RHS) -> Self::Output {
                Calc::$func(self.val(), rhs.val())
            }
        }
    };
}

impl_op!(Add, add, Val);
impl_op!(Sub, sub, Val);
impl_op!(Mul, mul, Val);
impl_op!(Div, div, Val);

impl_op!(Add, add, &'static Var);
impl_op!(Sub, sub, &'static Var);
impl_op!(Mul, mul, &'static Var);
impl_op!(Div, div, &'static Var);
