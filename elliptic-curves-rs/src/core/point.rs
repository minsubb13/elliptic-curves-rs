use crate::core::traits::{Field, PrimeField, Zero};
use crate::core::field::FieldOps;
use std::ops::Add;
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

    /// addition for point
    /// p1.add(&p2) => return new point
    pub fn add(&self, other: &Point<F>) -> Self {
        match (self, other) {
            // 0 + 0 = 0
            (Self::Infinity, Self::Infinity) => Self::Infinity,
            // 0 + Q = Q, P + 0 = P
            (Point::Infinity, p) | (p, Point::Infinity) => p.clone(),

            (Self::Affine { x: x1, y: y1 }, Self::Affine { x: x2, y: y2 }) => {
                Self::Affine {
                    x: *x1 + *x2,
                    y: *y1 + *y2,
                }
            }
        }
    }

    /// doubling point
    pub fn double_point(&self) -> Self {
        match self {
            Self::Infinity => Self::Infinity,
            Self::Affine { x: x1, y: y1 } => {
                Self::Affine {
                    x: *x1 + *x1,
                    y: *y1 + *y1,
                }
            }
        }
    }
}

impl<F: Field> std::ops::Add for Point<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(&other)
    }
}

impl<F: Field> std::ops::Add<&Point<F>> for Point<F> {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        self.add(other)
    }
}

impl<F: Field> std::ops::Add<Point<F>> for &Point<F> {
    type Output = Point<F>;
    
    fn add(self, other: Point<F>) -> Point<F> {
        self.add(&other)
    }
}

impl<F: Field> std::ops::Add for &Point<F> {
    type Output = Point<F>;
    
    fn add(self, other: Self) -> Point<F> {
        self.add(other)
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



#[cfg(test)]
mod tests {
    use super::*;
    use ark_secp256k1::Fq;

    #[test]
    fn test_point_addition() {
        let p1 = Point::Affine {
            x: Fq::from(1u64),
            y: Fq::from(2u64), 
        };

        let p2 = Point::Affine {
            x: Fq::from(3u64),
            y: Fq::from(4u64),
        };

        let result = p1.add(&p2);

        let expected = Point::Affine {
            x: Fq::from(4u64),
            y: Fq::from(6u64),
        };

        assert_eq!(result, expected);

    }
}