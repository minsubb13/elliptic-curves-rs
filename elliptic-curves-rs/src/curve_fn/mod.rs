use crate::point::Point;
use ark_ff::{Field, BigInteger256};
use std::fmt;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EllipticCurveError {
    #[error("Point is not on the curve")]
    PointNotOnCurve,
    
    #[error("Invalid curve parameters")]
    InvalidCurveParameters,
    
    #[error("Invalid field element")]
    InvalidFieldElement,
}

pub struct EllipticCurve<F: Field> {
    a: F,
    b: F,
    g: Point<F>,
    n: F,
}

impl<F: Field> EllipticCurve<F> {
    pub fn new(
        a: F,
        b: F,
        g: Point<F>,
        n: F
    ) -> Result<Self, EllipticCurveError> {
        let curve = Self{ a, b, g, n };

        let four = F::from(4u64);
        let twenty_seven = F::from(27u64);

        let discriminant = four * a.pow([3]) + twenty_seven * b.square();
        if discriminant.is_zero() {
            return Err(EllipticCurveError::InvalidCurveParameters);
        }
        
        if !curve.is_on_curve(&g) {
            return Err(EllipticCurveError::PointNotOnCurve);
        }

        Ok(curve)
    }

    pub fn is_on_curve(&self, point: &Point<F>) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine { x, y } => {
                let lhs = y.square();
                let rhs = x.pow([3]) + self.a * x + self.b;
                lhs == rhs
            }
        }
    }

    pub fn inverse(&self, point: &Point<F>) -> Result<Point<F>, EllipticCurveError> {
        if !self.is_on_curve(point) {
            return Err(EllipticCurveError::PointNotOnCurve);
        }

        match point {
            Point::Infinity => Ok(Point::Infinity),
            Point::Affine { x, y } => {
                let neg_y = -*y;
                Ok(Point::new(*x, neg_y))
            }
        }
    }

    pub fn add(&self, p: &Point<F>, q: &Point<F>)
        -> Result<Point<F>, EllipticCurveError> {
        if !self.is_on_curve(p) || !self.is_on_curve(q) {
            return Err(EllipticCurveError::PointNotOnCurve);
        }

        match (p, q) {
            (Point::Infinity, _) => return Ok(q.clone()),
            (_, Point::Infinity) => return Ok(p.clone()),
            _ => {}
        }

        // 이후 실제 덧셈 구현
        
        // Ok(Point::)
    }

    pub fn get_order(&self) -> F {
        self.n
    }

    pub fn get_generator(&self) -> Point<F> {
        self.g.clone()
    }
}

impl<F: Field + fmt::Display> fmt::Display for EllipticCurve<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "y^2 = x^3 + {}x _ {}",
            self.a, self.b
        )
    }
}
