use crate::breaking_dlp::DiscreteLog;
use crate::core::curve::Curve;
use crate::core::point::CurvePoint;

use ark_ff::One;
use ark_std::{
    UniformRand,
    rand::thread_rng,
};

pub struct BruteForce;

/// For breaking discrete logarithm problem using brute-force, it takes
/// very long time to break. Therefore, in this implementation, I limit
/// the range of loop for observing the algorithm process.
impl<C: Curve> DiscreteLog<C> for BruteForce {
    fn solve(
            p: &CurvePoint<C>,
            q: &CurvePoint<C>,
        ) -> (u64, C::ScalarField) {
        if !C::is_on_curve(&p.inner) || !C::is_on_curve(&q.inner) {
            panic!("p or q are not on curve");
        }

        let mut rng = thread_rng();
        let start = C::ScalarField::rand(&mut rng);
        let mut current = start;
        let end = u64::MAX;

        for steps in 0..end {
            let result = p.mul_scalar(&current);
            if result.inner == q.inner {
                return (steps, current);
            }

            current += C::ScalarField::one();
        }
        panic!("Failed to find discrete logarithm within {} iterations", end);
    }
}