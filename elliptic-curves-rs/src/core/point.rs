use crate::core::traits::{PrimeField, Zero};
use crate::core::field::FieldOps;

pub struct Point<F: PrimeField> {
    pub x: F,
    pub y: F,
    pub infinity: bool,
}

impl<F: PrimeField> Point<F> {
    pub fn new(x: F, y: F) -> Self {
        Self { x, y, infinity: false }
    }

    pub fn infinity() -> Self {
        Self { x: F::zero(), y: F::zero(), infinity: true }
    }

    pub fn add(&self, other: &Self) -> Self {
        // to be implemented
    }

    pub fn double(&self) -> Self {

    }
}