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
- [x] Decrypt messages with Secret Key. (**TESTED AND WORKING**)
- [ ] Implement From trait for Public, Secret and KeyPair.
- [ ] Implement Parallel computations of p & q prime original numbers to increase performance.
- [ ] Implement Hybrid encryption process using AES-128/256 (to determine).
- [ ] Optimize Rabin-Miller algorithm by discarting multiples of [3, 5, 7, 9, 11, 13, 15, 19] before start computing the algoriythm.
- [ ] Brenchmark Rabin-Miller vs. Fermat's Primality test (see: [https://en.wikipedia.org/wiki/Fermat_primality_test](https://en.wikipedia.org/wiki/Fermat_primality_test). Not considering Baillie–PSW  since relies on Rabin-Miller.
- [ ] Sign messages.
- [ ] Implement Paddings (PKCS1, PKCS7, PKCS-OAP)
- [ ] Order and write better tests.
