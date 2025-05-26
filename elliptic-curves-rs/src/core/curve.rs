use crate::core::field::{PrimeField};
use crate::core::point::Point;
use ark_ff::{PrimeField as ArkPrimeField, BigInteger};

/// PrimeField를 상속받는 Curve
/// Curve (BaseField) -> PrimeField -> Field
/// 아래 추상 메서드를 각 타원 곡선에서 세부 구현
pub trait Curve {
    // Curve를 구현할 때 BaseField라는 이름으로 구체적인 타입을 하나 정해야 하는데,
    // 그 타입은 반드시 PrimeField trait과 ark_ff::PrimeField(=ArkPrimeField) 두
    // trait을 모두 구현해야 한다. 즉, PrimeField의 두 함수
    // from_u64, to_u64와 ArkPrimeField의 함수를 구현해야 한다.
    type BaseField: PrimeField + ArkPrimeField;

    fn add_point(
        p: &Point<Self::BaseField>,
        q: &Point<Self::BaseField>,
    ) -> Point<Self::BaseField>;

    fn double_point(p: &Point<Self::BaseField>) -> Point<Self::BaseField>;

    fn mul_scalar(
        p: &Point<Self::BaseField>,
        scalar: &<Self::BaseField as ArkPrimeField>::BigInt,
    ) -> Point<Self::BaseField> {
        let mut result = Point::infinity();
        let mut acc = p.clone();
        for bit in scalar.to_bits_le() {
            if bit {
                result = Self::add_point(&result, &acc);
            }
            acc = Self::double_point(&acc);
        }
        result
    }
}