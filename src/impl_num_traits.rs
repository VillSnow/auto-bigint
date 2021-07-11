use num_traits::*;

use crate::{AutoBigInt, BigInt, LittleInt};
use AutoBigInt::{Big, Little};

impl Num for AutoBigInt {
    type FromStrRadixErr = <BigInt as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<AutoBigInt, Self::FromStrRadixErr> {
        LittleInt::from_str_radix(str, radix)
            .map(Little)
            .or_else(|_| BigInt::from_str_radix(str, radix).map(Big))
    }
}
