//! Generic helper functions.
use crate::types::*;
use num_bigint::{BigUint, BigInt};
use num::{Zero, One, Signed};
use std::str::FromStr;
use base64::*;
use std::str::from_utf8;
use std::fs::File;
use std::io::{BufRead, BufReader};


/// Formats a BigUint ready to be written on a file.
macro_rules! encode_to_print {
    ($big_num: expr) => {
        encode(&$big_num.to_radix_be(16u32)).as_bytes()
    };
}

#[cfg(test)]
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

// Format Keypair to print it on a file.
pub fn prepare_to_print(kp: &KeyPair) -> Result<(String, String), &'static str> {
    let (mut encoded_pk, mut encoded_sk) = (String::new(), String::new());
    // Encoding Public Key
    encoded_pk.push_str("---------- BEGIN RSA PUBLIC KEY ----------");
    encoded_pk.push_str("\n");
    encoded_pk.push_str(from_utf8(encode_to_print!(&kp.pk.n)).unwrap());
    encoded_pk.push_str("\n");
    encoded_pk.push_str(from_utf8(encode_to_print!(&kp.pk.e)).unwrap());
    encoded_pk.push_str("\n");
    encoded_pk.push_str("----------- END RSA PUBLIC KEY -----------");
    encoded_pk.push_str("\n");
    encoded_pk.push_str(&kp.size.to_string());
    encoded_pk.push_str("\n");
    encoded_pk.push_str(&kp.threshold.to_string());
    
    // Encoding Secret Key
    encoded_sk.push_str("---------- BEGIN RSA PRIVATE KEY ----------");
    encoded_sk.push_str("\n");
    encoded_sk.push_str(from_utf8(encode_to_print!(&kp.sk.n)).unwrap());
    encoded_sk.push_str("\n");
    encoded_sk.push_str(from_utf8(encode_to_print!(&kp.sk.d)).unwrap());
    encoded_sk.push_str("\n");
    encoded_sk.push_str("----------- END RSA PRIVATE KEY -----------");
    encoded_sk.push_str("\n");
    encoded_sk.push_str(&kp.size.to_string());
    encoded_sk.push_str("\n");
    encoded_sk.push_str(&kp.threshold.to_string());
    Ok((encoded_pk, encoded_sk))
}

/// Gets a path to rsa keys and completes it.
pub fn get_full_path(path: &String) -> (String, String) {
    let mut full_pk_path = String::new();
    let mut full_sk_path = String::new();
    full_pk_path.push_str(path);
    full_pk_path.push_str("/rsa_pk.key");
    full_sk_path.push_str(path);
    full_sk_path.push_str("/rsa_sk.key");
    (full_pk_path, full_sk_path)
}

/// Gets Public Key params from the Pk file.
/// Returns a Public Key Struct or an Error.
pub fn get_pk_params(pk_file: &File) -> Result<PublicKey, &'static str> {
    let mut lines = vec!();
    for line in BufReader::new(pk_file).lines() {
        lines.push(line)
    }
    let pk = PublicKey::new(
    &BigUint::from_radix_be(from_utf8(&base64::decode(&lines.remove(1).unwrap()).unwrap()).unwrap().as_bytes(), 16u32).unwrap(),
    &BigUint::from_radix_be(from_utf8(&base64::decode(&lines.remove(1).unwrap()).unwrap()).unwrap().as_bytes(), 16u32).unwrap()).unwrap();
    Ok(pk)
}


/// Gets ecret Key params from the Pk file.
/// Returns a ecret Key Struct or an Error.
pub fn get_sk_params(sk_file: &File) -> Result<SecretKey, &'static str> {
    let mut lines = vec!();
    for line in BufReader::new(sk_file).lines() {
        lines.push(line)
    }
    let sk = SecretKey::new(
    &BigUint::from_radix_be(from_utf8(&base64::decode(&lines.remove(1).unwrap()).unwrap()).unwrap().as_bytes(), 16u32).unwrap(),
    &BigUint::from_radix_be(from_utf8(&base64::decode(&lines.remove(1).unwrap()).unwrap()).unwrap().as_bytes(), 16u32).unwrap()).unwrap();
    println!("{}", sk);
    Ok(sk)
}
