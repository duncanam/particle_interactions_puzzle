use std::{collections::VecDeque, fmt::Display};

use anyhow::{anyhow, bail};

use crate::{
    particle::Particles,
    types::{
        AbsoluteTime, DomainBoundaryLength, Float, InstantaneosOrder, Noise,
        ParticleDistanceThreshold, RelativeTime, Speed,
    },
};

/// Beyond this many iterations the computation will fail
const MAX_STATIONARY_ORDER_PARAM_ITERATIONS: usize = 5000;

/// Defines the sliding window size for stationary order parameter
const STATIONARY_ORDER_PARAM_AVG_WINDOWSIZE: usize = 100;

const STATIONARY_ORDER_EPSILON: Float = 0.001;

// By putting these parameters in their own struct it also makes the copy update more readable and
// easier to maintain
#[derive(Copy, Clone)]
pub(crate) struct SimulationParameters {
    pub(crate) boundary_side_length: DomainBoundaryLength,
    pub(crate) noise: Noise,
    pub(crate) speed: Speed,
    pub(crate) timestep: RelativeTime,
    pub(crate) particle_distance_threshold: ParticleDistanceThreshold,
}

/// A particle interaction simulator
pub struct Simulation {
    pub(crate) particles: Particles,
    pub(crate) instantaneous_order: InstantaneosOrder,
    pub(crate) current_time: AbsoluteTime,
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

        let instantaneous_order = particles.compute_instantaneous_order();

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
            instantaneous_order,
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

        let instantaneous_order = particles.compute_instantaneous_order();

        let current_time = self.current_time + self.params.timestep;

        Self {
            particles,
            instantaneous_order,
            current_time,
            params: self.params,
        }
    }

    /// Compute the stationary order parameter, which is the temporal average of the particle
    /// system polarization
    pub fn compute_stationary_order_parameter(&self) -> anyhow::Result<Float> {
        // Get an initial simulation
        let mut sim = self.to_timestepped();

        // This will store a sliding window of our instantaneous orders
        let mut instantaneous_order_window =
            VecDeque::with_capacity(STATIONARY_ORDER_PARAM_AVG_WINDOWSIZE);

        for _ in 0..MAX_STATIONARY_ORDER_PARAM_ITERATIONS {
            sim = sim.to_timestepped();

            // Keep track of the values over time for a sliding average
            if instantaneous_order_window.len() >= STATIONARY_ORDER_PARAM_AVG_WINDOWSIZE {
                instantaneous_order_window.pop_front();
            }
            instantaneous_order_window.push_back(sim.instantaneous_order.0);

            // Sliding average of the parameter over time
            // TODO: could consider a running sum to optimize
            let stationary_order_parameter = instantaneous_order_window.iter().sum::<Float>()
                / instantaneous_order_window.len() as f64;

            // Convergence criteria- also ensure window is full
            if instantaneous_order_window.len() == STATIONARY_ORDER_PARAM_AVG_WINDOWSIZE
                && (sim.instantaneous_order.0 - stationary_order_parameter).abs()
                    <= STATIONARY_ORDER_EPSILON
            {
                return Ok(stationary_order_parameter);
            }
        }

        Err(anyhow!(
            "max iterations (`{}`) reached for stationary order parameter",
            MAX_STATIONARY_ORDER_PARAM_ITERATIONS
        ))
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
