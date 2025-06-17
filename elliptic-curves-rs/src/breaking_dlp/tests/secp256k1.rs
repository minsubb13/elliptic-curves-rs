use crate::breaking_dlp::brute_force::BruteForce;
use crate::breaking_dlp::DiscreteLog;
use crate::core::curve::Curve;
use crate::curves::secp256k1::secp256k1::{
    Secp256k1Curve,
    FqSecp256k1,
    FrSecp256k1,
};

use ark_std::{UniformRand, rand::thread_rng};

#[test]
#[ignore = "It takes very long time"]
fn test_brute_force_in_secp256k1() {
    let mut rng = thread_rng();
    let random_integer = FrSecp256k1::rand(&mut rng);

    let g = Secp256k1Curve::generator();
    let q = g.mul_scalar(&random_integer);

    let (steps, logarithm) = BruteForce::solve(&g, &q);
    println!("steps: {}, logarithm: {}", steps, logarithm);
}