use std::fmt::Display;

use anyhow::bail;

use crate::{
    particle::Particles,
    types::{
        AbsoluteTime, DomainBoundaryLength, Float, Noise, ParticleDistanceThreshold, RelativeTime,
        Speed,
    },
};

// By putting these parameters in their own struct it also makes the copy update more readable and
// easier to maintain
#[derive(Copy, Clone)]
pub(crate) struct SimulationParameters {
    pub(crate) boundary_side_length: DomainBoundaryLength,
    noise: Noise,
    speed: Speed,
    timestep: RelativeTime,
    particle_distance_threshold: ParticleDistanceThreshold,
}

/// A particle interaction simulator
pub struct Simulation {
    particles: Particles,
    current_time: AbsoluteTime,
    pub(crate) params: SimulationParameters,
}

impl Simulation {
    /// Instantiate a new particle simulator with randomized initial conditions
    pub fn new(
        num_particles: usize,
        boundary_side_length: DomainBoundaryLength,
        noise: Noise,
        speed: Speed,
        timestep: RelativeTime,
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
    pub fn to_timestepped(&self) -> Self {
        let particles = self.particles.to_timestepped(
            self.params.particle_distance_threshold,
            self.params.speed,
            self.params.noise,
            self.params.timestep,
            self.params.boundary_side_length,
        );

        let current_time = self.current_time + self.params.timestep;

        Self {
            particles,
            current_time,
            params: self.params,
        }
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=================== Simulation ===================")?;
        writeln!(f, "Current time: {}", self.current_time.0)?;
        writeln!(f, "Particles: {}", self.particles.len())?;
        writeln!(f, "Domain size: {}", self.params.boundary_side_length.0)?;
        writeln!(f, "Timestep: {}", self.params.timestep.0)?;
        writeln!(f, "Noise: {}", self.params.noise.0)?;
        writeln!(f, "Particle speed: {}", self.params.speed.0)
    }
}

/// Storage API for simulation data
pub struct SimulationData {
    pub x: Vec<Float>,
    pub y: Vec<Float>,
    pub u: Vec<Float>,
    pub v: Vec<Float>,
}

impl From<&Simulation> for SimulationData {
    fn from(sim: &Simulation) -> Self {
        let x = sim
            .particles
            .iter()
            .map(|particle| particle.pos_x)
            .collect();

        let y = sim
            .particles
            .iter()
            .map(|particle| particle.pos_y)
            .collect();

        let u = sim
            .particles
            .iter()
            .map(|particle| particle.theta.cos())
            .collect();

        let v = sim
            .particles
            .iter()
            .map(|particle| particle.theta.sin())
            .collect();

        Self { x, y, u, v }
    }
}
