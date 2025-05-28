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

    pub fn x(&self) -> Option<F> {
        match self {
            Self::Affine {x, .. } => Some(*x),
            Self::Infinity => None,
        }
    }

    pub fn y(&self) -> Option<F> {
        match self {
            Self::Affine {y, .. } => Some(*y),
            Self::Infinity => None,
        }
    }

    pub fn is_infinity(&self) -> bool {
        matches!(self, Self::Infinity)
    }
}

#[derive(PartialEq)]
pub struct CurvePoint<C: Curve> {
    pub inner: Point<C::BaseField>,
}

impl<C: Curve> CurvePoint<C> {
    pub fn new(x: C::BaseField, y: C::BaseField) -> Self {
        CurvePoint { inner: Point::Affine { x, y } }
    }

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

    pub fn mul_scalar(&self, scalar: &C::BaseField) -> Self {
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

impl<F> fmt::Display for Point<F>
where
    F: Field + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point::Infinity => write!(f, "Infinity"),
            Point::Affine { x, y } => write!(f, "({}, {})", x, y),
        }
    }
}