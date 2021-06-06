//! This module is an attempt to unify the definition of frames across all
//! software, books, lectures, and industry standards that I have encountered
//! during my short existence in the known history of the Universe.

use nalgebra::Vector3;

mod heliocentric;
mod planetocentric;
mod topocentric;

/// This module is purposed towards calculating relative kinematics. I miss the
/// days of dynamics classes 😉.
mod relative_motion {
    use nalgebra::Vector3;

    /// Calculate the relative position of `b` with respect to frame `a`.
    pub fn r_b(r_a: Vector3::<f64>, r_ba: Vector3::<f64>) { r_a + r_ba }

    /// Calculate the relative velocity of `b` with respect to frame `a`.
    pub fn v_b(v_a: Vector3::<f64>, v_ba: Option<Vector3::<f64>>, omega: Option<Vector3::<f64>>, r_ba: Option<Vector3::<f64>>) {
        if v_ba.is_some() {
            v_a + v_ba.unwrap()
        } else if omega.is_some() & r_ba.is_some() {
            v_a + omega.unwrap().cross(&r_ba.unwrap())
        } else {
            panic!("For now, we panic!")
        }
    }

    /// Calculate the relative acceleration of `b` with respect to frame `a`.
    pub fn a_b(a_a: Vector3<f64>, a_ba: Option<Vector3<f64>>, alpha: Option<Vector3<f64>>, r_ba: Option<Vector3<f64>>, omega: Option<Vector3<f64>>) {
        if a_ba.is_some() {
            a_a + a_ba.unwrap()
        } else if alpha.is_some() & r_ba.is_some() & omega.is_some() {
            a_a + alpha.unwrap().cross(&r_ba.unwrap()) + omega.unwrap().cross(&omega.unwrap().cross(&r_ba))
        } else {
            panic!("For now, we panic!")
        }
    }
}

/// Starting point for what defines a reference frame across all platforms,
/// systems, and software encountered.
///
/// (From NAIF: Frames and Coordinate Systems)
/// -  The center of any inertial frame is ALWAYS the solar system barycenter.
/// - The center of a body-fixed frame is the center of the body.
///     - “Body” means a natural body: sun, planet, satellite, comet, asteroid.
///     - The location of the “body” center is specified using an SPK file.
/// - A frame’s center may play a role in specification of states.
///     – The location of the origin cancels out when doing vector subtraction, but
///       the center is used in computing light time to the center of any non-inertial
///       frame being used
struct FrameDefinition {
    vector: [Vector3<f64>;3],
    orient: String,
    orient_id: i32,
    origin: String,
    origin_id: i32
}

/// As described by Curtis, any quantity measured in this frame is "absolute",
/// e.g. "absolute acceleration".
///
/// Inertial
/// – Non-rotating with respect to stars
/// – Non-accelerating origin
///      » Velocity is typically non-zero, but acceleration is negligible
/// – Examples:
///      » J2000 (also known as EME 2000, and is actually ICRF)
///      » ECLIPJ2000
trait InertialFrame {}

/// As described by Curtis once more, any quantity in this frame is "relative".
/// This frame can move by its own accord or it can be attached to a celestial
/// body or spacecraft in an inertial frame.
trait MovingFrame {}





