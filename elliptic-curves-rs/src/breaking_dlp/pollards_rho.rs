//! Pollard's rho algorithm for Discrete Logarithm Problem

use crate::breaking_dlp::DiscreteLog;
use crate::core::curve::Curve;
use crate::core::point::CurvePoint;

use ark_ff::{BigInteger, Field, PrimeField, Zero};
use ark_std::{UniformRand, rand::thread_rng};

pub struct PollardsRho;

/// X = aP + bQ
#[derive(Debug)]
struct IterationState<C: Curve> {
    point: CurvePoint<C>,
    a: C::ScalarField,
    b: C::ScalarField,
}

impl<C: Curve> IterationState<C> {
    fn new(
        point: CurvePoint<C>,
        a: C::ScalarField,
        b: C::ScalarField,
    ) -> Self {
        Self { point, a, b }
    }
}

/// pseudo-random function
struct PseudoRandomFunction<C: Curve> {
    p: CurvePoint<C>,
    q: CurvePoint<C>,
    partition_count: usize,
    precomputed_points: Vec<CurvePoint<C>>,
    a_coeffs: Vec<C::ScalarField>,
    b_coeffs: Vec<C::ScalarField>,
}

impl<C: Curve> PseudoRandomFunction<C> {
    fn new(p: &CurvePoint<C>, q: &CurvePoint<C>) -> Self {
        let mut rng = thread_rng();
        let partition_count = 20;

        let mut precomputed_points = Vec::with_capacity(partition_count);
        let mut a_coeffs = Vec::with_capacity(partition_count);
        let mut b_coeffs = Vec::with_capacity(partition_count);

        // precompute a point R from R_0 to R_19
        for i in 0..partition_count {
            let a = C::ScalarField::rand(&mut rng);
            let b = C::ScalarField::rand(&mut rng);

            // R_i = aP + bQ
            let r_i = p.mul_scalar(&a).add(&q.mul_scalar(&b));

            precomputed_points.push(r_i);
            a_coeffs.push(a);
            b_coeffs.push(b);
        }

        Self {
            p: p.clone(),
            q: q.clone(),
            partition_count,
            precomputed_points,
            a_coeffs,
            b_coeffs,
        }
    }

    /// When given point X, determine the point that belongs
    fn get_partition(&self, point: &CurvePoint<C>) -> usize {
        match point.inner.x() {
            Some(x) => {
                let bytes = x.into_bigint().to_bytes_be();
                
                let mut hash = 0usize;
                for (i, &byte) in bytes.iter().take(8).enumerate() {
                    hash ^= (byte as usize) << (i * 8);
                }

                hash % self.partition_count
            }
            None => 0,
        }
    }

    /// f(X) = X + R_j, where j = partition(X)
    /// R_j = a_j * P + b_j * Q
    fn apply(&self, state: &IterationState<C>) -> IterationState<C> {
        let j = self.get_partition(&state.point);

        let new_point = state.point.add(&self.precomputed_points[j]);

        let new_a = state.a + self.a_coeffs[j];
        let new_b = state.b + self.b_coeffs[j];

        IterationState::new(new_point, new_a, new_b)
    }
}

impl<C: Curve> DiscreteLog<C> for PollardsRho {
    fn solve(
            p: &CurvePoint<C>,
            q: &CurvePoint<C>,
        ) -> (u64, <C as Curve>::ScalarField) {
        if !C::is_on_curve(&p.inner) || !C::is_on_curve(&q.inner) {
            panic!("p or q are not on curve");
        }

        let f = PseudoRandomFunction::new(p, q);

        let mut rng = thread_rng();
        let start_a = C::ScalarField::rand(&mut rng);
        let start_b = C::ScalarField::rand(&mut rng);
        let start_point = p.mul_scalar(&start_a).add(&q.mul_scalar(&start_b));

        let mut tortoise = IterationState::new(start_point.clone(), start_a, start_b);
        let mut hare = IterationState::new(start_point.clone(), start_a, start_b);

        let mut steps = 0u64;

        loop {
            tortoise = f.apply(&tortoise);

            hare = f.apply(&hare);
            hare = f.apply(&hare);

            steps += 1;
            
            if steps % 10000 == 0 {
                println!("Steps: {}", steps);
            }

            if tortoise.point.inner == hare.point.inner {
                println!("Collision found at step {}", steps);
                // tortoise: a1*P + b1*Q
                // hare: a2*P + b2*Q
                // tortoise = hare: a1*P + b1*Q = a2*P + b2*Q
                //                  (a1-a2)*P = (b2-b1)*Q
                // Q = k*P:         (a1-a2)*P = k*(b2-b1)*P
                // k = (a1-a2)/(b2-b1) mod n

                let a_diff = tortoise.a - hare.a;
                let b_diff = hare.b - tortoise.b;

                println!("a_diff: {:?}, b_diff: {:?}", a_diff, b_diff);

                if b_diff.is_zero() {
                    println!("b_diff is zero, retrying...");
                    continue;
                }

                if let Some(b_diff_inv) = b_diff.inverse() {
                    let k = a_diff * b_diff_inv;

                    if &p.mul_scalar(&k) == q {
                        return (steps, k);
                    }
                }
            }
            if steps >= 10_000_000 {
                panic!("Failed to find discrete logarithm within 10m steps");
            }
        }
    }
}
