use std::fmt::*;

use crate::AutoBigInt;
use AutoBigInt::{Big, Little};

impl Display for AutoBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Little(x) => Display::fmt(x, f),
            Big(x) => Display::fmt(x, f),
        }
    }
}

impl Binary for AutoBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Little(x) => Binary::fmt(x, f),
            Big(x) => Binary::fmt(x, f),
        }
    }
}

impl Octal for AutoBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Little(x) => Octal::fmt(x, f),
            Big(x) => Octal::fmt(x, f),
        }
    }
}

impl LowerHex for AutoBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Little(x) => LowerHex::fmt(x, f),
            Big(x) => LowerHex::fmt(x, f),
        }
    }
}

impl UpperHex for AutoBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Little(x) => UpperHex::fmt(x, f),
            Big(x) => UpperHex::fmt(x, f),
        }
    }
}
