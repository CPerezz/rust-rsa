use num_bigint::BigUint;
use rsa_rust::helpers::math::*;
use rsa_rust::types::*;
use std::str::FromStr;

#[cfg(test)]
#[test]
fn generates_random_biguint() {
    let a = gen_big_num(&1024);
    assert_eq!(a.to_bytes_be().len(), 128 as usize);
    let b = gen_big_num(&2048);
    assert_eq!(b.to_bytes_be().len(), 256 as usize);
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

#[cfg(test)]
#[test]
fn gets_pk_from_path() {
    let pk = PublicKey::from(".");
    assert_eq!(pk.n, BigUint::from_str("75626462905383810114071019025488086794291983623690245837345212912178468083847523332076280546591910929873274046866692147482675076300505326618868907457488692995114269236242451830025453338858200761669743695045994767037911174449670925555213937983874922734205349249559091867088950690421455820250299733076434610179").unwrap());
    assert_eq!(pk.e,  BigUint::from_str("40599802969944225367292239538097091647163178952613318779033512642944064830700837798736987876876511076542797131452079443801477521231136368963012113038768054186377971936865216631799722533794162350509136211558812954457005899222310648591974354098529147616639793901395613303896100491614721393418949191445512208289").unwrap());
}  


#[cfg(test)]
#[test]
fn gets_sk_from_path() {
    let sk = SecretKey::from(".");
    assert_eq!(sk.n, BigUint::from_str("75626462905383810114071019025488086794291983623690245837345212912178468083847523332076280546591910929873274046866692147482675076300505326618868907457488692995114269236242451830025453338858200761669743695045994767037911174449670925555213937983874922734205349249559091867088950690421455820250299733076434610179").unwrap());
    assert_eq!(sk.d,  BigUint::from_str("70567293958308636347718913085194140682351888216283971750405263761265129602353252274070127980356078397568566368854202969237467807089612840209364579583329803963852037573721436975376676795617996884644830446381279708101045164227601412236323450894814032789941265287234035928552676308259040442454576341751061187609").unwrap());
}  