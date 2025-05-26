use std::u64;

use elliptic_curves_rs::core::curve::Curve;
// use crate::finite_fields::secp256k1::{Fq, Secp256k1Config};
use elliptic_curves_rs::curves::secp256k1::FqSecp256k1;
use elliptic_curves_rs::curves::secp256k1::PointSecp256k1;
use elliptic_curves_rs::curves::secp256k1::Secp256k1Curve;


use ark_ff::{biginteger, BigInteger, BigInteger256, Field, FpConfig, MontBackend, PrimeField, Zero};
use ark_std::{One, UniformRand};

fn main() {
    let p = PointSecp256k1::new(
        FqSecp256k1::from(1),
        FqSecp256k1::from(2),
    );

    let q = PointSecp256k1::new(
        FqSecp256k1::from(3),
        FqSecp256k1::from(4),
    );
    println!("{}", p);
    println!("{}", q);

    let r1 = p.add(&q);
    println!("{}", r1);

    // let r2 = Secp256k1Curve::add_point(&p, &q);
    // println!("{}", r2);

}
