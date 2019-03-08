//! Type definitions
use num_bigint::{BigUint, BigInt, ToBigInt, Sign};
use crate::helpers::math::*;
use crate::helpers::generics::*;
use num::{Signed, One, Num};
use std::fmt;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;


#[derive(Clone, PartialEq)]
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
    pub size: u32,
    pub threshold: u32
}

#[derive(Clone, PartialEq)]
pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint
}

#[derive(Clone, PartialEq)]
pub struct SecretKey {
    pub n: BigUint,
    pub d: BigUint
}

#[derive(Clone, Copy, PartialEq)]
pub struct Threshold {
    value: u32
}

impl From<u32> for Threshold {
    /// Implement from<u32> for Threshold
    fn from(val: u32) -> Self {
        Threshold {
            value: val
        }
    }
}

impl Default for Threshold {
    /// Creates a Threshold with a default error probability of generating a prime of 4^-64
    fn default() -> Self {
        let threshold = Threshold {
            value: 9 as u32
        };
        threshold
    }
}

impl Threshold {
    /// Creates a Threshold with a selected value as thresholf of P(err). P(err prime) = 4^-threshold. 
    pub fn new(th: &u32) -> Self {
        let th = Threshold {
            value: *th
        };
        th
    }

    /// Gets the value of a Threshold and returns it as u32.
    pub fn value(th: Self) -> u32 {
        th.value
    }
}


/// Implementation of Display for KeyPair Struct.
impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nPublic Key: \n{}\nSecret Key: \n{}\nSize: {}\nThreshold: {} which gives a P(err_primes_gen) = 4^(-{})", self.pk, self.sk, self.size, self.threshold, self.threshold)
    }
}

impl KeyPair {
    /// Generate a new KeyPair Struct from scratch by giving the size of the key desired (in bits) and the threshold of P(err) while assuming that
    /// a number is prime. Statistic methods are used to found that numbers. P(err) = 4^-threshold (As is demonstraded on the Rabin-Miller algorithm)
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
        let e = find_e(&fi_n).unwrap();

        // Building Pk Struct
        let pk = PublicKey::new(&n, &e).unwrap();
        // Finding d and building Secret Key Struct
        let (_, _,mut d) = egcd(&mut fi_n.to_bigint().unwrap(), &mut e.to_bigint().unwrap());
        while d.is_negative() {
            d = d + BigInt::from_biguint(Sign::Plus, fi_n.clone());
        }
        let sk = SecretKey::new(&n, &biguint_from_bigint(&d).unwrap()).unwrap();
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

    /// Saves the KeyPair on two separated documents on the project folder encoded as base64.
    pub fn print(&self) -> Result<(), &'static str> {
        let mut pk_file = File::create("rsa_pk.key").unwrap();
        let mut sk_file = File::create("rsa_sk.key").unwrap();
        //Ask for encoded params and write.
        let (pk, sk) = prepare_to_print(&self).unwrap();
        pk_file.write_all(pk.as_bytes()).unwrap();
        sk_file.write_all(sk.as_bytes()).unwrap();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn prints_kp() {
    let kp = KeyPair::new(&512u32, &Threshold::default()).unwrap();
    kp.print().unwrap();
}



/// Implementation of Display for KeyPair Struct.
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "n: {}\ne: {}", self.n, self.e)
    }
}

/// Allow to get a Public Key from different ways.
impl From<&str> for PublicKey {
    /// Generate a Public Key from it's Keys path.
    fn from(path: &str) -> Self {
        let (pk_path, _) = get_full_path(&String::from_str(path).unwrap());
        let _pk_file = match File::open(&pk_path) {
            Ok(res) => {
                let a = get_pk_params(&res);
            },
            Err(_) => panic!("Failed to load Public Key from path: {}", pk_path)
        };
        unimplemented!()
    }
}
#[cfg(test)]
#[test]
fn gets_pk_from_path() {
    let pk = PublicKey::from(".");
}   

impl PublicKey {
    /// Generate a PublicKey struct from n and d co-prime factors.
    pub fn new(_n: &BigUint, _e: &BigUint) -> Result<Self, &'static str> {
        Ok(PublicKey {
            n: _n.to_owned(),
            e: _e.to_owned()
        })
    }
    /// Generate a PublicKey struct from n, fi_n and d params with the co-prime property checking.
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
                    return Err("Params passed to Sk builder haven't the right properties to be a Public Key")
            
                }            
            }
        }
    }
    /// Encrypts the data passed on the params.
    pub fn encrypt(&self, msg: &str) -> Result<String, &'static str> {
        if !msg.is_ascii(){
            return Err("Message isn't ASCII like. Please remove non-ASCII characters.")
        }else{
            let res = BigUint::from_bytes_be(msg.as_bytes());
            Ok(format!("{}", mod_exp_pow(&res, &self.e, &self.n).to_str_radix(16u32)))
        }
    }
}


/// Implementation of Display for KeyPair Struct.
impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "n: {}\nd: {}", self.n, self.d)
    }
}

/// Allow to get a Secret Key from different ways.
impl From<&str> for SecretKey {
    /// Generate a Secret Key from it's Keys folder path.
    fn from(path: &str) -> Self {
        let (_, sk_path) = get_full_path(&String::from_str(path).unwrap());
        let sk_file = match File::open(&sk_path) {
            Ok(res) => res,
            Err(_) => panic!("Failed to load Secret Key from path: {}", sk_path)
        };
        

        unimplemented!()
    }
}
/*
impl From<&'a BigUint> <&'b BigUint> for SecretKey {

    fn from(fi_n: &BigUint, d: &BigUint) -> Self {
        unimplemented!()
    }
}
*/
impl SecretKey {
    /// Generate a SecretKey struct from n and d co-prime factors.
    fn new(_n: &BigUint, _e: &BigUint) -> Result<Self, &'static str> {
        Ok(SecretKey {
            n: _n.to_owned(),
            d: _e.to_owned()
        })
    }

    /// Generate a SecretKey struct from n, fi_n and d params with the co-prime property checking.
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
    
    /// Decrypts the cyphertext giving back an &str
    pub fn decrypt(&self, text: &String) -> Result<String, &'static str> {
        let c = BigUint::from_str_radix(&text, 16u32).unwrap();
        let result_as_bytes = mod_exp_pow(&c, &self.d, &self.n).to_bytes_be();
        let res_decrypt = std::str::from_utf8(&result_as_bytes).unwrap();
        Ok(format!("{}", res_decrypt))
    }
}

