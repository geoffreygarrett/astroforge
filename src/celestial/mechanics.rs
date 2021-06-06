use crate::consts::*;

use std::f64::consts::PI;
use nalgebra::{Vector3};

macro_rules! km {
    ($a:expr)=>{{ 1e3 * $a }}
}

macro_rules! deg {
    ($a:expr)=>{{ $a / 180. * PI }}
}


/// Compute gravitational acceleration.
/// # Arguments
/// - `GM` gravitational parameter of body exerting acceleration
/// - `r1` position of body exerting acceleration
/// - `r2` position of body subject acceleration
/// # Returns
/// - acceleration exerted on second body
#[allow(non_snake_case)]
pub fn a_point_mass(GM: f64, r1: &Vector3::<f64>, r2: &Vector3::<f64>) -> Vector3::<f64> {
    let r = r2 - r1;
    -GM * (r) / (r.norm().powi(3))
}


/// Calculates escape velocity.
///
/// # Arguments
/// * `M` - Mass of the body at the focal point
/// * `r` - Distance of orbiter from the focal point
/// # Notes
/// In physics (specifically, celestial mechanics), escape velocity is
/// the minimum speed needed for a free, non-propelled object to escape
/// from the gravitational influence of a massive body, that is, to
/// eventually reach an infinite distance from it.
/// $$V_{escape}\geqslant\sqrt{\frac{2GM}{r}}$$
///
/// # Examples
/// ```
/// use astroforge_celestial::prelude::*;
/// use astroforge_celestial::consts::*;
/// use astroforge_celestial::mechanics::*;
/// let v : f64 = v_circular(
///     EARTH_MASS,
///     EARTH_RADIUS + km!(150.));
/// ```
#[allow(non_snake_case)]
pub fn v_escape(M: f64, r: f64) -> f64 {
    (2. * G * M / r).sqrt()
}

/// Calculates the velocity of a circular orbit.
///
/// # Arguments
/// * `G` - Universal gravitational constant
/// * `M` - Mass of the body at the focal point
/// * `r` - Distance of orbiter from the focal point
#[allow(non_snake_case)]
pub fn v_circular(M: f64, r: f64) -> f64 {
    (G * M / r).sqrt()
}

/// Calculates the orbital period.
///
/// # Arguments
/// * `a` - Semi-major axis of the conic
/// * `GM` - Gravitational parameter of the body at the focal point
/// * `r` - Orbital distance from the focal point
#[allow(non_snake_case)]
pub fn t_orbit(a: f64, GM: f64) -> f64 {
    2. * PI * ((a.powi(3)) / GM).sqrt()
}

/// Calculates the orbital distance.
///
/// # Arguments
/// * `a` - Semi-major axis of the conic
/// * `e` - Eccentricity of the conic
/// * `theta` - True anomaly of the orbital position
pub fn r_orbit(a: f64, e: f64, theta: f64) -> f64 {
    a * (1. - e.powi(2)) / (1. + e * theta.cos())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test escape velocity.
    fn test_v_escape() {

        // Test with Earth constants.
        let v_escape = v_escape(
            EARTH_MASS,
            EARTH_RADIUS + km!(1000.));

        println!("v_escape: {}", v_escape);
    }


    #[test]
    /// Test escape velocity.
    fn test_v_circular() {

        // Test with Earth constants.
        let v_circular = v_circular(
            EARTH_MASS,
            EARTH_RADIUS + km!(1000.));

        println!("v_circular: {}", v_circular);
    }

    #[test]
    /// Test angle calculations.
    fn test_r_orbit() {

        // Test with Earth constants.
        let r_orbit = r_orbit(
            EARTH_RADIUS + km!(1000.),
            0.,
            deg!(0.5));

        println!("r_orbit: {}", r_orbit);
    }

    #[test]
    /// Test angle calculations.
    fn test_t_orbit() {

        // Test with Earth constants.
        let t_orbit = t_orbit(
            EARTH_RADIUS + km!(100.),
            EARTH_GM);

        println!("t_orbit: {}", t_orbit);
    }

    #[test]
    /// Test angle calculations.
    fn test_a_point_mass() {

        // Test with Earth constsants.
        let a_point_mass = a_point_mass(
            EARTH_GM,
            &Vector3::new(km!(6700.), 0., 0.),
            &Vector3::new(0., 0., km!(6700.)),
        );

        println!("t_orbit: {}", a_point_mass);
    }
}
