use crate::core::curve::Curve;
use crate::core::point::CurvePoint;

use ark_ff::Zero;
use ark_std::{
    UniformRand,
    rand::thread_rng,
};

/// ECDH 프로토콜은 상태를 가지기보단 연산을 수행하는 역할이므로, 상태를 저장하는
/// 필드가 필요 없음. 이럴 때 PhantomData를 사용하는 것이 rust의 일반적인 패턴
/// PhantomData<C>는 구조체가 실제로 C 타입의 데이터를 저장하진 않지만, 마치 저장하는
/// 것처럼 C 타입과 연관되도록 만들어줌
/// 덕분에 Ecdh<Curve1>와 Ecdh<Curve2>가 구분됨
pub struct Ecdh<C: Curve> {
    // _로 시작: 이 값이 실제로 사용되지 않는 필드
    // 사용되지 않음에도 불구하고 존재해야만 하는 표식 필드
    // 코드를 한번만 작성하되, C:::generator() 등의 곡선 구체 타입별 정적 메서드를
    // 호출하여 ECDH를 수행하려는 목적
    _curve: std::marker::PhantomData<C>,
}

impl<C: Curve> Ecdh<C> {
    pub fn generate_keypair() -> (C::ScalarField, CurvePoint<C>) {
        let mut rng = thread_rng();
        let g: CurvePoint<C> = C::generator();

        // private key is a random integer d chosen from {1, ..., n-1}
        // (n is the order of the subgroup)
        let private_key = loop {
            let d = C::ScalarField::rand(&mut rng);
            if d != C::ScalarField::zero() { break d }
        };
        // public key is the point H = dG
        let public_key = g.mul_scalar(&private_key);

        (private_key, public_key)
    }

    pub fn compute_shared_secret(
        private_key: &C::ScalarField,
        other_public_key: &CurvePoint<C>,
    ) -> CurvePoint<C> {
        let shared = other_public_key.mul_scalar(&private_key);

        shared
    }
}
