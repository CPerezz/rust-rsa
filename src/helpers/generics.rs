//! Generic helper functions.
use num_bigint::{BigUint, BigInt};
use num::{Zero, One, Signed};
use std::str::FromStr;

/// Generates 0, 1 and 2 numbers as BigUint
pub fn gen_basic_biguints() -> (BigUint, BigUint, BigUint) {
    (BigUint::zero(), BigUint::one(), BigUint::one() + BigUint::one())
}

/// Generates 0, 1 and 2 numbers as BigUint
pub fn gen_basic_bigints() -> (BigInt, BigInt, BigInt) {
    (BigInt::zero(), BigInt::one(), BigInt::one() + BigInt::one())
}

/// Generate a BigUint from a positive BigInt
pub fn biguint_from_bigint(a: &BigInt) -> Result<BigUint, &'static str> {
    if a.is_negative() {return Err("Error converting a negative BigInt to a BigUint")}
    let boxed = format!("{}", a.clone()).into_boxed_str();
    let biguint_str = Box::leak(boxed);
    Ok(BigUint::from_str(biguint_str).unwrap())
}
