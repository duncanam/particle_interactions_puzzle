use argmin::{
    core::{CostFunction, Executor},
    solver::neldermead::NelderMead,
};

use anyhow::{Context, anyhow};

use crate::{
    DomainBoundaryLength, Float, Noise, ParticleDistanceThreshold, RelativeTime, Simulation, Speed,
};

/// Defines how the left and right points are selected for the critical noise optimizer
const NOISE_CRITICAL_OFFSET: Float = 0.05;

/// Defines how large of a difference in right/left stationary order params define criticality
const CRITICAL_STATIONARY_ORDER_PARAM_DELTA: Float = 0.25;

const MAX_NELDER_MEAD_ITERATIONS: u64 = 100;

/// Optimize speed and the radius threshold to find a target noise
pub fn optimize_for_critical_noise(
    num_particles: usize,
    boundary_side_length: DomainBoundaryLength,
    timestep: RelativeTime,
    noise_critical_target: Noise,
) -> anyhow::Result<(ParticleDistanceThreshold, Speed)> {
    // This will be our residual function
    let cost = SimOptimizerCost::new(
        num_particles,
        boundary_side_length,
        timestep,
        noise_critical_target,
    );

    // Initial conditions for the Nelder-Mead simplex to meander about. These are rough
    // and it's very robust. However, this is still quite impactful, especially for many
    // local minima.
    // TODO: global const these
    let initial_simplex = vec![vec![1.0, 1.0], vec![1.5, 1.0], vec![1.0, 1.5]];

    let solver = NelderMead::new(initial_simplex)
        .with_sd_tolerance(0.0001)
        .context("could not initialize NelderMead with tolerance")?;

    let result = Executor::new(cost, solver)
        .configure(|state| state.max_iters(MAX_NELDER_MEAD_ITERATIONS))
        .run()
        .context("run failed")?;

    let best_param = result
        .state
        .best_param
        .ok_or_else(|| anyhow!("optimizer found no best parameters"))?;
    let best_particle_distance_threshold = ParticleDistanceThreshold(best_param[0]);
    let best_speed = Speed(best_param[1]);

    Ok((best_particle_distance_threshold, best_speed))
}

/// This defines the cost function for Nelder-Mead to optimize against
struct SimOptimizerCost {
    num_particles: usize,
    boundary_side_length: DomainBoundaryLength,
    timestep: RelativeTime,
    noise_critical_left: Noise,
    noise_critical_right: Noise,
}

impl SimOptimizerCost {
    fn new(
        num_particles: usize,
        boundary_side_length: DomainBoundaryLength,
        timestep: RelativeTime,
        noise_critical_target: Noise,
    ) -> Self {
        // We set up the optimizer by considering target noise on either side of the target,
        // through an offset. This is roughly approximate, and can be made better through actually
        // solving the \psi routine directly in the optimizer but IMO this is "good enough"
        let noise_critical_left = noise_critical_target * (1.0 - NOISE_CRITICAL_OFFSET);
        let noise_critical_right = noise_critical_target * (1.0 + NOISE_CRITICAL_OFFSET);

        Self {
            num_particles,
            boundary_side_length,
            timestep,
            noise_critical_left,
            noise_critical_right,
        }
    }
}

impl CostFunction for SimOptimizerCost {
    type Param = Vec<Float>;
    type Output = Float;

    /// Function to be minimized
    fn cost(&self, param: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        let particle_distance_threshold = ParticleDistanceThreshold(param[0]);
        let speed = Speed(param[1]);

        let sim_left = Simulation::new(
            self.num_particles,
            self.boundary_side_length,
            self.noise_critical_left,
            speed,
            self.timestep,
            particle_distance_threshold,
        )
        .context("could not instantiate noise-critical-left simulation in optimizer")?;

        let sim_right = Simulation::new(
            self.num_particles,
            self.boundary_side_length,
            self.noise_critical_right,
            speed,
            self.timestep,
            particle_distance_threshold,
        )
        .context("could not instantiate noise-critical-right simulation in optimizer")?;

        let stationary_order_param_left = sim_left
            .compute_stationary_order_parameter()
            .context("cound not compute stationary order param for noise-critical-left simulation in optimizer")?;

        let stationary_order_param_right = sim_right
            .compute_stationary_order_parameter()
            .context("cound not compute stationary order param for noise-critical-right simulation in optimizer")?;

        let delta_stationary_order_param =
            stationary_order_param_left - stationary_order_param_right;

        // If the left and right points for the stationary order param show significant change,
        // this likely means we've hit our target point
        let residual = (delta_stationary_order_param - CRITICAL_STATIONARY_ORDER_PARAM_DELTA).abs();

        Ok(residual)
    }
}
