use anyhow::bail;

use crate::{
    particle::Particles,
    types::{AbsoluteTime, Noise, ParticleDistanceThreshold, RelativeTime, Speed},
};

// By putting these parameters in their own struct it also makes the copy update more readable and
// easier to maintain
#[derive(Copy, Clone)]
struct SimulationParameters {
    boundary_side_length: f64,
    noise: Noise,
    speed: Speed,
    timestep: RelativeTime,
    time_end: AbsoluteTime,
    particle_distance_threshold: ParticleDistanceThreshold,
}

/// A particle interaction simulator
struct Simulation {
    particles: Particles,
    current_time: AbsoluteTime,
    params: SimulationParameters,
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
        particle_distance_threshold: ParticleDistanceThreshold,
    ) -> anyhow::Result<Self> {
        if num_particles == 0 {
            bail!("at least one particle must be simulated");
        }

        let particles = Particles::new(num_particles, boundary_side_length);

        let params = SimulationParameters {
            boundary_side_length,
            noise,
            speed,
            timestep,
            time_end,
            particle_distance_threshold,
        };

        let current_time = AbsoluteTime(0.0);

        Ok(Self {
            particles,
            current_time,
            params,
        })
    }

    /// Update the simulation to new timestep
    fn to_timestepped(&self) -> Self {
        let particles = self.particles.to_timestepped(
            self.params.particle_distance_threshold,
            self.params.speed,
            self.params.noise,
            self.params.timestep,
        );

        let current_time = self.current_time + self.params.timestep;

        Self {
            particles,
            current_time,
            params: self.params,
        }
    }
}
