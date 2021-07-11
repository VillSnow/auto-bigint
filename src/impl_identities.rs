use num_traits::*;

use crate::AutoBigInt;
use AutoBigInt::Little;

impl Zero for AutoBigInt {
    fn zero() -> AutoBigInt {
        Little(0)
    }

    fn is_zero(&self) -> bool {
        *self == zero()
    }
}

impl One for AutoBigInt {
    fn one() -> AutoBigInt {
        Little(1)
    }

    fn is_one(&self) -> bool {
        *self == one()
    }
}
