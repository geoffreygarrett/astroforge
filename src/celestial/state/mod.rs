//!
//!
//! # Notes
//! - Should the distinction between `coordinates` and `elements` be made? This
//!   would make sense since elements are coupled to the parent attracting body,
//!   and do not define
//!
//! ```
//! let keplerian_state =
//! ```


mod keplerian;
mod conversion;
mod cartesian;

use nalgebra::{Vector6, Vector3};

/// ## Design Motivation
/// - **Name**: https://en.wikipedia.org/wiki/Primary_(astronomy)
///     It is made clear that other variants are `gravitational primary`,
///     `primary body` or `central body` however, the `primary` classification
///     provides a better categorisation when dealing with multi hierarchical
///     systems (i.e. Solar System Barycentric (primary),
///     Mars Barycentric (secondary), {Mars, Phobos, Deimos} (tertiaries).
///     This naming scheme will then make the distinction clear when dealing
///     with and designing celestial systems. This requires more development
///     however. This scheme is relational, not absolute in nature.
/// ## Contained structs
/// - GM(f64):
///     * Allows for state conversions (e.g. CartesianCoordinates <-> KeplerianElements)
/// - FrameGM(Frame, f64):
///     * Allows for state conversions (e.g. CartesianCoordinates <-> KeplerianElements)
///     * Allows for frame transformations that differ by Z-rotation (e.g. J2000 <-> J1900)
/// - CelestialBody:
///     * Allows for state conversions (e.g. CartesianCoordinates <-> KeplerianElements)
///     * Allows for frame transformations that differ by Z-rotation (e.g. J2000 <-> J1900)
///     * Allows for frame transformations that differ by plane (within attainable accuracy) (Ecliptic <-> Equatorial)
enum Primary {
    GM(f64),
    FrameGM(Frame, f64),
    CelestialBody,  // Body contains Frame, Gravity and Ephemeris defining the State
}

pub trait State {
    // conversion
    // fn to_cartesian(&self) -> CartesianState;
    // fn to_keplerian(&self) -> KeplerianState;

    // construction
    ///
    /// # Notes
    /// Taking example from `poliastro`, perhaps we should avoid using the
    /// semi-major axis (a) in the case of a parabolic orbit (a=∞), and opt for the semi-minor axis (p).
    fn from_keplerian_elements(p: f64, e: f64, i: f64, laan: f64, argp: f64, nu: f64) -> Self;

    ///
    fn from_state_vectors(r: Vector3::<f64>, v: Vector3::<f64>) -> Self;

    // fn from_equonoctial_elements()
}
