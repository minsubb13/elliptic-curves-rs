use crate::core::traits::{ArkField, PrimeField};

use ark_ff::{BigInteger, BigInteger256, Fp256, PrimeField};
use ark_ff::fields::{MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "3"]
pub struct Secp256k1Config;
pub type Fq = Fp256<MontBackend<Secp256k1Config, 4>>;

pub trait FieldOps: ArkField {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn inv(&self) -> Option<Self>;
}

impl<F: PrimeField> FieldOps for F {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }

    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }

    fn inv(&self) -> Option<Self> {
        self.inverse()
    }
}