pub mod brute_force;
pub mod baby_step_giant_step;
pub mod pollards_rho;
#[cfg(test)]
pub mod tests;

use crate::core::curve::Curve;
use crate::core::point::CurvePoint;

// 추상 메서드(=trait)만 선언하고, 하위 파일에서 구체 구현
// 동일한 인터페이스(solve)를 갖되, 여러 알고리즘이 플러그인 형태도 들어와야.
// 하지만, ECDH는 주어진 Curve, Field 타입에 대해 동작하는 단일 프로토콜 구현이 필요했기 떄문에
// 제네릭 구조체 하나만으로 충분
pub trait DiscreteLog<C: Curve> {
    fn solve(
        p: &CurvePoint<C>,
        q: &CurvePoint<C>,
    ) -> (u16, C::ScalarField); // (steps, logarithm)
}