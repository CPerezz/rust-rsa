use num_bigint::{BigUint};
use crate::helpers::math::*;

pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
    pub size: u32
}

pub struct SecretKey {
    n: BigUint,
    d: BigUint
}

pub struct PublicKey {
    n: BigUint,
    e: BigUint
}

pub struct Threshold {
    value: u32
}

impl Threshold {
    // Creates a Threshold with a default error probability of generating a prime of 4^-64
    pub fn default() -> Self {
        let threshold = Threshold {
            value: 64 as u32
        };
        threshold
    }
}

/*
impl KeyPair {
    fn new(size: &u32, threshold: &Threshold) -> Result<Self, &'static str> {
        let p = gen_big_prime(size, threshold.value);
        let q = gen_big_prime(size, threshold.value);
        let n = ecc(&p, &q);
    }
}
*/

/*
impl PublicKey {
    pub fn new(size: &u32) -> Result<Self, &'static str> {
        
    }
}
*/