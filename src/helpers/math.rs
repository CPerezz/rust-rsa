//! Math functions to build keys with trusted primes
use std::str::FromStr;
use rand::Rng;
use num_bigint::{ToBigUint, BigUint, RandBigInt, BigInt, Sign};
use num::{Zero, One, Integer};
use crate::helpers::generics::*;


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

// Posible to remove and implement it on gen big prime
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
    let (z, o, tw) = gen_basic_biguints();
    let (zero, one, two) = (&z, &o, &tw);
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
        let a = rng.gen_biguint_range(&two , &(proposal - two) );

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
    let (zero, one, _) = gen_basic_biguints();
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
  let (mut s, one, two) = gen_basic_biguints();
  let mut d = n.clone() - one.clone();

  while d.is_even() {
    d = d / two.clone();
    s = s + one.clone();
  }
  (s, d)
}

// Extended Euclidean Algorithm
// Returns gcd(a,b) and Bézout's identity coefficients
// ax + by = gcd(a,b)
pub fn egcd<'a>(a: &'a mut BigInt, b: &'a mut BigInt) -> (BigInt, BigInt, BigInt) {
    // base case
    if a.to_owned() == BigInt::from(0 as u32) {
        (b.clone(), BigInt::from(0 as i32), BigInt::from(1 as i32))
    } else {
        let mut b_mod_a = b.clone() % a.clone();
        let ref_b_mod_a = &mut b_mod_a;
        let (g, x, y) = egcd(ref_b_mod_a, a);
        let mut b_div_a = b.clone() / a.clone();
        let ref_b_div_a = &mut b_div_a;
        (g, (y - ref_b_div_a.clone() * x.clone()), x)
    }
}

// Given a fi_n, find on the interval (fi_n/2, fi_n) a number 
// that is co-prime with fi_n
pub fn found_e(fi_n: &BigUint) -> Result<BigUint, bool> {
    // Gen random number on interval
    let mut rng = rand::thread_rng();
    //Get fi_n as 
    let sign = Sign::Plus;
    let mut fi_n = BigInt::from_biguint(sign, fi_n.clone());
    let (zero, one, two) = gen_basic_bigints();
    let mut a = rng.gen_bigint_range(&(fi_n.clone()/two.clone()) , &(fi_n.clone()));
    //We want to avoid the even random numbers.
    if a.is_even() {a = a + one.clone()};
    let mut res = zero;
    println!("Starting While");
    while res != one.clone() && a <= fi_n.clone() - one.clone() {
        println!("New iteration a = {}", a);
        let (res2, _, _) = egcd(&mut fi_n, &mut a);
        println!("Res of inside egcd: {}", res2);
        res = res2;
        a = a.clone() + two.clone(); 
    }

    if res == one {
        return Ok(bigUnt_from_bigIint(&a));
    }
    Err(false)
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
    let res2 = rabin_miller(&82589933u32.to_biguint().unwrap(), 64);
    assert_eq!(res2, true);
    
    
    // Big primes
    let known_prime_str =
    "118595363679537468261258276757550704318651155601593299292198496313960907653004730006758459999825003212944725610469590674020124506249770566394260832237809252494505683255861199449482385196474342481641301503121142740933186279111209376061535491003888763334916103110474472949854230628809878558752830476310536476569";
    let known_prime: BigUint = FromStr::from_str(known_prime_str).unwrap();
    assert!(rabin_miller(&known_prime, 64));
}


#[test]
fn gen_big_prime_works() {
    let res = gen_big_prime(&2056u32, 9);
    println!("The generated prime of 1024 bits is: {}", res);
}

#[test]
fn egcd_test() {
    use num_bigint::ToBigInt;
    use std::str::FromStr;

    // small primes
    let a = &mut 179425357u32.to_bigint().unwrap();
    let b = &mut 97u32.to_bigint().unwrap();
    let (g, x, y) = egcd(a, b);
    assert_eq!(a.clone()*x + b.clone()*y, g);

    // small primes
    let a = &mut 1024u32.to_bigint().unwrap();
    let b = &mut 512u32.to_bigint().unwrap();
    let (g, x, y) = egcd(a, b);
    assert_eq!(512u32.to_bigint().unwrap(), g);

    // big primes
    let known_prime_str = "118595363679537468261258276757550704318651155601593299292198496313960907653004730006758459999825003212944725610469590674020124506249770566394260832237809252494505683255861199449482385196474342481641301503121142740933186279111209376061535491003888763334916103110474472949854230628809878558752830476310536476569";
    let known_prime_str_2 = "357111317192329313741434753596167717379838997101103107109113127131137139149151157163167173179181191193197199211223227229233239241251257263269271277281283293307311313317331337347349353359367373379383389397401409419421431433439443449457461463467479487491499503509521523541547557563569571577587593599601607613617619631641643647653659661673677683691701709719727733739743751757761769773787797809811821823827829839853857859863877881883887907911919929937941947953967971977983991997";
    let mut a: BigInt = FromStr::from_str(known_prime_str).unwrap();
    let mut b: BigInt = FromStr::from_str(known_prime_str_2).unwrap();
    let a_r = &mut a;
    let b_r = &mut b;
    let (g, x, y) = egcd(a_r, b_r);
    assert_eq!(a_r.clone()*x + b_r.clone()*y, g);
}