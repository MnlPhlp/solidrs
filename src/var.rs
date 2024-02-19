#[macro_export]
macro_rules! var {
    ($var:ident,$val:literal) => {
        let $var = Var::new(stringify!($var), $crate::Arg::f32(&$val));
        let $var = Val::Var(&$var);
    };
    ($var:ident,$val:literal,$comment:literal) => {
        let $var = Var::commented(stringify!($var), $comment, $crate::Arg::f32(&$val));
        let $var = Val::Var(&$var);
    };
}

pub trait Arg<'a>: Sized {
    fn val(self) -> Val<'a> {
        Val::Val(self.f32())
    }
    fn f32(&self) -> f32;
}
impl<'a> Arg<'a> for &'a Var {
    fn val(self) -> Val<'a> {
        Val::Var(self)
    }
    fn f32(&self) -> f32 {
        self.val
    }
}
impl<'a> Arg<'a> for Val<'a> {
    fn val(self) -> Val<'a> {
        self
    }
    fn f32(&self) -> f32 {
        match self {
            Val::Val(val) => *val,
            Val::Var(var) => var.val,
        }
    }
}
impl<'a> Arg<'a> for f32 {
    fn f32(&self) -> f32 {
        *self
    }
}
impl<'a> Arg<'a> for i32 {
    #[allow(clippy::cast_precision_loss)]
    fn f32(&self) -> f32 {
        *self as f32
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

impl std::ops::Neg for Val<'_> {
    type Output = Val<'static>;

    fn neg(self) -> Self::Output {
        Val::Val(-self.f32())
    }
}
impl<'a, RHS: Arg<'a>> std::ops::Add<RHS> for Val<'_> {
    type Output = Val<'static>;

    fn add(self, rhs: RHS) -> Self::Output {
        Val::Val(self.f32() + rhs.f32())
    }
}
impl<'a, RHS: Arg<'a>> std::ops::Sub<RHS> for Val<'_> {
    type Output = Val<'static>;

    fn sub(self, rhs: RHS) -> Self::Output {
        Val::Val(self.f32() - rhs.f32())
    }
}
impl<'a, RHS: Arg<'a>> std::ops::Mul<RHS> for Val<'_> {
    type Output = Val<'static>;

    fn mul(self, rhs: RHS) -> Self::Output {
        Val::Val(self.f32() * rhs.f32())
    }
}
impl<'a, RHS: Arg<'a>> std::ops::Div<RHS> for Val<'_> {
    type Output = Val<'static>;

    fn div(self, rhs: RHS) -> Self::Output {
        Val::Val(self.f32() / rhs.f32())
    }
}
#[derive(Copy, Clone)]
pub enum Val<'a> {
    Val(f32),
    Var(&'a Var),
}
impl std::fmt::Display for Val<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Val(val) => f.write_str(val.to_string().as_str()),
            Val::Var(var) => f.write_str(var.name),
        }
    }
}
