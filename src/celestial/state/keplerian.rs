use uom::si::f64::*;
// use crate::state::Attractor;

/// Keplerian element system.
#[derive(Clone)]
pub struct Keplerian {
    attractor: Attractor,
    sma: Length,
    ecc: Ratio,
    inc: Angle,
    argp: Angle,
    laan: Angle,
    nu: Angle,
}