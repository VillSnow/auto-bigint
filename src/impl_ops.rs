use std::ops::*;

use crate::AutoBigInt;
use AutoBigInt::{Big, Little};

impl Not for AutoBigInt {
    type Output = AutoBigInt;

    fn not(self) -> AutoBigInt {
        match self {
            Little(x) => Little(x.not()),
            Big(x) => Big(x.not()),
        }
    }
}

impl<'a> Not for &'a AutoBigInt {
    type Output = AutoBigInt;

    fn not(self) -> AutoBigInt {
        match self {
            Little(x) => Little(x.not()),
            Big(x) => Big(x.not()),
        }
    }
}

impl Neg for AutoBigInt {
    type Output = AutoBigInt;

    fn neg(self) -> AutoBigInt {
        match self {
            Little(x) => Little(x.neg()),
            Big(x) => Big(x.neg()),
        }
    }
}

impl<'a> Neg for &'a AutoBigInt {
    type Output = AutoBigInt;

    fn neg(self) -> AutoBigInt {
        match self {
            Little(x) => Little(x.neg()),
            Big(x) => Big(x.neg()),
        }
    }
}
