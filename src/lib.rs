use pyo3::prelude::*;

mod math;
mod particle;
mod simulation;
mod types;

// Exports for pure Rust use
pub use simulation::{Simulation, SimulationData};
pub use types::{
    AbsoluteTime, DomainBoundaryLength, Float, Noise, ParticleDistanceThreshold, RelativeTime,
    Speed,
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
    /// Construct a new particle Simulator
    #[new]
    fn new(
        num_particles: usize,
        boundary_side_length: Float,
        noise: Float,
        speed: Float,
        timestep: Float,
        particle_distance_threshold: Float,
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

    /// Timestep the simulation
    fn to_timestepped(&self) -> Self {
        Self(self.0.to_timestepped())
    }

    #[pyo3(name = "__repr__")]
    fn repr(&self) -> String {
        self.0.to_string()
    }

    fn get_data(&self) -> PySimulationData {
        PySimulationData((&self.0).into())
    }

    #[getter]
    fn boundary_side_length(&self) -> Float {
        self.0.params.boundary_side_length.0
    }

    #[getter]
    fn current_time(&self) -> Float {
        self.0.current_time.0
    }
}

#[pyclass(name = "SimulationData")]
struct PySimulationData(SimulationData);

#[pymethods]
impl PySimulationData {
    #[getter]
    fn x(&self) -> Vec<Float> {
        // Must clone because Python has no concept of ownership lol
        self.0.x.clone()
    }

    #[getter]
    fn y(&self) -> Vec<Float> {
        // Must clone because Python has no concept of ownership lol
        self.0.y.clone()
    }

    #[getter]
    fn u(&self) -> Vec<Float> {
        // Must clone because Python has no concept of ownership lol
        self.0.u.clone()
    }

    #[getter]
    fn v(&self) -> Vec<Float> {
        // Must clone because Python has no concept of ownership lol
        self.0.v.clone()
    }
}
