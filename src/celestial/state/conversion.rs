use nalgebra::{Vector3, Vector6};
use std::f64::consts::PI;

macro_rules! rad2deg {
    ($a:expr)=>{{ $a/PI * 180. }}
}

/// Function to convert Cartesian to Keplerian elements.
/// ````
/// use nalgebra::Vector3;
/// let r = Vector3::<f64>::new(8000e3, 2000e3, 1000e3);
/// let v = Vector3::<f64>::new(1e3, 2e3, 3e3);
/// let result = rv2ke(3.9860044188E14, r, v);
/// ````
fn rv2ke(
    mu: f64,
    r_vec: Vector3<f64>,
    v_vec: Vector3<f64>,
    // tol: f64
) -> (f64, f64, f64, f64, f64, f64) {

    // Construct position vector.
    let r = r_vec.norm();

    // Construct velocity vector.
    let v = v_vec.norm();

    // Define y unit vector.
    let k_vec = Vector3::new(0., 0., 1.);

    // Calculate angular momentum vector.
    let h_vec = r_vec.cross(&v_vec);
    let h = h_vec.norm();

    // Calculate eccentricity vector.
    let e_vec = v_vec.cross(&h_vec) / mu - r_vec / r_vec.norm();
    let e = e_vec.norm();

    //  Determine the vector n pointing towards the ascending node
    let n_vec = k_vec.cross(&h_vec);
    let n = n_vec.norm();

    // Determine the true anomaly ν [rad]
    let nu: f64;
    if r_vec.dot(&v_vec) >= 0. {
        nu = (e_vec.dot(&r_vec) / (e * r)).acos();
    } else {
        nu = 2. * PI - (e_vec.dot(&r_vec) / (e * r)).acos();
    }

    // Determine the orbital inclination i [rad]
    let i = (h_vec[2] / h).acos();

    // Calculate specific mechanical energy.
    let E = 2. * ((nu / 2.).tan() / ((1. + e) / (1. - e)).sqrt()).atan();

    let mut laan = (n_vec[0] / n).acos();
    if n_vec[1] >= 0. {} else { laan = 2. * PI - laan }

    let mut argp = (n_vec.dot(&e_vec) / (n * e)).acos();
    if e_vec[2] >= 0. {} else { argp = 2. * PI - argp }

    let M = E - e * E.sin();

    let a = 1. / (2. / r - v.powi(2) / mu);

    // Return the elements.
    (a, e, i, laan, argp, nu)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let r = Vector3::<f64>::new(8000e3, 2000e3, 1000e3);
        let v = Vector3::<f64>::new(1e3, 2e3, 3e3);
        let result = rv2ke(3.9860044188E14, r, v);
        assert_eq!(result, (
            4862658.706391304,
            0.7859376155840834,
            1.0306017747970404,
            0.17219081452293963,
            3.439992552071226,
            2.9840317797656732
        ));
    }
}