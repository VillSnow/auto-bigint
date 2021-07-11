use num_bigint::BigInt;
use std::cmp::*;

use crate::AutoBigInt;
use AutoBigInt::{Big, Little};

impl PartialEq for AutoBigInt {
    fn eq(&self, other: &AutoBigInt) -> bool {
        match (self, other) {
            (Little(x), Little(y)) => x.eq(y),
            (Little(x), Big(y)) => BigInt::from(*x).eq(y),
            (Big(x), Little(y)) => x.eq(&BigInt::from(*y)),
            (Big(x), Big(y)) => x.eq(y),
        }
    }
}

impl Eq for AutoBigInt {}

impl PartialOrd for AutoBigInt {
    #[inline]
    fn partial_cmp(&self, other: &AutoBigInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AutoBigInt {
    fn cmp(&self, other: &AutoBigInt) -> Ordering {
        match (self, other) {
            (Little(x), Little(y)) => x.cmp(y),
            (Little(x), Big(y)) => BigInt::from(*x).cmp(y),
            (Big(x), Little(y)) => x.cmp(&BigInt::from(*y)),
            (Big(x), Big(y)) => x.cmp(y),
        }
    }
}
