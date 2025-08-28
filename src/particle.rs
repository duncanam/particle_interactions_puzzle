use std::f64::consts::PI;

use num::Complex;

use crate::{
    math::Math,
    types::{DomainBoundaryLength, Float, Noise, ParticleDistanceThreshold, RelativeTime, Speed},
};

/// Represents 360 degrees of spatial rotation available
const MAX_PARTICLE_ANGLE: Float = 2.0 * PI;

/// An individual particle with spatial and rotational state
pub(crate) struct Particle {
    pub(crate) id: usize,
    pub(crate) pos_x: Float,
    pub(crate) pos_y: Float,
    pub(crate) theta: Float,
    pub(crate) phase: Float,
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

    /// Compute the Euclidean distance delta for a periodic BC
    // Snagged formula from online
    #[inline]
    fn compute_euclidean_coord_delta_w_periodic(
        x1: Float,
        x2: Float,
        boundary_side_length: DomainBoundaryLength,
    ) -> Float {
        let dx = (x1 - x2).rem_euclid(boundary_side_length.0);
        dx.min(boundary_side_length.0 - dx)
    }

    /// Compute the shortest distance between this particle and another particle
    ///
    /// sqrt((x2-x1)^2 - (y2-y1)^2)
    #[inline]
    fn compute_euclidean_distance(
        &self,
        other: &Self,
        boundary_side_length: DomainBoundaryLength,
    ) -> Float {
        let dx = Self::compute_euclidean_coord_delta_w_periodic(
            self.pos_x,
            other.pos_x,
            boundary_side_length,
        );
        let dy = Self::compute_euclidean_coord_delta_w_periodic(
            self.pos_y,
            other.pos_y,
            boundary_side_length,
        );

        (dx.square() + dy.square()).sqrt()
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
        boundary_side_length: DomainBoundaryLength,
    ) -> Float {
        let idxs_closest =
            self.compute_idxs_closest(particles, distance_threshold, boundary_side_length);

        // This is "|s_i(t)|"
        let num_closest = idxs_closest.0.len();

        // If there are no close particles, stay on current heading
        if num_closest == 0 {
            return self.theta;
        }

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
                // Decompose these with Euler's formula
                // v * e^{i \theta_j(t)} = v * (\cos(\theta_j) + i*\sin(\theta_j))
                speed.0 * Complex::new(particle.theta.cos(), particle.theta.sin())
            })
            // ...then compute the sum.
            .sum();

        // \eta * e^{i \xi_n(t)} = \eta * (\cos(\xi_n) + i*\sin(\xi_n))
        let noise_term = noise.0 * Complex::new(self.phase.cos(), self.phase.sin());

        let arg_argument = 1.0 / num_closest as f64 * summed_terms + noise_term;

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
        boundary_side_length: DomainBoundaryLength,
    ) -> Self {
        let theta = self.compute_new_theta(
            particles,
            distance_threshold,
            speed,
            noise,
            boundary_side_length,
        );
        let (pos_x, pos_y) = self.compute_new_coords(speed, delta_time);
        let phase = Self::sample_random_phase();

        // Enforce periodic boundary condition using modulus. Would normally use `%` operator but
        // for floats we need to use something a bit more special.
        let pos_x = pos_x.rem_euclid(boundary_side_length.0);
        let pos_y = pos_y.rem_euclid(boundary_side_length.0);

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
        boundary_side_length: DomainBoundaryLength,
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
                .filter(|particle| {
                    self.compute_euclidean_distance(particle, boundary_side_length)
                        < distance_threshold.0
                })
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
        boundary_side_length: DomainBoundaryLength,
    ) -> Self {
        Self(
            self.0
                .iter()
                .map(|particle| {
                    particle.to_timestepped(
                        self,
                        distance_threshold,
                        speed,
                        noise,
                        delta_time,
                        boundary_side_length,
                    )
                })
                .collect(),
        )
    }

    /// Get the number of particles
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, Particle> {
        self.0.iter()
    }
}

/// Contains the indices for the nearest particles for a given particle
struct IdxsNeighborParticles(Box<[usize]>);
