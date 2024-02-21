use crate::calc::Calc;

#[macro_export]
macro_rules! var {
    ($var:ident,$val:literal) => {
        let $var = Var::new(stringify!($var), $val as f32);
        let $var = &$var;
    };
    ($var:ident,$val:literal,$comment:literal) => {
        let $var = Var::commented(stringify!($var), $comment, $val as f32);
        let $var = &$var;
    };
}

pub trait Arg<'a>: Sized {
    fn val(self) -> Val<'a>;
}
impl<'a> Arg<'a> for &'a Var {
    fn val(self) -> Val<'a> {
        Val::Var(self)
    }
}
impl<'a> Arg<'a> for Val<'a> {
    fn val(self) -> Val<'a> {
        self
    }
}

impl<'a> Arg<'a> for f32 {
    fn val(self) -> Val<'a> {
        Val::Val(self)
    }
}
impl<'a> Arg<'a> for i32 {
    #[allow(clippy::cast_precision_loss)]
    fn val(self) -> Val<'a> {
        Val::Val(self as f32)
    }
}

#[derive(Clone)]
pub struct Var {
    name: &'static str,
    comment: &'static str,
    val: f32,
}
impl Var {
    #[must_use]
    pub fn new(name: &'static str, val: f32) -> Var {
        Self {
            name,
            comment: "",
            val,
        }
    }
    #[must_use]
    pub fn commented(name: &'static str, comment: &'static str, val: f32) -> Var {
        Self { name, comment, val }
    }
    pub(crate) fn get_comment(&self) -> &'static str {
        self.comment
    }
    pub(crate) fn get_name(&self) -> &'static str {
        self.name
    }
    pub(crate) fn get_val(&self) -> f32 {
        self.val
    }
}

#[derive(Clone)]
pub enum Val<'a> {
    Val(f32),
    Var(&'a Var),
    Calc(Box<Calc<'a>>),
}
impl std::fmt::Display for Val<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Val(val) => f.write_str(val.to_string().as_str()),
            Val::Var(var) => f.write_str(var.name),
            Val::Calc(calc) => calc.fmt(f),
        }
    }
}

impl std::ops::Neg for Val<'_> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Calc::neg(self)
    }
}
impl<'a> std::ops::Neg for &'a Var {
    type Output = Val<'a>;

    fn neg(self) -> Self::Output {
        Calc::neg(Val::Var(self))
    }
}

macro_rules! impl_op {
    ($op:ident,$func:ident,$t:ty) => {
        impl<'a, RHS: Arg<'a>> std::ops::$op<RHS> for $t {
            type Output = Val<'a>;

            fn $func(self, rhs: RHS) -> Self::Output {
                Calc::$func(self.val(), rhs.val())
            }
        }
    };
}

impl_op!(Add, add, Val<'a>);
impl_op!(Sub, sub, Val<'a>);
impl_op!(Mul, mul, Val<'a>);
impl_op!(Div, div, Val<'a>);

impl_op!(Add, add, &'a Var);
impl_op!(Sub, sub, &'a Var);
impl_op!(Mul, mul, &'a Var);
impl_op!(Div, div, &'a Var);
