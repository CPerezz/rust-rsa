extern crate num;
extern crate rand;
extern crate num_bigint;

pub mod helpers;
pub mod types;

use crate::types::{KeyPair, Threshold};
use num_bigint::BigUint;

pub fn generate_key_pair(size: &u32, threshold: &Threshold) -> KeyPair {
    // Probably will be implemented for KeyPair Struct
  unimplemented!()
}

pub fn generate_key_pair_from_p_q(p: &BigUint, q: &BigUint) -> Result<KeyPair, &'static str> {
    // Probably will be implemented for KeyPair Struct
 unimplemented!()
}

pub fn encrypt(){
    // Probably will be implemented for PublicKey Struct
unimplemented!()
}

pub fn decrypt(){
    // Probably will be implemented for SecretKey Struct
unimplemented!()
}

pub fn sign_message() {
    // Probably will be implemented for SecretKey Struct
unimplemented!()
}
