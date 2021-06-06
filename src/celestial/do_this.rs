struct KeplerianState {}

struct CartesianState {}

struct StateVectors {}

trait Ephemeris {
    fn get_keplerian_state(&self, t: T) -> KeplerianState;
    fn get_cartesian_state(&self, t: T) -> CartesianState;
    fn get_state_vectors(&self, t: T) -> StateVectors;
    fn from_spice(n: &str) -> Self;
}


pub trait CelestialBuilder {
    fn set_ephemeris(&self) -> self;
    fn set_atmosphere(&self) -> self;
    fn set_gravity(&self) -> self;
    fn set_magnetosphere(&self) -> self;
    fn set_rotation(&self) -> self;
    fn set_shape(&self) -> self;
    fn build(&self) -> self;
}

struct CelestialBody<E: Ephemeris, A: Atmosphere, R: Rotation, S: Shape, M: Magnetosphere, L: Links, G: Gravity> {
    ephemeris: Option<E>,
    atmosphere: Option<A>,
    rotation: Option<R>,
    shape: Option<S>,
    magnetosphere: Option<M>,
    links: Option<L>,
    gravity: Option<G>,
}

impl CelestialBuilder for CelestialBody<E, A, R, S, M, L, G> {
    fn set_ephemeris(&self) -> self {
        todo!()
    }

    fn set_atmosphere(&self) -> self {
        todo!()
    }

    fn set_gravity(&self) -> self {
        todo!()
    }

    fn set_magnetosphere(&self) -> self {
        todo!()
    }

    fn set_rotation(&self) -> self {
        todo!()
    }

    fn set_shape(&self) -> self {
        todo!()
    }

    fn build(&self) -> self {
        todo!()
    }
}

fn main() {

    // Create default Sun.
    let sun = library::Sun::default();

    // class Parent(Child) {
    //     Parent(arg1, arg2, arg3)
    //         .child_arg1(arg1)
    //         .child_arg2(arg2)
    //         .child_arg3(arg3)
    // }
    /// Create custom Earth.
    let earth = CelestialBodyBuilder
        // .set_frame(Frame::J2000)
        .ephemeris(Ephemeris::from_spice("Earth").to_frame(Frame::J2000))
        .atmosphere(Atmosphere::from_nrlmsise00())
        .shape(Shape::sphere::new(6700e3))
        .rotation(Rotation::from_spice("Earth"))
        .magnetosphere(Magnetosphere::from_yaml("emm99.txt"))
        .eom(Propagation::cowel())
        .parent(sun)
        .build();

    let spacecraft = Spacecraft
        // .set_acceleration(Acceleration::point_mass(earth))
        // .set_acceleration(Acceleration::aerodynamic(earth))
        // .set_acceelration()
        .set_accelerations(vec![
            Acceleration::point_mass(earth),
            Acceleration::aerodynamic(earth),
            Acceleration::srp(sun)
        ])
        .set_state(State::Keplerian::new(6e6, 0., 0., 0., 0., 0.))
        .validate();

    let solution = ode::Solution()  // more saving, less performance
        .save_error(..)
        .save_state(..)
        .save_evals(..);

    // let accelerations
    let ode = ode::RK4()
        .set_sol(solution)
        .set_y(vec![0., 0.])
        .set_f(|x, _| vec![x[1], x[0] * 2.])
        .set_h(1.)
        .set_tbounds([0., 100.])
        // .set_t(0.)  : alternative to tbounds, infers unknown ending bound.
        // .build()    // is this necessary?
        // .solve()    // maybe not in most cases useful
        .validate();   // only if bounds exist

    loop {
        if spacecraft.distance_to(earth) <= earth.radius {
            println!("Collision with Earth has occured!");
            break;
        }
        if ode.get_conditioning() <= 1e-8 {
            println!("Poor conditioning of ODE!");
            break;
        }
        if ode.is_terminal() {
            println!("Simulation has reached upper ODE time span!");
            break;
        }

        ode.step();
    }
}
