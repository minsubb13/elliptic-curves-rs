use crate::core::curve::Curve;
use crate::core::point::CurvePoint;

use ark_ff::{BigInteger, Field, PrimeField, Zero};
use ark_std::{
    UniformRand,
    rand::thread_rng
};

pub struct Ecdsa<C: Curve> {
    _curve: std::marker::PhantomData<C>,
}

impl<C: Curve> Ecdsa<C> {
    pub fn signing_message(
        private_key: &C::ScalarField,
        z: C::ScalarField,
    ) -> (C::ScalarField, C::ScalarField) {
        let mut rng = thread_rng();
        let k = C::ScalarField::rand(&mut rng);
        let generator = C::generator();

        let r: C::ScalarField = loop {
            let p = generator.mul_scalar(&k);
            if let Some(x_base) = p.inner.x() {
                let repr = x_base.into_bigint();
                let bytes = repr.to_bytes_be();

                let r_candidate = C::ScalarField::from_be_bytes_mod_order(&bytes);

                if r_candidate != C::ScalarField::zero() {
                    break r_candidate;
                }
            }
        };

        let s: C::ScalarField = loop {
            let k_inverse = k.inverse();
            if let Some(element) = k_inverse {
                let temp_term = z + r * private_key;
                let right_term = element * temp_term;

                if right_term != C::ScalarField::zero() {
                    break right_term;
                }
            }
        };

        (r, s)        
    }

    pub fn verifying_message(
        other_public_key: &CurvePoint<C>,
        z: C::ScalarField,
        signature: &(C::ScalarField, C::ScalarField), // (r, s)
    ) -> bool {
        let (r, s) = *signature;
        let g = C::generator();

        if let Some(s_inv) = s.inverse() {
            let u1 = s_inv * z;
            let u2 = s_inv * r;

            let p1 = g.mul_scalar(&u1);
            let p2 = other_public_key.mul_scalar(&u2);
            let p = p1.add(&p2);

            if let Some(x_base) = p.inner.x() {
                let bytes = x_base
                    .into_bigint()
                    .to_bytes_be();
                let x_scalar = C::ScalarField::from_be_bytes_mod_order(&bytes);
                
                return x_scalar == r;
            }
        }

        // if s.inverse() was None, or P was infinity, or x_conversion failed
        false
    }
}
