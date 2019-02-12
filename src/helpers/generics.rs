use num_bigint::{BigUint, BigInt};
use num::{Zero, One};
use std::str::FromStr;

pub fn gen_basic_biguints() -> (BigUint, BigUint, BigUint) {
    (BigUint::zero(), BigUint::one(), BigUint::one() + BigUint::one())
}

pub fn gen_basic_bigints() -> (BigInt, BigInt, BigInt) {
    (BigInt::zero(), BigInt::one(), BigInt::one() + BigInt::one())
}

pub fn bigUnt_from_bigIint(a: &BigInt) -> BigUint {
    let boxed = format!("{}", a.clone()).into_boxed_str();
    let biguint_str = Box::leak(boxed);
    BigUint::from_str(biguint_str).unwrap()
}