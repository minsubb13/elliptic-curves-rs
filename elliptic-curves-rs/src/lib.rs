pub mod finite_fields;
pub mod point;
pub mod curve_fn;
pub mod curves;

pub use point::Point;
pub use curve_fn::{EllipticCurve, EllipticCurveError};

// pub mod curves {
//     real elliptic curve domain parameter
// }