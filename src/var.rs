use crate::calc::Calc;

#[macro_export]
macro_rules! var {
    ($var:ident,$val:literal) => {
        const $var: Var = Var::new(stringify!($var), stringify!($val), false);
    };
    ($var:ident,$val:literal,$comment:literal) => {
        #[doc = $comment]
        const $var: Var = Var::commented(stringify!($var), $comment, stringify!($val), false);
    };
}
#[macro_export]
macro_rules! calc {
    ($var:ident,$val:expr) => {
        const $var: Var = Var::new(stringify!($var), stringify!($val), true);
    };
    ($var:ident,$val:expr,$comment:literal) => {
        #[doc = $comment]
        const $var: Var = Var::commented(stringify!($var), $comment, stringify!($val), true);
    };
}

pub trait Arg: Sized {
    fn val(self) -> Val;
}
impl Arg for Var {
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
#[derive(Clone, Copy)]
pub struct Var {
    name: &'static str,
    comment: &'static str,
    val: &'static str,
    calc: bool,
}
impl Var {
    #[must_use]
    pub const fn new(name: &'static str, val: &'static str, calc: bool) -> Var {
        Self {
            name,
            comment: "",
            val,
            calc,
        }
    }
    #[must_use]
    pub const fn commented(
        name: &'static str,
        comment: &'static str,
        val: &'static str,
        calc: bool,
    ) -> Var {
        Self {
            name,
            comment,
            val,
            calc,
        }
    }
    pub(crate) fn get_comment(&self) -> &'static str {
        self.comment
    }
    pub(crate) fn get_name(&self) -> &'static str {
        self.name
    }
    pub(crate) fn get_val(&self) -> &'static str {
        self.val
    }

    pub(crate) fn is_clac(&self) -> bool {
        self.calc
    }
}

#[derive(Clone, Copy)]
pub enum Val {
    Val(f32),
    Var(Var),
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
impl std::ops::Neg for Var {
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

impl_op!(Add, add, Var);
impl_op!(Sub, sub, Var);
impl_op!(Mul, mul, Var);
impl_op!(Div, div, Var);
