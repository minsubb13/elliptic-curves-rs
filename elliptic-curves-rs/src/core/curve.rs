use crate::core::field::{PrimeField};
use crate::core::point::{Point, CurvePoint};

use ark_ff::{PrimeField as ArkPrimeField};

/// PrimeField를 상속받는 Curve
/// Curve (BaseField) -> PrimeField -> Field
/// 아래 추상 메서드를 각 타원 곡선에서 세부 구현
pub trait Curve {
    // Curve를 구현할 때 BaseField라는 이름으로 구체적인 타입을 하나 정해야 하는데,
    // 그 타입은 반드시 PrimeField trait과 ark_ff::PrimeField(=ArkPrimeField) 두
    // trait을 모두 구현해야 한다. 즉, PrimeField의 두 함수
    // from_u64, to_u64와 ArkPrimeField의 함수를 구현해야 한다.
    type BaseField: PrimeField + ArkPrimeField;
    type ScalarField: ArkPrimeField;

    fn a() -> Self::BaseField;
    fn b() -> Self::BaseField;

    // 곡선 타입이 compile time에 확정된 상태여야만 이 메서드를 쓸 수 있다.
    // `where Self: Sized;`를 통해 컴파일러는 generator 호출 시점에 Self의 크기를 알 수 있다.
    fn generator() -> CurvePoint<Self>
    where
        Self: Sized;

    fn order() -> <Self::ScalarField as ArkPrimeField>::BigInt;

    fn add_point(
        p: &Point<Self::BaseField>,
        q: &Point<Self::BaseField>,
    ) -> Point<Self::BaseField>;

    fn double_point(p: &Point<Self::BaseField>) -> Point<Self::BaseField>;

    fn mul_scalar(
        p: &Point<Self::BaseField>,
        scalar: &Self::ScalarField,
    ) -> Point<Self::BaseField>;
}
