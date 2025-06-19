use crate::core::field::{Field, PrimeField};
use crate::core::curve::Curve;
use crate::core::point::{Point, CurvePoint};

use ark_ff::{
    fields::{MontBackend, MontConfig},
    Field as ArkField,
    PrimeField as ArkPrimeField,
    Fp256,
    Zero,
};

use std::str::FromStr;
use std::u64;

// Struct related to secp256k1 prime
#[derive(MontConfig, PartialEq, Debug)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "1"]
pub struct Secp256k1Curve;
pub type FqSecp256k1 = Fp256<MontBackend<Secp256k1Curve, 4>>;
pub type PointSecp256k1 = CurvePoint<Secp256k1Curve>;

// Struct related to secp256k1 subgroup order
#[derive(MontConfig, PartialEq, Debug)]
#[modulus = "115792089237316195423570985008687907852837564279074904382605163141518161494337"]
#[generator = "1"]
pub struct Secp256k1ScalarConfig;
pub type FrSecp256k1 = Fp256<MontBackend<Secp256k1ScalarConfig, 4>>;

impl Field for FqSecp256k1 {
    fn zero() -> Self {
        ark_ff::Zero::zero()
    }

    fn one() -> Self {
        ark_ff::One::one()
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

    fn neg(&self) -> Self {
        -*self
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
    type ScalarField = FrSecp256k1;

    fn a() -> FqSecp256k1 {
        FqSecp256k1::from_u64(0)
    }

    fn b() -> FqSecp256k1 {
        FqSecp256k1::from_u64(7)
    }

    fn generator() -> CurvePoint<Self> {
        let x = FqSecp256k1::from_str(
            "55066263022277343669578718895168534326250603453777594175500187360389116729240"
        ).unwrap();
        let y = FqSecp256k1::from_str(
            "32670510020758816978083085130507043184471273380659243275938904335757337482424"
        ).unwrap();
        
        let inner_affine = Point::new(x, y);
        CurvePoint { inner: inner_affine }
    }

    fn order() -> <Self::ScalarField as ArkPrimeField>::BigInt {
        FrSecp256k1::MODULUS
    }

    fn is_on_curve(p: &Point<FqSecp256k1>) -> bool {
        let a = Secp256k1Curve::a();
        let b = Secp256k1Curve::b();
        // y^2 = x^3 + ax + b
        match (p.x(), p.y()) {
            (Some(x), Some(y)) => {
                let left_term = y.square();
                let right_term = x.pow([3u64]) + a * x + b;
                left_term == right_term
            }
            _ => true,
        }
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
                // P + (-P) = infinity
                if x1 == x2 && (y1 + y2).is_zero() {
                    return Infinity;
                }

                // P + P = 2P
                if x1 == x2 && y1 == y2 {
                    return Self::double_point(p);
                }

                // P + Q (P != Q)
                let m = {
                    let numerator = y1 - y2;
                    let denominator = (x1 - x2).inverse()
                        .expect("denominator is zero.");
                    numerator * denominator
                };
                let x_r = m.square() - x1 - x2;
                let y_r = m * (x1 - &x_r) - y1;

                Point::Affine { x: x_r, y: y_r }
            }
        }
    }

    fn double_point(p: &Point<FqSecp256k1>) -> Point<FqSecp256k1> {
        use Point::*;
        match p {
            Infinity => Infinity,
            Affine { x, y } => {
                let two = FqSecp256k1::from_u64(2);
                let three = FqSecp256k1::from_u64(3);
                let numerator = three * x.square() + Self::a();
                let denominator = (two * y).inverse()
                    .expect("denominator is zero.");
                let m = numerator * denominator;
                let x_r = m.square() - x - x;
                let y_r = m * (x - &x_r) - y;
                Affine { x: x_r, y: y_r }
            }
        }
    }

    fn mul_scalar(
        p: &Point<FqSecp256k1>,
        scalar: &FrSecp256k1,
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

    fn subtract_point(
            p: &Point<FqSecp256k1>,
            q: &Point<FqSecp256k1>,
        ) -> Point<FqSecp256k1> {
        let neg_q = Self::negate_point(q);
        Self::add_point(p, &neg_q)
    }

    fn negate_point(p: &Point<FqSecp256k1>) -> Point<FqSecp256k1> {
        match p {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } => Point::Affine {
                x: *x,
                y: y.neg()
            },
        }
    }
}
