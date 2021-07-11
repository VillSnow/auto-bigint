mod auto_bigint;
mod impl_arithmetic_assigns;
mod impl_arithmetics;
mod impl_cast;
mod impl_cmp;
mod impl_fmt;
mod impl_from_primint;
mod impl_identities;
mod impl_num_traits;
mod impl_ops;
mod impl_sign;

pub use auto_bigint::AutoBigInt;
pub use auto_bigint::BigInt;
pub use auto_bigint::LittleInt;

#[cfg(feature = "num")]
mod impl_integer;

mod tests {
    #![allow(unused_imports)]
    use super::AutoBigInt;
    use AutoBigInt::{Big, Little};

    #[test]
    pub fn test_add() {
        assert_eq!(Little(12) + Little(34), Little(46));
        assert_eq!(Little(12) + &Little(34), Little(46));
        assert_eq!(&Little(12) + Little(34), Little(46));
        assert_eq!(&Little(12) + &Little(34), Little(46));
    }
}
