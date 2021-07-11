use num_traits::*;

use crate::AutoBigInt;
use AutoBigInt::{Big, Little};

impl Signed for AutoBigInt {
    fn abs(&self) -> AutoBigInt {
        match self {
            &Little(x) => x.abs().into(),
            Big(x) => x.abs().into(),
        }
    }

    fn abs_sub(&self, other: &AutoBigInt) -> AutoBigInt {
        if self < other {
            zero()
        } else {
            self - other
        }
    }

    fn signum(&self) -> AutoBigInt {
        if self.is_positive() {
            Little(1)
        } else if self.is_negative() {
            Little(-1)
        } else {
            Little(0)
        }
    }

    fn is_positive(&self) -> bool {
        self > &AutoBigInt::zero()
    }

    fn is_negative(&self) -> bool {
        self < &AutoBigInt::zero()
    }
}
