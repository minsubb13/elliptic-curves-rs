use super::secp256k1::FqSecp256k1;
use super::secp256k1::Secp256k1Curve;
use super::secp256k1::PointSecp256k1;
use super::secp256k1::FrSecp256k1;
use crate::core::curve::Curve;
use crate::core::field::PrimeField;

use ark_ff::BigInt;
use ark_ff::PrimeField as ArkPrimeField;
use ark_ff:: Field as ArkField;
use ark_ff::biginteger::BigInteger256;
use ark_ff::Zero;

use std::str::FromStr;

#[test]
fn test_check_parameters() {
    // prime:
    // 115792089237316195423570985008687907853269984665640564039457584007908834671663
    let prime = FqSecp256k1::MODULUS;
    let secp256k1_prime = BigInteger256::from_str(
        "115792089237316195423570985008687907853269984665640564039457584007908834671663",
    ).expect("Failed to parse modulus");
    assert_eq!(prime, secp256k1_prime);
    
    // a = 0
    // b = 7
    let a = Secp256k1Curve::a();
    let b  = Secp256k1Curve::b();
    assert_eq!(a, FqSecp256k1::zero());
    assert_eq!(b, FqSecp256k1::from_u64(7));

    // generator:
    // x: 55066263022277343669578718895168534326250603453777594175500187360389116729240
    // y: 32670510020758816978083085130507043184471273380659243275938904335757337482424
    let generator = Secp256k1Curve::generator();
    let g_x = FqSecp256k1::from_str(
        "55066263022277343669578718895168534326250603453777594175500187360389116729240",
    ).expect("panicked at g_x");
    let g_y = FqSecp256k1::from_str(
        "32670510020758816978083085130507043184471273380659243275938904335757337482424",
    ).expect("panicked at g_y");
    let g = PointSecp256k1::new(g_x, g_y);
    assert_eq!(generator, g);

    // order:
    // 115792089237316195423570985008687907852837564279074904382605163141518161494337
    let order = Secp256k1Curve::order();
    let expected_order = BigInt::from_str(
        "115792089237316195423570985008687907852837564279074904382605163141518161494337"
    ).unwrap();
    assert_eq!(order, expected_order);
}

#[test]
fn test_add_infinity_point() {
    // P + infinity = P
    // infinity + P = P
    let g = Secp256k1Curve::generator();
    let infinity = PointSecp256k1::infinity();

    let g_add_infinity = g.add(&infinity);
    let infinity_add_g = infinity.add(&g);

    assert_eq!(g, g_add_infinity);
    assert_eq!(g_add_infinity, infinity_add_g);
}

#[test]
fn test_basic_operations() {
    use ark_ff::AdditiveGroup;
    use ark_std::{One, UniformRand, test_rng};
    let mut rng = test_rng();

    let a = FqSecp256k1::rand(&mut rng);
    let b = FqSecp256k1::rand(&mut rng);
    let c = a + b;
    let d = a - b;

    assert_eq!(c + d, a.double());

    let e = c * d;
    assert_eq!(e, a.square() - b.square());
    assert_eq!(a.inverse().unwrap() * a, FqSecp256k1::one());
}

#[test]
fn test_is_on_curve() {
    let g = Secp256k1Curve::generator();
    let is_on_curve = Secp256k1Curve::is_on_curve(&g.inner);
    assert!(is_on_curve);
}

#[test]
fn test_point_addition() {
    let g = Secp256k1Curve::generator();
    let two_gx = FqSecp256k1::from_str(
        "89565891926547004231252920425935692360644145829622209833684329913297188986597"
    ).unwrap();
    let two_gy = FqSecp256k1::from_str(
        "12158399299693830322967808612713398636155367887041628176798871954788371653930"
    ).unwrap();

    let expected_2g = PointSecp256k1::new(two_gx, two_gy);
    let two_g = g.add(&g); // g + g = 2g
    let double_g = g.double(); // doubling g

    assert_eq!(two_g, expected_2g);
    assert_eq!(double_g, expected_2g);
}

#[test]
fn test_point_subtract() {
    let g = Secp256k1Curve::generator();
    let two_g = g.double();
    let two_g_minus_g = two_g.subtract(&g);
    
    let infinity = PointSecp256k1::infinity();
    let g_minus_g = g.subtract(&g);

    assert_eq!(g, two_g_minus_g);
    assert_eq!(infinity, g_minus_g);
}

#[test]
fn test_scalar_mul() {
    let generator = Secp256k1Curve::generator();
    let k = FrSecp256k1::from(2u64);
    let three = FrSecp256k1::from(3);

    let two_g_expected = generator.add(&generator);
    let two_times_g = generator.mul_scalar(&k);

    let three_g = two_g_expected.add(&generator);
    let three_times_g = generator.mul_scalar(&three);

    assert_eq!(two_times_g, two_g_expected);
    assert_eq!(three_g, three_times_g);
}