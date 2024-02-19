#[macro_export]
macro_rules! var {
    ($var:ident,$val:literal) => {
        let $var = Var::named(stringify!($var), $crate::Arg::val($val));
    };
    ($var:ident,$val:literal,$comment:literal) => {
        let $var = Var::commented(stringify!($var), $comment, $crate::Arg::val($val));
    };
}

pub trait Arg: Sized {
    fn var(self) -> Var {
        Var::val(self.val())
    }
    fn val(self) -> f32;
}
impl Arg for Var {
    fn var(self) -> Var {
        self
    }
    fn val(self) -> f32 {
        self.val
    }
}
impl Arg for f32 {
    fn val(self) -> f32 {
        self
    }
}
impl Arg for i32 {
    fn val(self) -> f32 {
        self as f32
    }
}

#[derive(Clone, Copy)]
pub struct Var {
    name: &'static str,
    comment: &'static str,
    val: f32,
}
impl Var {
    pub fn val(val: f32) -> Var {
        Self {
            name: "",
            comment: "",
            val,
        }
    }
    pub fn named(name: &'static str, val: f32) -> Var {
        Self {
            name,
            comment: "",
            val,
        }
    }
    pub fn commented(name: &'static str, comment: &'static str, val: f32) -> Var {
        Self { name, comment, val }
    }
    pub fn get_comment(&self) -> &'static str {
        self.comment
    }
    pub fn get_name(&self) -> &'static str {
        self.name
    }
    pub fn get_val(&self) -> f32 {
        self.val
    }
}
impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() {
            f.write_str(self.val.to_string().as_str())
        } else {
            f.write_str(self.name)
        }
    }
}

impl std::ops::Neg for Var {
    type Output = Var;

    fn neg(self) -> Self::Output {
        Var::val(-self.val)
    }
}
impl<RHS: Arg> std::ops::Add<RHS> for Var {
    type Output = Var;

    fn add(self, rhs: RHS) -> Self::Output {
        Var::val(self.val + rhs.val())
    }
}
impl<RHS: Arg> std::ops::Sub<RHS> for Var {
    type Output = Var;

    fn sub(self, rhs: RHS) -> Self::Output {
        Var::val(self.val - rhs.val())
    }
}
impl<RHS: Arg> std::ops::Mul<RHS> for Var {
    type Output = Var;

    fn mul(self, rhs: RHS) -> Self::Output {
        Var::val(self.val * rhs.val())
    }
}
impl<RHS: Arg> std::ops::Div<RHS> for Var {
    type Output = Var;

    fn div(self, rhs: RHS) -> Self::Output {
        Var::val(self.val / rhs.val())
    }
}
