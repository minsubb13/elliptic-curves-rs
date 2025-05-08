use ark_ff::{BigInteger, BigInteger256, Field, Fp256, PrimeField};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Point<F:Field> {
    Infinity,
    Affine {x: F, y: F},
}

impl<F: Field> Point<F> {
    pub fn new(x: F, y: F) -> Self {
        Self::Affine { x, y }
    }

    pub fn infinity() -> Self {
        Self::Infinity
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

impl<F: Field + fmt::Display> fmt::Display for Point<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infinity => write!(f, "Infinity"),
            Self::Affine { x, y } => write!(f, "({}, {})", x, y),
        }
    }
}
