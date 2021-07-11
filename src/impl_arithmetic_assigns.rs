use std::ops::*;

use crate::{AutoBigInt, BigInt};
use AutoBigInt::{Big, Little};

macro_rules! impl_arithmetic_assign {
    ($f:ident,$g:ident,$checked_g:ident,$t:tt) => {
        impl $t<AutoBigInt> for AutoBigInt {
            fn $f(&mut self, other: AutoBigInt) {
                if let Some(x) = self.little() {
                    *self = match other {
                        Little(y) => {
                            if let Some(result) = x.$checked_g(y) {
                                Little(result)
                            } else {
                                Big(BigInt::from(x).$g(y))
                            }
                        }
                        Big(y) => Big(BigInt::from(x).$g(y)),
                    }
                } else {
                    let x = self.big_mut().unwrap();
                    match other {
                        Little(y) => x.$f(y),
                        Big(y) => x.$f(y),
                    }
                }
            }
        }
        impl<'a> $t<&'a AutoBigInt> for AutoBigInt {
            fn $f(&mut self, other: &AutoBigInt) {
                if let Some(x) = self.little() {
                    *self = match other {
                        &Little(y) => {
                            if let Some(result) = x.$checked_g(y) {
                                Little(result)
                            } else {
                                Big(BigInt::from(x).$g(y))
                            }
                        }
                        Big(y) => Big(BigInt::from(x).$g(y)),
                    }
                } else {
                    let x = self.big_mut().unwrap();
                    match other {
                        &Little(y) => x.$f(y),
                        Big(y) => x.$f(y),
                    }
                }
            }
        }
    };
}

impl_arithmetic_assign!(add_assign, add, checked_add, AddAssign);
impl_arithmetic_assign!(sub_assign, sub, checked_sub, SubAssign);
impl_arithmetic_assign!(mul_assign, mul, checked_mul, MulAssign);
impl_arithmetic_assign!(div_assign, div, checked_div, DivAssign);
impl_arithmetic_assign!(rem_assign, rem, checked_rem, RemAssign);
