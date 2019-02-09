//! Math functions to build keys with trusted primes
use rand;
use num_bigint::{ToBigUint, BigUint, RandBigInt};
use num::{Zero, One, Integer};

// Generates a big number of lenght = u32 param.
pub fn gen_big_num(bit_len: &u32) -> BigUint {
    // RNG depends on rng_core crate.
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(bit_len.to_owned() as usize);
    a
}

// Given lenght, generates a prime number of that lenght approximately. 
pub fn gen_big_prime(size: &u32) -> BigUint {
    let mut proposal =  gen_big_num(size);
    // Remove all even numbers to reduce the iterations a half.
    if proposal.is_even() {
        proposal = proposal + BigUint::one();
    }
    while !is_prime(&proposal) {
        // Steps of 2 to avoid the even numbers on the iterations.
        proposal =  proposal + 2.to_biguint().unwrap();
    }
    proposal
}

// Given a prime proposal, compute Rabin Miller's algorithm.
fn is_prime(proposal: &BigUint) -> bool {
    if !rabin_miller(proposal) {return false}
    true
}

// Rabin-Miller is a probabilistic algorithm that checks if a number is prime based on Riemmann's conjecture.
fn rabin_miller(proposal: &BigUint) -> bool {



    true
}

// Modular exponentiation squaring
fn mod_pow(base: &BigUint, exp: &BigUint, md: &BigUint) -> BigUint {
    let mut res = BigUint::one();
    let (zero, one) = (BigUint::zero(), BigUint::one());
    let (mut base, mut exponent) = (base.clone(), exp.clone());

    while exponent > zero {
        if exponent.clone() & one.clone() > zero {
            res = (res * base.clone()) % md;
        }
        exponent >>= 1;
        base = (base.clone() * base.clone()) % md;
    }
    res
}


#[test]
fn generates_random_biguint() {
    let a = gen_big_num(&1024);
    assert_ne!(a, BigUint::zero());
}

#[test]
fn mod_exp_works() {
    let res = mod_pow(&BigUint::from(4 as u32), &BigUint::from(13 as u32), &BigUint::from(497 as u32));
    assert_eq!(res, BigUint::from(445 as u32));

    let res2 = mod_pow(&BigUint::from(5 as u32), &BigUint::from(3 as u32), &BigUint::from(13 as u32));
    assert_eq!(res2, BigUint::from(8 as u32));
}