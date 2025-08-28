use std::f64::consts::PI;

use num::Complex;

use crate::{
    math::Math,
    types::{DomainBoundaryLength, Float, Noise, ParticleDistanceThreshold, RelativeTime, Speed},
};

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
    fn sample_random_linear_position(boundary_side_length: DomainBoundaryLength) -> Float {
        boundary_side_length.0 * rand::random::<Float>()
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
    fn new(id: usize, boundary_side_length: DomainBoundaryLength) -> Self {
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

    /// Compute a new theta
    ///
    /// # Notes
    /// This represents Equation 1
    fn compute_new_theta(
        &self,
        particles: &Particles,
        distance_threshold: ParticleDistanceThreshold,
        speed: Speed,
        noise: Noise,
    ) -> Float {
        let idxs_closest = self.compute_idxs_closest(particles, distance_threshold);

        // This is "|s_i(t)|"
        let num_closest = idxs_closest.0.len();

        // This is \sum_{j in s_i(t)}(...) in equation 1
        // Note: calling map 2x for readability instead of chonky inline block
        let summed_terms: Complex<_> = idxs_closest
            .0
            // Iterate over all the closest particles "j"...
            .into_iter()
            // ...then grab their actual particle struct references for use...
            .map(|idx| &particles.0[idx])
            // ...then compute each sum term...
            .map(|particle| {
                // v * e^{i \theta_j(t)}
                let sum_term_1 = speed.0 * Complex::new(particle.theta, 1.0);

                // \eta * e^{i \xi_n(t)}
                let sum_term_2 = noise.0 * Complex::new(particle.phase, 1.0);

                sum_term_1 + sum_term_2
            })
            // ...then compute the sum.
            .sum();

        // Already validated in sim setup that number of particles >=1.
        let arg_argument = 1.0 / num_closest as f64 * summed_terms;

        // Lastly, we compute the angle per equation 1
        arg_argument.arg()
    }

    /// Compute the new spatial coordinates
    fn compute_new_coords(&self, speed: Speed, delta_time: RelativeTime) -> (f64, f64) {
        let new_pos_x = self.pos_x + speed.0 * delta_time.0 * self.theta.cos();
        let new_pos_y = self.pos_y + speed.0 * delta_time.0 * self.theta.sin();

        (new_pos_x, new_pos_y)
    }

    /// Temporally update the particle to a new angle and position
    fn to_timestepped(
        &self,
        particles: &Particles,
        distance_threshold: ParticleDistanceThreshold,
        speed: Speed,
        noise: Noise,
        delta_time: RelativeTime,
    ) -> Self {
        let theta = self.compute_new_theta(particles, distance_threshold, speed, noise);
        let (pos_x, pos_y) = self.compute_new_coords(speed, delta_time);
        let phase = Self::sample_random_phase();

        Self {
            pos_x,
            pos_y,
            theta,
            phase,
            id: self.id,
        }
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
        distance_threshold: ParticleDistanceThreshold,
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
                .filter(|particle| self.compute_euclidean_distance(particle) < distance_threshold.0)
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
    // Note: we are technically duplicating the particle IDs because we store them on both the
    // particle struct as well as implicitly in the array itself. However, it's very readable
    // and nice to be able to grab an arbitrary particle's ID when iterating over an arbitrary
    // collection. Plus, it allows for refactoring into different kinds of collections and passing
    // around particles without also passing their indices separately.
    pub(crate) fn new(num_particles: usize, boundary_side_length: DomainBoundaryLength) -> Self {
        Self(
            // For each particle...
            (0..num_particles)
                // ...instantiate a random new one...
                .map(|id| Particle::new(id, boundary_side_length))
                // ...then collect all the particles together into this data structure.
                .collect(),
        )
    }

    /// Temporally update the particles to new angles and positions
    pub(crate) fn to_timestepped(
        &self,
        distance_threshold: ParticleDistanceThreshold,
        speed: Speed,
        noise: Noise,
        delta_time: RelativeTime,
    ) -> Self {
        Self(
            self.0
                .iter()
                .map(|particle| {
                    particle.to_timestepped(self, distance_threshold, speed, noise, delta_time)
                })
                .collect(),
        )
    }

    /// Get the number of particles
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

/// Contains the indices for the nearest particles for a given particle
struct IdxsNeighborParticles(Box<[usize]>);
