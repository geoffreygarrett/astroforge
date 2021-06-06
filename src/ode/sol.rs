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
    state: DMatrix<T, D>,
    evals: usize,
    error: SVector<T, D>,
}

impl<T, const D: usize> Solution {
    pub fn save_error(&self){
        self.save_error = true;
        self
    }
    pub fn save_state(&self){
        self.save_state = true;
        self
    }
    pub fn save_evals(&self){
        self.save_evals = true;
        self
    }

}