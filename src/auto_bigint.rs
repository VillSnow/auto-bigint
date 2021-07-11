use std::borrow::Cow;
use std::str::FromStr;

use num_bigint::ParseBigIntError;
use num_traits::ToPrimitive;

use AutoBigInt::{Big, Little};

pub type LittleInt = i64;
pub type BigInt = num_bigint::BigInt;

#[derive(Debug, Clone, Hash)]
pub enum AutoBigInt {
    Little(LittleInt),
    Big(BigInt),
}

impl AutoBigInt {
    pub fn shrink(&mut self) {
        if let Big(large) = self {
            if let Some(little) = large.to_i64() {
                *self = Little(little)
            }
        }
    }

    pub fn is_little(&self) -> bool {
        self.little().is_some()
    }

    pub fn is_big(&self) -> bool {
        self.big().is_some()
    }

    pub fn little(&self) -> Option<LittleInt> {
        if let &Little(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn big(&self) -> Option<&BigInt> {
        if let Big(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn big_mut(&mut self) -> Option<&mut BigInt> {
        if let Big(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn into_big(self) -> BigInt {
        match self {
            Little(x) => BigInt::from(x),
            Big(x) => x,
        }
    }

    pub fn to_big(&self) -> Cow<BigInt> {
        match self {
            &Little(x) => Cow::Owned(BigInt::from(x)),
            Big(x) => Cow::Borrowed(x),
        }
    }
}

impl From<BigInt> for AutoBigInt {
    fn from(x: BigInt) -> AutoBigInt {
        Big(x)
    }
}

impl FromStr for AutoBigInt {
    type Err = ParseBigIntError;
    fn from_str(s: &str) -> Result<AutoBigInt, ParseBigIntError> {
        let large = BigInt::from_str(s)?;
        Ok(Big(large))
    }
}

impl Default for AutoBigInt {
    #[inline]
    fn default() -> AutoBigInt {
        Little(0)
    }
}
