use std::f64::consts::PI;

/// Represents 360 degrees of spatial rotation available
const MAX_PARTICLE_ANGLE: f64 = 2.0 * PI;

/// An individual particle with spatial and rotational state
struct Particle {
    pos_x: f64,
    pos_y: f64,
    omega: f64,
}

impl Particle {
    /// Generate a random linear spatial position
    #[inline]
    fn sample_random_linear_position(boundary_side_length: f64) -> f64 {
        boundary_side_length * rand::random::<f64>()
    }

    /// Generate a random angular position
    #[inline]
    fn sample_random_angular_position() -> f64 {
        MAX_PARTICLE_ANGLE * rand::random::<f64>()
    }

    fn new(boundary_side_length: f64) -> Self {
        let pos_x = Self::sample_random_linear_position(boundary_side_length);
        let pos_y = Self::sample_random_linear_position(boundary_side_length);
        let omega = Self::sample_random_angular_position();

        Self {
            pos_x,
            pos_y,
            omega,
        }
    }
}

/// Contains all the particles.
// We use Box<[]> over Vec<> because it more clearly conveys that the collection should not be
// mutating.
pub(crate) struct Particles(Box<[Particle]>);

impl Particles {
    fn new(num_particles: usize, boundary_side_length: f64) -> Self {
        todo!()
    }
}
