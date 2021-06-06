#[cfg(test)]
mod tests {
    // use super::*;
    use crate::*;
    use crate::nalgebra::Vector6;
    use crate::ode::ODESolver;
    use nalgebra::Vector2;

    #[test]
    fn test_rk4() {
        let y0 = Vector2::<f64>::new(0., 0.);
        let t0 = 0.;
        let t1 = 100.;
        let h = 1.;

        /// Point accelerating at 2. m/s^2
        fn dy(s: Vector2<f64>, _: f64) -> Vector2<f64> {
            Vector2::<f64>::new(s[1], 2.)
        }

        let mut integrator = rk::RK4::new(y0, [t0, t1], h, dy);

        integrator.step(20);

        println!("{:?}", integrator.y);
        // assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_orbit() {
        let y0 = Vector6::<f64>::new(6450e3, 0., 0., 0., 7.66e3, 0.);
        let t0 = 0.;
        let t1 = 100.;
        let h = 100.;
        // let gm = 3.9860044188E14;

        /// Point accelerating at 2. m/s^2
        fn earth_point_mass(s: Vector6<f64>, _: f64) -> Vector6<f64> {
            let gm = 3.9860044188E14;
            let r = s.rows_range(..3);
            let v = s.rows_range(3..);
            let a = -gm / r.norm().powi(3) * r;
            println!("[{}, {}, {}],", s[0], s[1], s[2]);
            Vector6::<f64>::new(v[0], v[1], v[2], a[0], a[1], a[2])
        }

        let mut integrator = rk::RK4::new(y0, [t0, t1], h, earth_point_mass);

        integrator.step((92.68 * 60. / h) as isize);

        println!("{:?}", integrator.y);
        // assert_eq!(2 + 2, 4);
    }
}
