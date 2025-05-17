use crate::core::traits::PrimeField;
use crate::core::point::Point;

pub struct Curve<F: PrimeField> {
    pub a: F,
    pub b: F,
    pub generator: Point<F>,
    pub order: F,
}

impl<F: PrimeField> Curve<F> {
    pub fn new(a: F, b: F, generator: Point<F>, order: F) -> Self {
        Self { a, b, generator, order }
    }

    pub fn is_on_curve(&self, p: &Point<F>) -> bool {

    }

    pub fn scalar_mul(&self, k: F, p: Point<F>) -> Point<F> {
        
    }
}
