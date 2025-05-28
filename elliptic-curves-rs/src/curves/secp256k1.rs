use crate::core::field::{Field, PrimeField};
use crate::core::curve::Curve;
use crate::core::point::{Point, CurvePoint};

use ark_ff::{
    fields::{MontBackend, MontConfig},
    BigInteger,
    BigInteger256,
    Field as ArkField,
    PrimeField as ArkPrimeField,
    Fp256,
};

use std::str::FromStr;
use std::u64;

// Secp256k1 parameter
#[derive(MontConfig, PartialEq)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "1"]
pub struct Secp256k1Curve;
pub type FqSecp256k1 = Fp256<MontBackend<Secp256k1Curve, 4>>;

pub type PointSecp256k1 = CurvePoint<Secp256k1Curve>;

impl Field for FqSecp256k1 {
    fn zero() -> Self {
        Self::zero()
    }

    fn one() -> Self {
        Self::one()
    }

    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn mul(&self, other: &Self) -> Self {
        self * other
    }
    
    fn inv(&self) -> Self {
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

    fn a() -> FqSecp256k1 {
        FqSecp256k1::from_u64(0)
    }

    fn b() -> FqSecp256k1 {
        FqSecp256k1::from_u64(7)
    }

    fn generator() -> Point<Self::BaseField> {
        let x = FqSecp256k1::from_str(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"
        ).unwrap();
        let y = FqSecp256k1::from_str(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"
        ).unwrap();
        Point::Affine { x, y }
    }

    fn add_point(
        p: &Point<FqSecp256k1>,
        q: &Point<FqSecp256k1>,
    ) -> Point<FqSecp256k1> {
        use Point::*;

        match (p, q) {
            (Infinity, _) => q.clone(),
            (_, Infinity) => p.clone(),
            (Affine { x: x1, y: y1 }, Affine { x: x2, y:y2 }) => {
                if x1 == x2 && y1 == &y2.mul(&FqSecp256k1::from_u64(u64::MAX)) {
                    return Infinity;
                }

                if x1 == x2 && y1 == y2 {
                    return Self::double_point(p);
                }

                let m = {
                    let numerator = y1 - y2;
                    let denominator = (x1 - x2).inverse().expect("y1 != 0");
                    numerator * denominator
                };
                let x_r = m.square() - x1 - x2;
                let y_r = m * (x1 - &x_r) - y1;

                Point::Affine { x: x_r, y: y_r }
            }
        }
        // if p.is_infinity() {
        //     return q.clone();
        // }
        // if q.is_infinity() {
        //     return p.clone();
        // }

        // if p.x() == q.x() && p.y() == q.y() {
        //     return Self::double_point(p);
        // }
        
        // // if p.is_on_curve() == false

        // let x1 = p.x().unwrap();
        // let y1 = p.y().unwrap();
        // let x2 = q.x().unwrap();
        // let y2 = q.y().unwrap();

        // // a = 0 in Secp256k1
        // let a = FqSecp256k1::zero();
        // let two = FqSecp256k1::from(2u64);
        // let three = FqSecp256k1::from(3u64);

        // let m = if p.x() == q.x() && p.y() == q.y() {
        //     // m = (3 * x1^2 + a) / (2 * y1)
        //     let numerator = three * x1.square() + a;
        //     let denominator = (two * y1).inverse().expect("y1 != 0");
        //     numerator * denominator
        // } else {
        //     // m = (y1 - y2) / (x1 - x2)
        //     let numerator = y1 - y2;
        //     let denominator = (x1 - x2).inverse().expect("!= 0");
        //     numerator * denominator
        // };
        // // x_r = m^2 - x1 - x2
        // // y_r = y1 + m(x_r - x1)
        // let x_r = m.square() - x1 - x2;
        // let y_r = m * (x1 - x_r) - y1;

        // Point::Affine { x: x_r, y: y_r }
    }

    fn double_point(p: &Point<FqSecp256k1>) -> Point<FqSecp256k1> {
        use Point::*;
        match p {
            Infinity => Infinity,
            Affine { x, y } => {
                let two = FqSecp256k1::from_u64(2);
                let three = FqSecp256k1::from_u64(3);
                let numerator = three * x.square() + Self::a();
                let denominator = (two * y).inverse().expect("y1 != 0");
                let m = numerator * denominator;
                let x_r = m.square() - x - x;
                let y_r = m * (x - &x_r) - y;
                Affine { x: x_r, y: y_r }
            }
        }
        // if p.is_infinity() {
        //     return Point::Infinity;
        // }

        // Secp256k1Curve::add_point(p, p)
    }

    fn mul_scalar(
        p: &Point<FqSecp256k1>,
        scalar: &FqSecp256k1,
    ) -> Point<FqSecp256k1> {
        let mut result = Point::infinity();
        let acc = p.clone();
        let bits = scalar.into_bigint().0;
        for limb in bits.iter().rev() {
            for i in (0..64).rev() {
                result = Self::double_point(&result);
                if (limb >> i) & 1 == 1 {
                    result = Self::add_point(&result, &acc);
                }
            }
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