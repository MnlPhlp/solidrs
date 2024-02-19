use crate::Num;

#[derive(Clone, Copy)]
pub struct Var<N: Num>(pub N);

impl<N: Num> Num for Var<N> {
    fn f32(&self) -> f32 {
        self.0.f32()
    }
}

impl<N: Num> std::ops::Neg for Var<N> {
    type Output = Var<f32>;

    fn neg(self) -> Self::Output {
        Var(-self.0.f32())
    }
}
impl<A: Num, B: Num> std::ops::Add<B> for Var<A> {
    type Output = Var<f32>;

    fn add(self, rhs: B) -> Self::Output {
        Var(self.0.f32() + rhs.f32())
    }
}
impl<A: Num, B: Num> std::ops::Sub<B> for Var<A> {
    type Output = Var<f32>;

    fn sub(self, rhs: B) -> Self::Output {
        Var(self.0.f32() - rhs.f32())
    }
}
impl<A: Num, B: Num> std::ops::Mul<B> for Var<A> {
    type Output = Var<f32>;

    fn mul(self, rhs: B) -> Self::Output {
        Var(self.0.f32() * rhs.f32())
    }
}
impl<A: Num, B: Num> std::ops::Div<B> for Var<A> {
    type Output = Var<f32>;

    fn div(self, rhs: B) -> Self::Output {
        Var(self.0.f32() / rhs.f32())
    }
}
