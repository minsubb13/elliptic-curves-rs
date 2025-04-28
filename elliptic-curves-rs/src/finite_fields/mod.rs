use ark_ff::{BigInteger, BigInteger256, Fp256, PrimeField};
use ark_ff::fields::{MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "3"]
pub struct Secp256k1Config;
pub type Fq = Fp256<MontBackend<Secp256k1Config, 4>>;

#[test]
fn test_arithmetic() {
    let a = Fq::from(123u64);
    let b = Fq::from(456u64);
    let c = a + b;
    assert_eq!(c, a + b);
}

#[test]
fn test_max_int() {
    let a = Fq::from(u64::MAX);
    let b = Fq::from(u64::MAX);
    let c = a + b;
    let mut a_ = a.into_bigint();
    let mut b_ = b.into_bigint();
    let carry = a_.add_with_carry(&b_);
    assert_eq!(carry, false);
}