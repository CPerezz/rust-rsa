//! Math functions to build keys with trusted primes
use std::str::FromStr;
use rand::Rng;
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
// That prime number is prime with probability = 4^-threshold 
pub fn gen_big_prime(size: &u32, threshold: u32) -> BigUint {
    let mut proposal =  gen_big_num(size);
    // Remove all even numbers to reduce the iterations a half.
    if proposal.is_even() {
        proposal = proposal + BigUint::one();
    }
    while !is_prime(&proposal, threshold) {
        // Steps of 2 to avoid the even numbers on the iterations.
        proposal =  proposal + 2.to_biguint().unwrap();
    }
    proposal
}

// Given a prime proposal, compute Rabin Miller's algorithm.
fn is_prime(proposal: &BigUint, threshold: u32) -> bool {
    if !rabin_miller(proposal, threshold) {return false}
    true
}

// Rabin-Miller is a probabilistic algorithm that checks if a number is prime based on Riemmann's conjecture.
// Implemented from psoudocode found on: https://en.wikibooks.org/wiki/Algorithm_Implementation/Mathematics/Primality_Testing 
// The function recieves a prime proposal and the threshold probability of a false positive
// due to composite numbers reported as primes.
// The pobability of a false positive is 4^-threshold. With t=9 => P(false_positive) = 3/1_000_000 
fn rabin_miller(proposal: &BigUint, t: u32) -> bool {
    // Needed constants
    let (zero, one, two) = (&BigUint::zero(), &BigUint::from(1 as u32), &BigUint::from(2 as u32));
    
    // If proposal <= 1 Rabin-Miller has to fail.
    if proposal.clone() <= one.to_owned() {return false};
    // If proposal != 2 and modulus 2 = 0, Rabin-Miller fails.
    if proposal.clone() != two.to_owned() && proposal.clone() % two == zero.to_owned() {return false};
    // Getting exp to execute mulmod.
    let (s,d) = refactor(proposal);

    let mut counter = 0;
    while counter < t {
        // Gen rand biguint from a range (2, proposal-2)
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(two , &(proposal - two) );

        let mut x = mod_exp_pow(&a, &d, proposal);
        if x != one.to_owned() && x != proposal.to_owned() - one {
            let mut i = zero.clone();
            loop {
                x = mod_exp_pow(&x, &two, proposal);
                if x == proposal.to_owned() - one {break;}
                if x == one.to_owned() || i >= s.clone()- one{return false;};
                
                i = i.clone() + one;
            }
        }
        counter +=2;
    }  
    true
}

// Modular exponentiation implemented on binary exponentiation (squaring)
fn mod_exp_pow(base: &BigUint, exp: &BigUint, md: &BigUint) -> BigUint {
    let mut res = BigUint::one();
    let (zero, one) = (BigUint::zero(), BigUint::one());
    let (mut base, mut exponent) = (base.clone(), exp.clone());

    while exponent > zero {
        if exponent.clone() & one.clone() > zero {
            res = (res * base.clone()) % md;
        }
        // Shifting 1 bit of the exponent as a binary number.
        exponent >>= 1;
        // Generating next base by squaring
        base = (base.clone() * base.clone()) % md;
    }
    res
}

// Given a number n, write n − 1 as 2s·d with d odd by factoring powers of 2 from n − 1
fn refactor(n: &BigUint) -> (BigUint, BigUint) {
  let mut s: BigUint = Zero::zero();
  let one: BigUint = One::one();
  let two = one.clone() + one.clone();
  let mut d = n.clone() - one.clone();

  while d.is_even() {
    d = d / two.clone();
    s = s + one.clone();
  }
  (s, d)
}


#[test]
fn generates_random_biguint() {
    let a = gen_big_num(&1024);
    assert_ne!(a, BigUint::zero());
}

#[test]
fn mod_exp_works() {
    let res = mod_exp_pow(&BigUint::from(4 as u32), &BigUint::from(13 as u32), &BigUint::from(497 as u32));
    assert_eq!(res, BigUint::from(445 as u32));

    let res2 = mod_exp_pow(&BigUint::from(5 as u32), &BigUint::from(3 as u32), &BigUint::from(13 as u32));
    assert_eq!(res2, BigUint::from(8 as u32));
}


#[test]
fn rabin_miller_works() {
    //Small primes
    let res = rabin_miller(&179425357u32.to_biguint().unwrap(), 9);
    assert_eq!(res, true);
    let res2 = rabin_miller(&97u32.to_biguint().unwrap(), 64);
    assert_eq!(res2, true);
    
    
    // Big primes
    let known_prime_str =
    "118595363679537468261258276757550704318651155601593299292198496313960907653004730006758459999825003212944725610469590674020124506249770566394260832237809252494505683255861199449482385196474342481641301503121142740933186279111209376061535491003888763334916103110474472949854230628809878558752830476310536476569";
    let known_prime: BigUint = FromStr::from_str(known_prime_str).unwrap();
    assert!(rabin_miller(&known_prime, 64));
}


#[test]
fn gen_big_prime_works() {
    let res = gen_big_prime(&1024u32, 64);
    println!("The generated prime of 1024 bits is: {}", res);
}