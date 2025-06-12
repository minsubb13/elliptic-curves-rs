use std::iter::Scan;

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
        let generator = C::generator();

        loop {
            let k = C::ScalarField::rand(&mut rng);
            let p = generator.mul_scalar(&k);
            
            let r = match p.inner.x() {
                Some(x_base) => {
                    let bytes = x_base.into_bigint().to_bytes_be();
                    let r_candinate = C::ScalarField::from_be_bytes_mod_order(&bytes);
                    if r_candinate.is_zero() {
                        continue;
                    }
                    r_candinate
                }
                None => continue,
            };

            if let Some(k_inv) = k.inverse() {
                let s = k_inv * (z + r * private_key);

                if !s.is_zero() {
                    return (r, s);
                }
            }
        }
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
