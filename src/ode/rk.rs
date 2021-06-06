use crate::ode::*;
// use nalgebra::Vector2;
// use nalgebra::*;
use nalgebra::{SVector};

/// Integrand type definition.
type Integrand<Y, T> = fn(Y, T) -> Y;


/// Fixed step parameters
pub struct FixedStepODE<Y, T> {
    pub t: T,
    // transient
    pub h: T,
    // step size
    pub y: Y,
    // state
    pub f: Integrand<Y, T>, // state derivative
}

/// RK4 integrator.
/// # Attributes
/// - `s0`: initial state
/// - `t0`: initial transient value
/// - `t1`: final transient value
/// - `sf`: integrand function
/// - `sf`: integrand function
pub struct RK4<Y, T> {
    // pub fixed: FixedStepODE<Y, T>,
    pub k: (Y, Y, Y, Y),
    pub y0: Y,
    pub tspan: [T; 2],
    pub h: T,
    pub f: fn(Y, T) -> Y,
    pub y: Y,
    pub t: T,
}

// pub struct Euler<X, T> {
//     pub y0: X,
//     pub t0: T,
//     pub t1: T,
//     pub h: T,
//     pub f: fn(X, T) -> X,
//     pub y: X,
//     pub t: T,
// }

// impl<X, T> RK4<X, T> {
//     pub fn new(
//
//     )
// }

//
// impl<Y, T> RK4<Y, T> {
//     pub fn new(
//         y0: Y,
//         tspan: [T; 2],
//         h: T,
//         f: Integrand<Y, T>,
//     ) -> RK4<Y, T> {
//         RK4 {
//             y0: y0,
//             tspan: tspan,
//             h: h,
//             f: f,
//             y: y0.clone(),
//             t: tspan[0],
//             k: (
//             y0.clone(),
//             y0.clone(),
//             y0.clone(),
//             y0.clone(),
//             ),
//         }
//     }
// }

trait FixedStepBuilder<T, D> {
    fn y0(&self) -> self {
        self.y0 = y0;
        self
    }
    fn tspan(&self, tspan:) -> self {
        self.tspan = tspan;
    }
}

impl<const D: usize> RK4<SVector::<f64, D>, f64> {
    // pub fn new(
    //     y0: SVector::<f64, D>,
    //     tspan: [f64; 2],
    //     h: f64,
    //     f: Integrand<SVector::<f64, D>, f64>,
    // ) -> RK4<SVector::<f64, D>, f64> {
    //     RK4 {
    //         y0: y0,
    //         tspan: tspan,
    //         h: h,
    //         f: f,
    //         y: y0.clone(),
    //         t: tspan[0],
    //         k: (
    //             y0.clone(),
    //             y0.clone(),
    //             y0.clone(),
    //             y0.clone(),
    //         ),
    //     }
    // }
    pub fn y0 (&self){ }
    self.y = y0;
}

// GOAL <---------------------------
// impl<Y, T> RK4<Y, T> {
//     pub fn new(
//         y0: Y,
//         tspan: [T; 2],
//         h: T,
//         f: Integrand<Y, T>,
//     ) -> RK4<Y, T> {
//         RK4 {
//             y0: y0,
//             tspan: tspan,
//             h: h,
//             f: f,
//             y: y0.clone(),
//             t: tspan[0],
//             k: (
//                 y0.clone(),
//                 y0.clone(),
//                 y0.clone(),
//                 y0.clone(),
//             ),
//         }
//     }
// }

// impl<const D: usize> RK4<SVector<f64, D>, f64> {
//     pub fn new(
//         y0: SVector<f64, D>,
//         t0: f64,
//         t1: f64,
//         h: f64,
//         f: Integrand<SVector<f64, D>, f64>,
//     ) -> RK4<SVector<f64, D>, f64> {
//         RK4 {
//             y0,
//             t0,
//             t1,
//             h,
//             f,
//             y: y0.clone_owned(),
//             t: t0.clone(),
//             k: (
//                 y0.clone_owned(),
//                 y0.clone_owned(),
//                 y0.clone_owned(),
//                 y0.clone_owned(),
//             ),
//         }
//     }
// }

impl<const D: usize> ODESolver<SVector<f64, D>, f64> for RK4<SVector<f64, D>, f64> {
    fn step(&mut self, n: isize) {
        for _ in 0..n {
            self.k.0 = (self.f)(self.y, self.t);
            self.k.1 = (self.f)(&self.y + self.h * &self.k.0 / 2., self.t + self.h / 2.);
            self.k.2 = (self.f)(&self.y + self.h * &self.k.1 / 2., self.t + self.h / 2.);
            self.k.3 = (self.f)(&self.y + self.h * &self.k.2, self.t + self.h);
            self.y += 1. / 6. * self.h * (&self.k.0 + 2. * &self.k.1 + 2. * &self.k.2 + &self.k.3);
            self.t += &self.h;
        }
    }
}
