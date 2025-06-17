use elliptic_curves_rs::core::curve::Curve;
use elliptic_curves_rs::core::field::PrimeField;
use elliptic_curves_rs::core::point::CurvePoint;
use elliptic_curves_rs::curves::secp256k1::secp256k1::FqSecp256k1;
use elliptic_curves_rs::curves::secp256k1::secp256k1::FrSecp256k1;
use elliptic_curves_rs::curves::secp256k1::secp256k1::PointSecp256k1;
use elliptic_curves_rs::curves::secp256k1::secp256k1::Secp256k1Curve;

use elliptic_curves_rs::breaking_dlp::brute_force::BruteForce;
use elliptic_curves_rs::breaking_dlp::DiscreteLog;

use ark_std::{UniformRand, rand::thread_rng};

fn main() {
    // let p = PointSecp256k1::new(
    //     FqSecp256k1::from_u64(1),
    //     FqSecp256k1::from_u64(2),
    // );

    // let q = PointSecp256k1::new(
    //     FqSecp256k1::from_u64(3),
    //     FqSecp256k1::from_u64(4),
    // );
    // println!("{}", p);
    // println!("{}", q);

    // let r1 = p.add(&q);
    // println!("{}", r1);

    // let a: CurvePoint<Secp256k1Curve> = PointSecp256k1::new(
    //     FqSecp256k1::from_u64(1),
    //     FqSecp256k1::from_u64(2),
    // );
    // let b: CurvePoint<Secp256k1Curve> = PointSecp256k1::new(
    //     FqSecp256k1::from_u64(1),
    //     FqSecp256k1::from_u64(2),
    // );

    // let r2 = Secp256k1Curve::add_point(&a.inner, &b.inner);
    // println!("{}", r2);

    // let g = Secp256k1Curve::generator();
    // println!("G = {}", g);
    // println!("2G = {}", two_g);
    
    let mut rng = thread_rng();
    let random_integer = FrSecp256k1::rand(&mut rng);
    
    let g = Secp256k1Curve::generator();
    let two = FrSecp256k1::from(2);
    let q = g.mul_scalar(&two);

    let (steps, logarithm) = BruteForce::solve(&g, &q);
    println!("steps: {}, logarithm: {}", steps, logarithm);
}
