use num_bigint::{BigUint, BigInt, ToBigInt, Sign};
use crate::helpers::math::*;
use crate::helpers::generics::*;
use num::One;

pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
    pub size: u32,
    pub threshold: u32
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


impl KeyPair {
    // Generate a new KeyPair Struct from scratch by giving the size of the key desired (in bits) and the threshold of P(err) while assuming that
    // a number is prime. Statistic methods are used to found that numbers. P(err) = 4^-threshold (As is demonstraded on the Rabin-Miller algorithm)
    pub fn new(size: &u32, threshold: &Threshold) -> Result<Self, &'static str> {
        // Gen basic needed variables
        let (_, one, _) = gen_basic_biguints();
        // Gen p q primal base
        let p = gen_big_prime(size, threshold.value);
        let q = gen_big_prime(size, threshold.value);
        // Gen n and fi_n
        let n = &p * &q;
        let fi_n = (&p - &one) * (&q - &one);
        // Find a positive integer minor than fi_n , co-prime with fi_n 
        let e = found_e(&fi_n).unwrap();

        // Building Pk Struct
        let pk = PublicKey::new(&n, &e).unwrap();
        // Finding d and building Secret Key Struct
        let (_, emuld, _) = egcd(&mut fi_n.to_bigint().unwrap(), &mut e.to_bigint().unwrap());
        let d = emuld / e.to_bigint().unwrap();
        let sk = SecretKey::new(&n, &bigUnt_from_bigIint(&d)).unwrap();

        //Building KeyPair struct
        let kp = KeyPair {
            pk: pk,
            sk: sk,
            size: size.to_owned(),
            threshold: threshold.value.to_owned()
        };
        // Return the KeyPair struct
        Ok(kp)
    }
}



impl PublicKey {
    // Generate a PublicKey struct from n and d co-prime factors.
    fn new(_n: &BigUint, _e: &BigUint) -> Result<Self, &'static str> {
        Ok(PublicKey {
            n: _n.to_owned(),
            e: _e.to_owned()
        })
    }
    // Generate a PublicKey struct from n, fi_n and d params with the co-prime property checking.
    pub fn new_from_fi_n_e(_n: &BigUint, _fi_n: &BigUint, _e: &BigUint) -> Result<Self, &'static str> {
        let (_, _one, _) = gen_basic_bigints();

        match egcd(&mut BigInt::from_biguint(Sign::Plus, _fi_n.to_owned()), &mut BigInt::from_biguint(Sign::Plus, _e.to_owned())) {
            (possible_one, _, _) => {
                if possible_one.is_one() {
                    return  Ok(PublicKey {
                                n: _n.to_owned(),
                                e: _e.to_owned()
                            }
                        )
                }else {
                    return Err("Params passed to Sk builder haven't the properties to be a Public Key")
            
                }            
            }
        }
    }
}

impl SecretKey {
    // Generate a SecretKey struct from n and d co-prime factors.
    fn new(_n: &BigUint, _e: &BigUint) -> Result<Self, &'static str> {
        Ok(SecretKey {
            n: _n.to_owned(),
            d: _e.to_owned()
        })
    }

    // Generate a SecretKey struct from n, fi_n and d params with the co-prime property checking.
    pub fn new_from_fi_n_e(_n: &BigUint, _fi_n: &BigUint, _d: &BigUint) -> Result<Self, &'static str> {
        let (_, _one, _) = gen_basic_bigints();

        match egcd(&mut BigInt::from_biguint(Sign::Plus, _fi_n.to_owned()), &mut BigInt::from_biguint(Sign::Plus, _d.to_owned())) {
            (possible_one, _, _) => {
                if possible_one.is_one() {
                    return  Ok(SecretKey {
                                n: _n.to_owned(),
                                d: _d.to_owned()
                            }
                    )
                }else {
                    return Err("Params passed to Sk builder haven't the properties to be a Public Key")
            
                }            
            }
        }
    }
}
