# Rabe

rabe is a rust library implementing several Attribute Based Encryption (ABE) schemes using a modified version of the `bn` library of zcash (type-3 pairing / Baretto Naering curve). The modification of `bn` brings in `serde` instead of the deprecated `rustc_serialize`.

This is a rust crate and comes with C bindings. For integration in distributed applications head over to [rabe-keyserver](ttps://github.com/Fraunhofer-AISEC/rabe-keyserver), which wraps rabe in a standalone REST API webserver.

# Implemented Ciphertext Policy Schemes (CP-ABE)

## BDABE CP-ABE

Georg Bramm, Mark Gall, Julian Schütte , "Blockchain based Distributed Attribute-based Encryption". In Proceedings of SECRYPT 2018. (not published yet)

## AC17 CP-ABE

Shashank Agrawal, Melissa Chase, "FAME: Fast Attribute-based Message Encryption", (Section 3). In Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security 2017. Available from https://eprint.iacr.org/2017/807.pdf

## AW11 CP-ABE

Lewko, Allison, and Brent Waters, "Decentralizing Attribute-Based Encryption.", (Appendix D). In Eurocrypt 2011. Available from http://eprint.iacr.org/2010/351.pdf

## BSW CP-ABE

John Bethencourt, Amit Sahai, Brent Waters, "Ciphertext-Policy Attribute-Based Encryption" In IEEE Symposion on Security and Privacy, 2007. Available from https://doi.org/10.1109/SP.2007.11

## MKE08 CP-ABE

S Müller, S Katzenbeisser, C Eckert , "Distributed Attribute-based Encryption". Published in International Conference on Information Security and Cryptology, Heidelberg, 2008. Available from http://www2.seceng.informatik.tu-darmstadt.de/assets/mueller/icisc08.pdf


# Implemented Key Policy Schemes (KP-ABE)

## AC17 KP-ABE

Shashank Agrawal, Melissa Chase, "FAME: Fast Attribute-based Message Encryption". In Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security 2017. Available from https://eprint.iacr.org/2017/807.pdf

## LSW KP-ABE 

Allison Lewko, Amit Sahai and Brent Waters, "Revocation Systems with Very Small Private Keys". In IEEE Symposium on Security and Privacy, 2010. SP'10. Available from http://eprint.iacr.org/2008/309.pdf

# Building rabe

In order to compile and test:
- install rust nightly
- git clone library 
- install build-essential
- and then run 'cargo build && RUST_BACKTRACE=1 cargo test -- --nocapture'

In order to run on the console use 
- target/debug/rabe

For example, in order to create msk and pk of an AC17 KP-ABE scheme run:
```bash
$ ./target/debug/rabe --scheme AC17KP setup
```

To compile the C testfile:
```bash
gcc test.c -lrabe -L./target/debug -o test
```
