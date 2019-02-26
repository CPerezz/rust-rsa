//! Generic helper functions.
use crate::types::*;
use num_bigint::{BigUint, BigInt};
use num::{Zero, One, Signed};
use std::str::FromStr;
use base64::*;



macro_rules! encode_to_print {
    ($big_num: expr) => {
        encode(&$big_num.to_radix_be(16u32)).as_bytes()
    };
}

#[cfg(test)]
use std::str::from_utf8;
#[test]
fn encodes_to_print() {
    let known_prime_str =
    "118595363679537468261258276757550704318651155601593299292198496313960907653004730006758459999825003212944725610469590674020124506249770566394260832237809252494505683255861199449482385196474342481641301503121142740933186279111209376061535491003888763334916103110474472949854230628809878558752830476310536476569";
    let known_prime: BigUint = FromStr::from_str(known_prime_str).unwrap();
    let expected = "CggOAgoJDQMBBAMEAgwKAwgABAwLBwAIDgEPBQYMDAEDDQ0BCQ0HDgoJDA0EBwMKCwkBBgQOAA4CAgsABAgOCwwDAwMDAQYLBgMBDwIIAwUAAwsEAwgGCg0BAwcFCwsBCgsKCQkACA0NDwYCCAgKDwALCgUODQcDCgoCAQYNCgsDCgAGCgoKAAQDCgcFBgsJCAADBw8KCAUACA4GCQgEDQsNBwkPCQcKDw4MDgQBCAICCwEEDwEBCggPBwEEDQMMBAMCCAgPBgQDDAQJCgwBCAMIAAkCDAUAAwIJDwsECAgOCwwGBgQGBQELCgoEAwINDAoPDAUDDA0DCAYHBQcJCQ==";
    assert_eq!(expected, from_utf8(encode_to_print!(&known_prime)).unwrap());
    
}


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

pub fn print(kp: &KeyPair) -> Result<String, &'static str> {
    let mut pk_string = String::new();
    let res1 = encode_to_print!(&kp.pk.e);

    unimplemented!()
}