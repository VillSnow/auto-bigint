use num::Integer;

use crate::AutoBigInt;
use AutoBigInt::{Big, Little};

impl Integer for AutoBigInt {
    fn div_floor(&self, other: &Self) -> Self {
        if let (Little(x), Little(y)) = (self, other) {
            Little(x.div_floor(y))
        } else {
            let mut result = Big(self.to_big().div_floor(&other.to_big()));
            result.shrink();
            result
        }
    }

    fn mod_floor(&self, other: &Self) -> Self {
        if let (Little(x), Little(y)) = (self, other) {
            Little(x.mod_floor(y))
        } else {
            let mut result = Big(self.to_big().mod_floor(&other.to_big()));
            result.shrink();
            result
        }
    }

    fn gcd(&self, other: &Self) -> Self {
        let mut result = Big(self.to_big().gcd(&other.to_big()));
        result.shrink();
        result
    }

    fn lcm(&self, other: &Self) -> Self {
        let mut result = Big(self.to_big().lcm(&other.to_big()));
        result.shrink();
        result
    }

    fn divides(&self, other: &Self) -> bool {
        if let (Little(x), Little(y)) = (self, other) {
            x.divides(y)
        } else {
            self.to_big().divides(&other.to_big())
        }
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        if let (Little(x), Little(y)) = (self, other) {
            x.is_multiple_of(y)
        } else {
            self.to_big().is_multiple_of(&other.to_big())
        }
    }

    fn is_even(&self) -> bool {
        match self {
            &Little(x) => x.is_even(),
            Big(x) => x.is_even(),
        }
    }

    fn is_odd(&self) -> bool {
        match self {
            &Little(x) => x.is_odd(),
            Big(x) => x.is_odd(),
        }
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (a, b) = self.to_big().div_rem(&other.to_big());
        let mut a = Big(a);
        let mut b = Big(b);
        a.shrink();
        b.shrink();
        (a, b)
    }
}
