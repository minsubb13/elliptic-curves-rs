use crate::core::curve::Curve;
use crate::core::point::{Point, CurvePoint};

pub struct Ecdh<C: Curve> {
    _curve: core::marker::PhantomData<C>,
}

impl<C: Curve> Ecdh<C> {
    pub fn generate_keypair() -> (C::BaseField, CurvePoint<C>);
}