
trait EphemerisModel {
    fn get_keplerian_state(&self, t: T) -> KeplerianState;
    fn get_cartesian_state(&self, t: T) -> CartesianState;
    fn get_state_vectors(&self, t: T) -> StateVectors;
    fn from_spice(n: &str) -> Self;
}
