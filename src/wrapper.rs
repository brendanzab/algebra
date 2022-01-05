//! Wrappers that attach an algebraic structure with a value type.

use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::cmp::ApproxEq;
use crate::ident::Identity;
use crate::ops::{Additive, Inverse, Multiplicative, Op, Recip};

use crate::structure::MagmaApprox;
use crate::structure::QuasigroupApprox;

/// Wrapper that allows to use operators on algebraic types.
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub struct Wrapper<M>(pub M);

impl<M: Display> Display for Wrapper<M> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(fmt)
    }
}

/// Creates wrapper with identity value for a specific operator.
pub fn id<O: Op, M>(_: O) -> Wrapper<M>
where
    M: Identity<O>,
{
    Wrapper(Identity::<O>::id())
}

impl<M> ApproxEq for Wrapper<M>
where
    M: ApproxEq,
{
    type Eps = M::Eps;
    fn default_epsilon() -> Self::Eps {
        M::default_epsilon()
    }

    fn approx_eq_eps(&self, b: &Self, epsilon: &Self::Eps) -> bool {
        M::approx_eq_eps(&self.0, &b.0, epsilon)
    }
}

impl<M> Add<Wrapper<M>> for Wrapper<M>
where
    M: MagmaApprox<Additive>,
{
    type Output = Self;
    fn add(self, lhs: Self) -> Self {
        Wrapper(self.0.approx(lhs.0))
    }
}

impl<M> Neg for Wrapper<M>
where
    M: QuasigroupApprox<Additive>,
{
    type Output = Self;
    fn neg(mut self) -> Self {
        self.0 = self.0.inv();
        self
    }
}

impl<M> Sub<Wrapper<M>> for Wrapper<M>
where
    M: QuasigroupApprox<Additive>,
{
    type Output = Self;
    fn sub(self, lhs: Self) -> Self {
        self + -lhs
    }
}

impl<M> Mul<Wrapper<M>> for Wrapper<M>
where
    M: MagmaApprox<Multiplicative>,
{
    type Output = Self;
    fn mul(self, lhs: Self) -> Self {
        Wrapper(self.0.approx(lhs.0))
    }
}

impl<M> Recip for Wrapper<M>
where
    M: QuasigroupApprox<Multiplicative>,
{
    type Result = Self;
    fn recip(self) -> Self {
        Wrapper(self.0.inv())
    }
}

impl<M> Div<Wrapper<M>> for Wrapper<M>
where
    M: QuasigroupApprox<Multiplicative>,
{
    type Output = Self;
    fn div(self, lhs: Self) -> Self {
        self * lhs.inv()
    }
}
