use num_bigint::{BigUint};

pub struct Key_Pair {
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