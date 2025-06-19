//! Baby-step Giant-step (BSGS) algorithm for solving the Discrete
//! Logarithm Problem
//! 
//! While BSGS improves upon brute-force from O(n) to O(sqrt(n)) time
//! complexity, it remains impractical for cryptography curves at real
//! elliptic curves like secp256k1 because it requires _very large_
//! memory and _a lot of_ time. It has O(sqrt(n)) time and space
//! complexity. This would need approximately 10^30 bytes of memory.
//! Therefore, BSGS is not suitable to break DLP in real world.
//! 
//! I would not implement this algorithm because this implementaion
//! has many challenges.

// use crate::breaking_dlp::DiscreteLog;
// use crate::core::curve::Curve;
// use crate::core::point::{Point, CurvePoint};

pub struct BabyStepGiantStep;

// impl<C: Curve> DiscreteLog<C> for BabyStepGiantStep {
//     fn solve(
//             p: &CurvePoint<C>,
//             q: &CurvePoint<C>,
//         ) -> (u16, <C as Curve>::ScalarField) {
        
//     }
// }