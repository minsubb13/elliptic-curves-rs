use elliptic_curves_rs::finite_fields::Fq;

use ark_ff::{Field, PrimeField, FpConfig, BigInteger, Zero};
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

    let field_size_in_bits = Fq::MODULUS_BIT_SIZE;
    println!("field_size_in_bits: {}", field_size_in_bits);
}
