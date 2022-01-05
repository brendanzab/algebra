// Copyright 2013-2014 The Algebra Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ops::{Add, Mul};

use crate::cmp::ApproxEq;
use crate::ops::{Additive, Multiplicative, Op};

/// Types that are approximately closed under a given operator.
///
/// ~~~notrust
/// a, b ∈ Self ⇒ ∃ c ≈ a ∘ b such that c ∈ Self
/// ~~~
pub trait MagmaApprox<O: Op>: Sized + PartialEq + ApproxEq + Clone {
    /// Performs an operation.
    fn approx(self, _: Self) -> Self;
    /// Performs specific operation.
    fn ap(self, _: O, lhs: Self) -> Self {
        self.approx(lhs)
    }
}

/// Types that are closed under a given operator.
///
/// ~~~notrust
/// a, b ∈ Self ⇒ a ∘ b ∈ Self
/// ~~~
pub trait Magma<O: Op>: Eq + MagmaApprox<O> {
    /// Performs an operation.
    fn operate(self, lhs: Self) -> Self {
        self.approx(lhs)
    }
    /// Performs specific operation.
    fn op(self, _: O, lhs: Self) -> Self {
        self.operate(lhs)
    }
}

impl<T> MagmaApprox<Additive> for T
where
    T: Add<T, Output = T> + PartialEq + ApproxEq + Clone,
{
    fn approx(self, lhs: Self) -> Self {
        self + lhs
    }
}

impl<T> Magma<Additive> for T where T: MagmaApprox<Additive> + Eq {}

impl<T> MagmaApprox<Multiplicative> for T
where
    T: Mul<T, Output = T> + PartialEq + ApproxEq + Clone,
{
    fn approx(self, lhs: Self) -> Self {
        self * lhs
    }
}

impl<T> Magma<Multiplicative> for T where T: MagmaApprox<Multiplicative> + Eq {}
