use std::ops::*;

use crate::{AutoBigInt, BigInt};
use AutoBigInt::{Big, Little};

macro_rules! impl_arithmetic {
    ($f:ident,$checked_f:ident, $t:tt) => {
        impl $t<AutoBigInt> for AutoBigInt {
            type Output = AutoBigInt;
            fn $f(self, other: AutoBigInt) -> AutoBigInt {
                let mut result = match (self, other) {
                    (Little(x), Little(y)) => x
                        .$checked_f(y)
                        .map(Little)
                        .unwrap_or_else(|| Big(BigInt::from(x).$f(y))),
                    (Little(x), Big(y)) => Big(BigInt::from(x).$f(y)),
                    (Big(x), Little(y)) => Big(x.$f(&BigInt::from(y))),
                    (Big(x), Big(y)) => Big(x.$f(y)),
                };
                result.shrink();
                result
            }
        }
        impl<'b> $t<&'b AutoBigInt> for AutoBigInt {
            type Output = AutoBigInt;
            fn $f(self, other: &AutoBigInt) -> AutoBigInt {
                let mut result = match (self, other) {
                    (Little(x), &Little(y)) => x
                        .$checked_f(y)
                        .map(Little)
                        .unwrap_or_else(|| Big(BigInt::from(x).$f(y))),
                    (Little(x), Big(y)) => Big(BigInt::from(x).$f(y)),
                    (Big(x), &Little(y)) => Big(x.$f(&BigInt::from(y))),
                    (Big(x), Big(y)) => Big(x.$f(y)),
                };
                result.shrink();
                result
            }
        }
        impl<'a> $t<AutoBigInt> for &'a AutoBigInt {
            type Output = AutoBigInt;
            fn $f(self, other: AutoBigInt) -> AutoBigInt {
                let mut result = match (self, other) {
                    (&Little(x), Little(y)) => x
                        .$checked_f(y)
                        .map(Little)
                        .unwrap_or_else(|| Big(BigInt::from(x).$f(y))),
                    (&Little(x), Big(y)) => Big(BigInt::from(x).$f(y)),
                    (Big(x), Little(y)) => Big(x.$f(&BigInt::from(y))),
                    (Big(x), Big(y)) => Big(x.$f(y)),
                };
                result.shrink();
                result
            }
        }
        impl<'a, 'b> $t<&'b AutoBigInt> for &'a AutoBigInt {
            type Output = AutoBigInt;
            fn $f(self, other: &AutoBigInt) -> AutoBigInt {
                let mut result = match (self, other) {
                    (&Little(x), &Little(y)) => x
                        .$checked_f(y)
                        .map(Little)
                        .unwrap_or_else(|| Big(BigInt::from(x).$f(y))),
                    (&Little(x), Big(y)) => Big(BigInt::from(x).$f(y)),
                    (Big(x), &Little(y)) => Big(x.$f(&BigInt::from(y))),
                    (Big(x), Big(y)) => Big(x.$f(y)),
                };
                result.shrink();
                result
            }
        }
    };
}

impl_arithmetic!(add, checked_add, Add);
impl_arithmetic!(sub, checked_sub, Sub);
impl_arithmetic!(mul, checked_mul, Mul);
impl_arithmetic!(div, checked_div, Div);
impl_arithmetic!(rem, checked_rem, Rem);
