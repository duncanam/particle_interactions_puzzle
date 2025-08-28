use anyhow::Context;
use particle_interactions_puzzle::{
    DomainBoundaryLength, Noise, ParticleDistanceThreshold, RelativeTime, Simulation, Speed,
};

const NUM_PARTICLES: usize = 125;
const DOMAIN_BOUNDARY_SIZE: DomainBoundaryLength = DomainBoundaryLength(5.0);
const PARTICLE_DIST_THRESH: ParticleDistanceThreshold = ParticleDistanceThreshold(1.0);
const SPEED: Speed = Speed(1.0);
const TIMESTEP: RelativeTime = RelativeTime(0.25);

const SMALL_NOISE: Noise = Noise(0.1);
const LARGE_NOISE: Noise = Noise(0.9);

fn sim_small_noise() -> anyhow::Result<()> {
    let sim = Simulation::new(
        NUM_PARTICLES,
        DOMAIN_BOUNDARY_SIZE,
        SMALL_NOISE,
        SPEED,
        TIMESTEP,
        PARTICLE_DIST_THRESH,
    )
    .context("small noise simulation failed to initialize")?;

    sim.to_timestepped();

    Ok(())
}

fn main() -> anyhow::Result<()> {
    sim_small_noise()?;

    Ok(())
}
