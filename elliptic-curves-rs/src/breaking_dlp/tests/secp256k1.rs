use crate::breaking_dlp::brute_force::BruteForce;
use crate::breaking_dlp::pollards_rho::PollardsRho;
use crate::breaking_dlp::DiscreteLog;
use crate::core::curve::Curve;
use crate::curves::secp256k1::secp256k1::{
    Secp256k1Curve,
    FrSecp256k1,
};

use ark_std::{UniformRand, rand::thread_rng};

#[test]
#[ignore = "It takes a very long time"]
fn test_brute_force_in_secp256k1() {
    let mut rng = thread_rng();
    let x = FrSecp256k1::rand(&mut rng);

    let g = Secp256k1Curve::generator();
    let q = g.mul_scalar(&x);

    let (steps, logarithm) = BruteForce::solve(&g, &q);
    println!("steps: {}, logarithm: {}", steps, logarithm);
    assert_eq!(x, logarithm);
}

#[test]
#[ignore = "It takes a very long time"]
fn test_pollards_rho_in_secp256k1() {
    let g = Secp256k1Curve::generator();
    let k = FrSecp256k1::from(50u64);
    let q = g.mul_scalar(&k);

    let (steps, found_k) = PollardsRho::solve(&g, &q);
    println!("Found discrete log in {} steps", steps);
    assert_eq!(found_k, k);
}