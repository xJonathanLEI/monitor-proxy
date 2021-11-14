use bigdecimal::BigDecimal;
use ethers::prelude::*;
use num_bigint::{BigInt, Sign};
use rust_decimal::Decimal;

pub fn u256_to_bigdecimal(u256: &U256, decimals: i64) -> BigDecimal {
    BigDecimal::new(u256_to_bigint(u256), decimals)
}

pub fn decimal_to_bigdecimal(dec: &Decimal) -> BigDecimal {
    let unpacked_dec = dec.unpack();

    let mut bytes: [u8; 12] = [0; 12];
    bytes[0..4].copy_from_slice(&unpacked_dec.hi.to_be_bytes());
    bytes[4..8].copy_from_slice(&unpacked_dec.mid.to_be_bytes());
    bytes[8..12].copy_from_slice(&unpacked_dec.lo.to_be_bytes());

    BigDecimal::new(
        BigInt::from_bytes_be(
            if unpacked_dec.negative {
                Sign::Minus
            } else {
                Sign::Plus
            },
            &bytes,
        ),
        unpacked_dec.scale.into(),
    )
}

fn u256_to_bigint(u256: &U256) -> BigInt {
    let mut bytes: [u8; 32] = [0; 32];

    for (ind_item, item) in u256.0.iter().rev().enumerate() {
        let item_bytes: [u8; 8] = item.to_be_bytes();
        bytes[(ind_item * 8)..((ind_item + 1) * 8)].copy_from_slice(&item_bytes);
    }

    BigInt::from_bytes_be(Sign::Plus, &bytes)
}
