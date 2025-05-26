use crate::core::field::{Field, PrimeField};
use crate::core::curve::Curve;
use crate::core::point::{Point, CurvePoint};

use ark_ff::{BigInteger, BigInteger256, Field as ArkField, PrimeField as ArkPrimeField, Fp256};
use ark_ff::fields::{MontBackend, MontConfig};

// Secp256k1 parameter
#[derive(MontConfig, PartialEq)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "3"]
pub struct Secp256k1Curve;
pub type FqSecp256k1 = Fp256<MontBackend<Secp256k1Curve, 4>>;

pub type PointSecp256k1 = CurvePoint<Secp256k1Curve>;

impl Field for FqSecp256k1 {
    fn zero() -> Self {
        FqSecp256k1::zero()
    }

    fn one() -> Self {
        FqSecp256k1::one()
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn mul(self, other: Self) -> Self {
        self * other
    }
    
    fn inv(self) -> Self {
        self.inverse().unwrap()
    }
}

impl PrimeField for FqSecp256k1 {
    fn from_u64(n: u64) -> Self {
        FqSecp256k1::from(n)
    }

    // fn to_u64(self) -> u64 {
    //     let repr = self.into_repr();
    //     repr.0[0]
    // }
}

impl Curve for Secp256k1Curve {
    type BaseField = FqSecp256k1;

    fn add_point(
        p: &Point<FqSecp256k1>,
        q: &Point<FqSecp256k1>,
    ) -> Point<FqSecp256k1> {
        eprintln!("add_point: p={:?}, q={:?}", p, q);
        if p.is_infinity() {
            return q.clone();
        }
        if q.is_infinity() {
            return p.clone();
        }

        if p.x() == q.x() && p.y() == q.y() {
            return Self::double_point(p);
        }
        
        // if p.is_on_curve() == false

        let x1 = p.x().unwrap();
        let y1 = p.y().unwrap();
        let x2 = q.x().unwrap();
        let y2 = q.y().unwrap();

        // a = 0 in Secp256k1
        let a = FqSecp256k1::zero();
        let two = FqSecp256k1::from(2u64);
        let three = FqSecp256k1::from(3u64);

        let m = if p.x() == q.x() && p.y() == q.y() {
            // m = (3 * x1^2 + a) / (2 * y1)
            let numerator = three * x1.square() + a;
            let denominator = (two * y1).inverse().expect("y1 != 0");
            numerator * denominator
        } else {
            // m = (y1 - y2) / (x1 - x2)
            let numerator = y1 - y2;
            let denominator = (x1 - x2).inverse().expect("!= 0");
            numerator * denominator
        };
        // x_r = m^2 - x1 - x2
        // y_r = y1 + m(x_r - x1)
        let x_r = m.square() - x1 - x2;
        let y_r = m * (x1 - x_r) - y1;

        Point::Affine { x: x_r, y: y_r }
    }

    fn double_point(p: &Point<FqSecp256k1>) -> Point<FqSecp256k1> {
        if p.is_infinity() {
            return Point::Infinity;
        }

        Secp256k1Curve::add_point(p, p)
    }

    fn mul_scalar(
        p: &Point<FqSecp256k1>,
        scalar: &<Self::BaseField as ArkPrimeField>::BigInt,
    ) -> Point<FqSecp256k1> {
        let bits = scalar.to_bits_le();
        let mut result = Point::infinity();
        let mut acc = p.clone();
        for bit in bits {
            if bit {
                result = Self::add_point(&result, &acc);
            }
            acc = Self::double_point(&acc);
        }
        result
    }
}

// #[test]
// fn test_arithmetic() {
//     let a = PointSecp256k1::from(123);
//     let b = PointSecp256k1::from(456);
//     let c = a + b;
//     assert_eq!(c, a + b);
// }

// #[test]
// fn test_point_addition() {
//     let p = PointSecp256k1::new(
//         FqSecp256k1::from(1),
//         FqSecp256k1::from(2),
//     );

//     let q = PointSecp256k1::new(
//         FqSecp256k1::from(3),
//         FqSecp256k1::from(4),
//     );

    
// }