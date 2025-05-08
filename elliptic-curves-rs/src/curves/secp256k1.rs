use crate::curve_fn::{EllipticCurve, EllipticCurveError};
use crate::finite_fields::secp256k1::Fq;
use crate::point::{Point};

pub struct Secp256k1;

impl Secp256k1 {
    pub fn new() -> Result<EllipticCurve<Fq>, EllipticCurveError> {
        let a = Fq::from(0u64);
        let b = Fq::from(7u64);
        
        let g_x_hex = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
        let g_y_hex = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
        
        let g = match Secp256k1Point::from_hex(g_x_hex, g_y_hex) {
            Some(point) => point,
            None => return Err(EllipticCurveError::InvalidCurveParameters),
        };
        
        let n_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
        let n = Fq::from_hex(n_hex)
            .ok_or(EllipticCurveError::InvalidCurveParameters)?;
        
        EllipticCurve::new(a, b, g, n)
    }
}