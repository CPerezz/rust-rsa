# Rust-rsa
## This is an implementation of the RSA algorithm in Rust.

### DISCLAIMER: THIS IS A PERSONAL PROJECT JUST FOR FUN. DO NOT USE THIS LIBRARY ON PRODUCTION ENVOIRMENTS SINCE IS NOT DESIGNED FOR THAT KIND OF PURPOSES

TODO:
- [x] Generate big numbers of 1024 bits and 2048 bits.
- [x] Implement modular exponentiation operation needed by Rabin-Miller algorithm. (**TESTED AND WORKING**)
- [x] Implement Rabin Miller algorithm. (**TESTED AND WORKING**)
- [x] Implement Extended Euclides Algorithm to search for mcd of two suposed prime numbers.
- [x] Generate valid KeyPairs of a deterministic lenght. (**TESTED AND WORKING. RECOMMENDED THRESHOLD:DEFAULT // RECOMENDED KEYPAIR SIZE: 512 OR 1024 BITS**)
- [x] Encrypt messages with Public Key.
- [x] Decrypt messages with Secret Key. (**TESTED AND WORKING, NEEDS TO BE EVEN MORE TESTED**)
- [ ] Sign messages.
- [ ] Implement Paddings (PKCS1, PKCS7, PKCS-OAP)
- [ ] Order and write better tests.
