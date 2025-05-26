use std::u64;

// use crate::finite_fields::secp256k1::{Fq, Secp256k1Config};
use elliptic_curves_rs::curves::secp256k1::{Fq, Secp256k1Config};

use ark_ff::{biginteger, BigInteger, BigInteger256, Field, FpConfig, MontBackend, PrimeField, Zero};
use ark_std::{One, UniformRand};

fn main() {
    let a = Fq::from(1u64);
    println!("a: {}", a);

    let mut rng = ark_std::test_rng();
    let b = Fq::rand(&mut rng);
    let b_bigint = b.into_bigint();
    let b_num_bits: u32 = b_bigint.num_bits();

    println!("b: {}", b);
    println!("length of b: {}", b_num_bits);

    let modulus = Fq::MODULUS;
    println!("modulus:  {}", modulus);

    let big_int_max = BigInteger256::new([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
    let big_int = BigInteger256::new([
        0xFFFFFFFEFFFFFC2E,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF
    ]);
    println!("big int : {}", big_int);
    println!("big int : {}", big_int.num_bits());
    let ex_max = Fq::new(big_int);
    println!("ex_max : {}", ex_max);

    let ex_max2 = Fq::new(big_int);
    println!("ex_max : {}", ex_max2);

    let one: ark_ff::Fp<MontBackend<Secp256k1Config, 4>, 4> = Fq::one();
    println!("one : {}", one);

    // let ex_max_bigint = ex_max2.into_bigint();
    // let ex_max_num_bits = ex_max_bigint.num_bits();
    // println!("length of ex_max_num_bits: {}", ex_max_num_bits);

    let c = ex_max + one;
    println!("c : {}", c);
    let c_bigint = c.into_bigint();
    let c_num_bits = c_bigint.num_bits();
    println!("length of c: {}", c_num_bits);

    let max_elem = Fq::from(big_int);  // 필드의 최대 요소 (모듈러스 - 1)

    let result = max_elem + one;       // 결과는?

    println!("max_elem: {}", max_elem.into_bigint());
    println!("one: {}", one.into_bigint());
    println!("result: {}", result.into_bigint());

    assert_eq!(result, Fq::zero());    // 결과가 0인지 확인

}
