use nalgebra::SVector;
use nalgebra::DMatrix;

/// ```
/// let solution = sol::Solution()  // more saving => less performance
///     .save_error(true)
///     .save_state(true)
///     .save_evals(true);
/// ```
struct Solution<T, D> {
    save_state: bool,
    save_evals: bool,
    save_error: bool,
    state: DMatrix<T>,
    evals: usize,
    error: SVector<T, D>,
}

impl<T, const D: usize> Solution<T, D> {
    pub fn save_error(&mut self) -> &self {
        self.save_error = true;
        self
    }
    pub fn save_state(&mut self) -> &self {
        self.save_state = true;
        self
    }
    pub fn save_evals(&mut self) -> &self {
        self.save_evals = true;
        self
    }
}

/// Trait that defines the behaviour of an ODE solver.
///
/// ```
/// let solution = ode::Solution()  // more saving, less performance
///     .save_error(..)
///     .save_state(..)
///     .save_evals(..);
///
/// let ode = ode::RK4()
///     .set_sol(solution)
///     .set_y(vec![0.,0.])
///     .set_f(|x, _| vec![x[1], x[0] * 2.])
///     .set_h(1.)
///     .set_tspan([0., 100.])
///     // .set_t(0.)  : alternative to tbounds, infers unknown ending bound.
///     // .build()    // is this necessary?
///     .solve();   // only if bounds exist
///
/// println!("{}", saver.state)
/// ```
pub trait ODESolver<X, T> {
    // Step `n` steps. Negative implies backwards. Positive implies forwards.
    fn step(&mut self, n: isize);

    /// This method solves the ODE with the provided `tspan` argument. If this
    /// argument is not provided (i.e. unknown integration bounds), then this
    /// will raise an error.
    fn solve(&mut self);

    // fn set_solution(&mut self, sol: Solution){

    // }
    // fn step_to(&mut self, t: T);
    // fn solve(&mut )

    // Step by a single input `h` step size value.
    // Note: This is probably going to be a private method.
    // fn step_by_step(h: T);

    // Step to a `t` value from current `t`.
    // # Arguments
    // - `t`: transient value to step to.
    // - `exact`: end exactly on provided `t`. If `true`, then last step will be adjusted.
    // fn step_to_value(t: T, exact: bool);

    // Step by `n` steps, with internal `h`.
    // # Arguments
    // - `n`: number of steps to step. Sign defines direction.
    // fn step_by_steps(n: isize);

    // // Get reference to integrand function.
    // fn get_f() -> (fn(X, T) -> X);
    //
    // // Get current step size.
    // fn get_h() -> T;
    //
    // // Get current error estimation.
    // fn get_e() -> X;
}
