use crate::{
    particle::Particles,
    types::{AbsoluteTime, Noise, RelativeTime, Speed},
};

/// A particle interaction simulator
struct Simulation {
    particles: Particles,
    boundary_side_length: f64,
    noise: Noise,
    speed: Speed,
    timestep: RelativeTime,
    time_end: AbsoluteTime,
}

impl Simulation {
    /// Instantiate a new particle simulator with randomized initial conditions
    fn new(
        num_particles: usize,
        boundary_side_length: f64,
        noise: Noise,
        speed: Speed,
        timestep: RelativeTime,
        time_end: AbsoluteTime,
    ) -> Self {
        let particles = Particles::new(num_particles, boundary_side_length);

        Self {
            particles,
            boundary_side_length,
            noise,
            speed,
            timestep,
            time_end,
        }
    }
}
