use crate::core::field::Field;
use crate::core::curve::Curve;
use ark_ff::{PrimeField as ArkPrimeField};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Point<F: Field> {
    Infinity,
    Affine { x: F, y: F },
}

impl<F: Field> Point<F> {
    pub fn infinity() -> Self {
        Point::Infinity
    }

    pub fn new(x: F, y: F) -> Self {
        Point::Affine { x, y }
    }
}

pub struct CurvePoint<C: Curve> {
    pub inner: Point<C::BaseField>,
}

impl<C: Curve> CurvePoint<C> {
    pub fn infinity() -> Self {
        CurvePoint { inner: Point::Infinity }
    }

    pub fn add(&self, other: &Self) -> Self {
        let p = C::add_point(&self.inner, &other.inner);
        CurvePoint { inner: p }
    }

    pub fn double(&self) -> Self {
        let p = C::double_point(&self.inner);
        CurvePoint { inner: p }
    }

    pub fn mul_scalar(&self, scalar: &<C::BaseField as ArkPrimeField>::BigInt) -> Self {
        let p = C::mul_scalar(&self.inner, scalar);
        CurvePoint { inner: p }
    }
}

impl<C: Curve> fmt::Display for CurvePoint<C>
where
    C::BaseField: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Point::Infinity => write!(f, "Infinity"),
            Point::Affine { x, y } => write!(f, "({}, {})", x, y),
        }
    }
}
