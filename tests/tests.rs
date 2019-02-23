extern crate rsa_rust;

use rsa_rust::helpers::math::*;
use rsa_rust::types::*;

#[cfg(test)]
#[test]
fn generates_random_biguint() {
    let a = gen_big_num(&1024);
    assert_eq!(a.to_bytes_be().len(), 128 as usize);
    let b = gen_big_num(&2056);
    assert_eq!(b.to_bytes_be().len(), 257 as usize);
    let c = gen_big_num(&512);
    assert_eq!(c.to_bytes_be().len(), 64 as usize);
    let d = gen_big_num(&4096);
    assert_eq!(d.to_bytes_be().len(), 512 as usize);
    let e = gen_big_num(&256);
    assert_eq!(e.to_bytes_be().len(), 32 as usize);
}

#[cfg(test)]
#[test]
fn encrypts_info() {
    let kp = KeyPair::new(&512u32, &Threshold::new(&10)).unwrap();
    let msg = "Hello World!";
    println!("{}", kp.pk.encrypt(msg).unwrap());
}

#[cfg(test)]
#[test]
fn encrypts_decrypts_info() {
    let kp = KeyPair::new(&512u32, &Threshold::new(&10)).unwrap();
    let msg = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent non nunc et ipsum tempus fermentum";
    let cyphertext = kp.pk.encrypt(msg).unwrap();
    

    let res_decrypt = kp.sk.decrypt(&cyphertext).unwrap();
    println!("Result of decryption is: {}", res_decrypt);
    assert_eq!(res_decrypt, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent non nunc et ipsum tempus fermentum")
}

