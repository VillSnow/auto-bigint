use num_bigint::BigInt;

use crate::AutoBigInt;
use crate::LittleInt;
use AutoBigInt::{Big, Little};

impl From<i8> for AutoBigInt {
    fn from(x: i8) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<u8> for AutoBigInt {
    fn from(x: u8) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<i16> for AutoBigInt {
    fn from(x: i16) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<u16> for AutoBigInt {
    fn from(x: u16) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<i32> for AutoBigInt {
    fn from(x: i32) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<u32> for AutoBigInt {
    fn from(x: u32) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<i64> for AutoBigInt {
    fn from(x: i64) -> AutoBigInt {
        Little(x as LittleInt)
    }
}

impl From<u64> for AutoBigInt {
    fn from(x: u64) -> AutoBigInt {
        let mut result = Big(BigInt::from(x));
        result.shrink();
        result
    }
}
