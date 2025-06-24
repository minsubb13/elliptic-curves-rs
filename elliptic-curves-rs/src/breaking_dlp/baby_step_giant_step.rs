//! Baby-step Giant-step (BSGS) algorithm for solving the Discrete
//! Logarithm Problem
//! 
//! While BSGS improves upon brute-force from O(n) to O(sqrt(n)) time
//! complexity, it remains impractical for cryptography curves at real
//! elliptic curves like secp256k1 because it requires _very large_
//! memory and _a lot of_ time. It has O(sqrt(n)) time and space
//! complexity. This would need approximately 10^30 bytes of memory.
//! Therefore, BSGS is not suitable to break DLP in real world.

use crate::breaking_dlp::DiscreteLog;
use crate::core::curve::Curve;
use crate::core::point::CurvePoint;

// TODO: Implement the Baby-step Giant-step (BSGS) algorithm for
// solving the Discrete Logarithm Problem. This requires defining
// the necessary imports and implementing the `DiscreteLog` trait for
// the `BabyStepGiantStep` struct. Note that this algorithm is
// currently impractical for real-world cryptographic curves due to its
// high memory and time complexity.
pub struct BabyStepGiantStep;

impl<C: Curve> DiscreteLog<C> for BabyStepGiantStep {
    fn solve(
            p: &CurvePoint<C>,
            q: &CurvePoint<C>,
        ) -> (u64, <C as Curve>::ScalarField) {
        unimplemented!()
    }
}