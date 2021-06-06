#[cfg(feature = "ode")]
//! The `ode` module includes all currently implemented ODE solvers. This
//! feature is automatically enabled with the `simulation` feature.
pub mod ode;

#[cfg(feature = "celestial")]
//! The `celestial` module is part of the default modules of `astroforge`.
//! This module includes general celestial mechanics equations.
pub mod celestial;

#[cfg(feature = "simulation")]
//! The `celestial` module is part of the default modules of `astroforge`.
//! This module includes general celestial mechanics equations.
pub mod simulation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
