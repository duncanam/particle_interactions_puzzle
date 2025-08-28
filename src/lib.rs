use pyo3::prelude::*;

mod math;
mod particle;
mod simulation;
mod types;

// Exports for pure Rust use
pub use simulation::Simulation;
pub use types::{
    AbsoluteTime, DomainBoundaryLength, Noise, ParticleDistanceThreshold, RelativeTime, Speed,
};

#[pymodule]
fn particle_interactions_puzzle(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySimulation>()?;

    Ok(())
}

#[pyclass(name = "Simulation")]
struct PySimulation(Simulation);

#[pymethods]
impl PySimulation {
    #[new]
    fn new(
        num_particles: usize,
        boundary_side_length: f64,
        noise: f64,
        speed: f64,
        timestep: f64,
        particle_distance_threshold: f64,
    ) -> PyResult<Self> {
        let boundary_side_length = DomainBoundaryLength(boundary_side_length);
        let noise = Noise(noise);
        let speed = Speed(speed);
        let timestep = RelativeTime(timestep);
        let particle_distance_threshold = ParticleDistanceThreshold(particle_distance_threshold);

        Ok(Self(Simulation::new(
            num_particles,
            boundary_side_length,
            noise,
            speed,
            timestep,
            particle_distance_threshold,
        )?))
    }

    fn to_timestepped(&self) -> Self {
        Self(self.0.to_timestepped())
    }

    fn thing(&self) -> String {
        "thing".to_string()
    }

    #[pyo3(name = "__repr__")]
    fn repr(&self) -> String {
        self.0.to_string()
    }
}
