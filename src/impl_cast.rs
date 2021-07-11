use num_traits::cast::*;

use crate::{AutoBigInt, BigInt, LittleInt};
use AutoBigInt::{Big, Little};

impl FromPrimitive for AutoBigInt {
    fn from_i64(n: i64) -> Option<Self> {
        LittleInt::from_i64(n)
            .map(Little)
            .or_else(|| BigInt::from_i64(n).map(Big))
    }

    fn from_u64(n: u64) -> Option<Self> {
        LittleInt::from_u64(n)
            .map(Little)
            .or_else(|| BigInt::from_u64(n).map(Big))
    }
}
