use std::f64::consts::PI;

use crate::{math::Math, types::Float};

/// Represents 360 degrees of spatial rotation available
const MAX_PARTICLE_ANGLE: Float = 2.0 * PI;

/// An individual particle with spatial and rotational state
struct Particle {
    id: usize,
    pos_x: Float,
    pos_y: Float,
    theta: Float,
    phase: Float,
}

// TODO: the random number generators here are instantiating each invocation. Instead, we should
// instantiate it once and pass it through. Benchmarking to determine if this is actually expensive
// will determine how urgent this is.
impl Particle {
    /// Generate a random linear spatial position
    #[inline]
    fn sample_random_linear_position(boundary_side_length: Float) -> Float {
        boundary_side_length * rand::random::<Float>()
    }

    /// Generate a random angular position
    #[inline]
    fn sample_random_angular_position() -> Float {
        MAX_PARTICLE_ANGLE * rand::random::<Float>()
    }

    /// Generate a random phase
    #[inline]
    fn sample_random_phase() -> Float {
        Self::sample_random_angular_position() - PI
    }

    /// Compute the shortest distance between this particle and another particle
    #[inline]
    fn compute_euclidean_distance(&self, other: &Self) -> Float {
        // sqrt((x2-x1)^2 - (y2-y1)^2)
        ((other.pos_x - self.pos_x).square() + (other.pos_y - self.pos_y).square()).sqrt()
    }

    /// Create a new particle with random initialization
    fn new(id: usize, boundary_side_length: Float) -> Self {
        let pos_x = Self::sample_random_linear_position(boundary_side_length);
        let pos_y = Self::sample_random_linear_position(boundary_side_length);
        let theta = Self::sample_random_angular_position();
        let phase = Self::sample_random_phase();

        Self {
            id,
            pos_x,
            pos_y,
            theta,
            phase,
        }
    }

    fn compute_new_theta(&self, particles: &Particles) -> Float {
        todo!()
    }

    /// Get the indices of the closest particles in the swarm given a `distance`.
    ///
    /// # Notes
    /// This function is O(n^2) when called externally on a collection, because it iterates over
    /// each particle checking the distance. If improved performance is required, we can look into
    /// something like a k-d tree alg that will partition the particles into spatial groups and
    /// allow for O(log(n)) lookup. See Rust crate `kiddo`.
    fn compute_idxs_closest(
        &self,
        particles: &Particles,
        distance_threshold: Float,
    ) -> IdxsNeighborParticles {
        IdxsNeighborParticles(
            particles
                .0
                // We iterate over each particle...
                .iter()
                // ...and ensure we aren't including the target particle itself...
                .filter(|particle| self.id != particle.id)
                // ...and then for each particle we compute the euclidean distance between them,
                // filtering out any particles that are further away than our threshold...
                .filter(|particle| self.compute_euclidean_distance(particle) < distance_threshold)
                // ...and then we snag the remaining, filtered indices for particles we know are
                // within the threshold distance...
                .map(|particle| particle.id)
                // ...and place them all together into a collection.
                .collect(),
        )
    }
}

/// Contains all the particles.
// We use Box<[]> over Vec<> because it more clearly conveys that the collection should not be
// mutating.
pub(crate) struct Particles(Box<[Particle]>);

impl Particles {
    /// Create a new collection of particles with random initialization
    pub(crate) fn new(num_particles: usize, boundary_side_length: Float) -> Self {
        Self(
            // For each particle...
            (0..num_particles)
                // ...instantiate a random new one...
                .map(|id| Particle::new(id, boundary_side_length))
                // ...then collect all the particles together into this data structure.
                .collect(),
        )
    }
}

/// Contains the indices for the nearest particles for a given particle
struct IdxsNeighborParticles(Box<[usize]>);
