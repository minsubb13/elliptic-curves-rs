use crate::core::field::PrimeField;
use crate::core::curve::Curve;
use crate::core::point::{Point, CurvePoint};

use ark_ff::{BigInteger, BigInteger256, Field, Fp256};
use ark_ff::fields::{MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "3"]
pub struct Secp256k1Curve;
pub type FqSecp256k1 = Fp256<MontBackend<Secp256k1Curve, 4>>;

pub type PointSecp256k1 = CurvePoint<Secp256k1Curve>;

impl Curve for Secp256k1Curve {
    type BaseField = FqSecp256k1;

    fn add_point(
        p: &Point<Self::BaseField>,
        q: &Point<Self::BaseField>,
    ) -> Point<Self::BaseField> {
        if p.is_infinity() {
            return q.clone();
        }
        if q.is_infinity() {
            return p.clone();
        }

        let x1 = p.x().unwrap();
        let y1 = p.y().unwrap();
        let x2 = q.x().unwrap();
        let y2 = q.y().unwrap();

        

    }

    fn double_point(p: &crate::Point<Self::BaseField>) -> crate::Point<Self::BaseField> {
        
    }

    fn mul_scalar(
            p: &crate::Point<Self::BaseField>,
            scalar: &<Self::BaseField as ark_ff::PrimeField>::BigInt,
        ) -> crate::Point<Self::BaseField> {
        
    }
}

impl crate::core::field::Field for FqSecp256k1 {
    fn
}

impl crate::core::field::PrimeField for FqSecp256k1 {
    fn from_u64(n: u64) -> Self { FqSecp256k1::from(n) }
    fn to_u64(self) -> u64 {
        
    }
}





#[test]
fn test_arithmetic() {
    let a = FqSecp256k1::from(123u64);
    let b = FqSecp256k1::from(456u64);
    let c = a + b;
    assert_eq!(c, a + b);

}

#[test]
fn test_point() {
    let p = PointSecp256k1::new(FqSecp256k1::from(1u64), FqSecp256k1::from(2u64));
}

#[test]
fn test_max_int() {
    let a = FqSecp256k1::from(u64::MAX);
    let b = FqSecp256k1::from(u64::MAX);
    let c = a + b;
    // let mut a_ = a.into_bigint();
    // let mut b_ = b.into_bigint();
    // let carry = a_.add_with_carry(&b_);
    // assert_eq!(carry, false);
}