use std::fmt::Debug;

pub trait Field:
    Copy
    + Clone
    + PartialEq
    + Debug
{
    fn zero() -> Self;
    fn one() -> Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn inv(&self) -> Self;
    fn neg(&self) -> Self;
}

/// Field를 상속받는 PrimeField
/// PrimeField -> Field
pub trait PrimeField: Field {
    fn from_u64(n: u64) -> Self;
    // fn to_u64(self) -> u64;
}
