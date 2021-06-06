use uom::si::f64::{Velocity, Length};
// use crate::state::Attractor;

/// Cartesian element system.
#[derive(Clone)]
pub struct Cartesian {
    // attractor: Attractor,
    x: Length,
    y: Length,
    z: Length,
    vx: Velocity,
    vy: Velocity,
    vz: Velocity,
}
//
// impl Cartesian {
//     pub fn new()
// }