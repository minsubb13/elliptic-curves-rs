use elliptic_curves_rs::core::curve::Curve;
use elliptic_curves_rs::core::field::PrimeField;
use elliptic_curves_rs::core::point::CurvePoint;
use elliptic_curves_rs::curves::secp256k1::secp256k1::FqSecp256k1;
use elliptic_curves_rs::curves::secp256k1::secp256k1::PointSecp256k1;
use elliptic_curves_rs::curves::secp256k1::secp256k1::Secp256k1Curve;

use ark_ff::{
    biginteger,
    BigInteger,
    BigInteger256,
    Field as ArkField,
    PrimeField as ArkPrimeField,
    FpConfig,
    MontBackend,
    Zero};
use ark_std::{One, UniformRand};

fn main() {
    let p = PointSecp256k1::new(
        FqSecp256k1::from_u64(1),
        FqSecp256k1::from_u64(2),
    );

    let q = PointSecp256k1::new(
        FqSecp256k1::from_u64(3),
        FqSecp256k1::from_u64(4),
    );
    println!("{}", p);
    println!("{}", q);

    // let r1 = p.add(&q);
    // println!("{}", r1);

    let a: CurvePoint<Secp256k1Curve> = PointSecp256k1::new(
        FqSecp256k1::from_u64(1),
        FqSecp256k1::from_u64(2),
    );
    let b: CurvePoint<Secp256k1Curve> = PointSecp256k1::new(
        FqSecp256k1::from_u64(1),
        FqSecp256k1::from_u64(2),
    );

    let r2 = Secp256k1Curve::add_point(&a.inner, &b.inner);
    println!("{}", r2);

    // let r2 = Secp256k1Curve::add_point(&p, &q);
}
