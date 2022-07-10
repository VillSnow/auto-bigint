use num_bigint::ToBigInt;
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

impl ToPrimitive for AutoBigInt {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Little(little) => little.to_i64(),
            Big(big) => big.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Little(little) => little.to_u64(),
            Big(big) => big.to_u64(),
        }
    }
}

impl ToBigInt for AutoBigInt {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.to_big().into_owned())
    }
}
