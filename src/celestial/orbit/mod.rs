enum Orbits {
    Hyperbolic,
    Parabolic,
    Elliptical,
    Circular,
}

enum SampleDomain {
    MeanAnomaly,
    Time,
}

pub trait OsculatingOrbit {
    fn to_ephemeris() -> Self;


    fn get_osculating_plane() -> Vector3::<f64> {
        todo!()
    }

    /// Apply an impulsive ΔV burn at current epoch.
    fn apply_impulsive_burn(&self, frame: Frame) -> self {
        match frame {
            Frame::pqw => {}
            Frame::body => {}
            Frame::parent => {}
            Frame::inertial => {}
            _ => { panic!("Frame not recognized!") }
        }
    }

    fn sample(n: u64, domain: Option<SampleDomain>) {
        todo!()
    }

    ///
    fn from_state() -> Self;
    fn from_ephemeris() -> Self;
}