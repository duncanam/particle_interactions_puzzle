# Particle Interactions Puzzle Simulator

## Prerequisites
Several tools are needed to run this project:

* Rust (as of time of authorship, 1.89)
  * Rust can be installed however desired, then: `rustup default stable`.
* Python 3.11
  * Pick your favorite package manager
* uv, manages Python projects
  * See Astral's documention on using/installing `uv`
* maturin, builds Python-Rust bindings
  * `uv tool install "maturin[patchelf]"`

## Install
You can compile the Rust project, Python bindings, and install into the ephemeral
virtual environment using:

```bash
maturin develop --uv --release
```

Install using `uv`, a Python project and build manager.
Sync deps for the project into the ephemeral virtual environment:

```bash
uv sync
```

## Usage
See `examples/project.ipynb` / `examples/project.pdf`. This Jupyter notebook executes
the Python bindings for the Rust tools, and also plots some results. This notebook
directly follows `problem/Particle_Interactions.pdf`.

## Discussion
This discussion directly follows `problem/Particle_Interactions.pdf`.

### Preface
To complete this assignment, a Rust-Python project architecture was chosen.
Rust powers mostly everything in the analysis, except when user-facing and
plotting scenarios are performed. This was selected for the following reasons.

Rust has excellent developer experience with strong typing and great
compiler feedback. It's easy to spool up projects quickly (this was completed
in <24 hours). When using type-driven-development, it allows one to iterate
more quickly and accurately compared to needing to write lots of test cases
to verify coverage. That said, if this were production software, test coverage
should probably still be used to validate error handling and physics.

Python, on the other hand, is very ergonomic, prevalent, and easy to quickly
toy with. This makes it a great user-facing interface. When combined with
Jupyter notebooks, this helps display your analysis efficiently and iterate
on a problem effectively.

Rust and Python have a great relationship, with the glue being Maturin. This
allows for trivial cross-language communication.

### Architecture
Each portion of the architecture will be briefly discussed. Before doing so, a few
points will be mentioned:

* Immutability was prioritized. This helps reduce unexpected statefulness mutation,
  and allows the updates to be more predictable and readable. If performance or
  further readability is desired, it's easy to swap a section to mutate. It should
  be noted that Rust is immutable by default, and otherwise requires the use of
  the keyword `mut` to denote mutation.
* Iterators were prioritized over loops. This allows for lazy-evaluation and is
  also arguably more readable in some scenarios. It was not used where it did not
  feel ergonomic.
* Errors are handled with the `anyhow` crate. Exceptions don't exist in Rust, so
  idiomatically potential errors are handled with the `Result` enum, with `Ok` and
  `Err` variants. `anyhow` provides some convenience wrapping on top of this.

#### Simulation
Covered in `simulation.rs`, this is the main entrypoint to the simulation. Here we
have `Simulator`, a struct that contains simulation state and parameters. Particles
are stored here, as well as domain information and other setup.

The `to_timestepped()` method updates the simulation state immutably. This has
the benefit of trivially allowing to store multiple temporal versions of the state
for comparison, if desired. Also while out of scope of this project, it also allows
for very natural checkpointing, as serialization methods are more likely to succeed.
This method calls the forward integration methods on each particle and also computes
the instantaneous order (polarization) of the collection of particles.

The `compute_stationary_order_parameter()` method is an example where using a loop
plus some mutation was cleaner. This method computes the steady-state value of the
order parameter, by iterating over the simulation, maintaining a rolling average
(using a window) of the current instantaneous order. A ring buffer was used for
the window, progressively adding/removing new values from the ends and recomputing
the average of the window. A smarter averaging technique could be used with pure
math (i.e. a mutating float representing the sum), but this was simple for now and
a benchmark later if required can determine if that update is required. This loop
is exited once the window is checked to actually be filled and the current
instantaneous order is within a tolerance of the sliding window value. This is
how I've often set up convergence criteria for certain CFD simulations, for the
target values (assuming numerics are also converged).

A convenience struct holds the simulation parameters for easy iteration copy,
and there are some display convenience traits implemented for printout in Python.
This file also contains a `SimulationData` struct that acts as an API layer
for pulling out particle state for easy plotting.

#### Particles
`particle.rs` is the bread and butter of this package. This sets up a particle collection
`Particles` which hold each individual `Particle` instances. `Particle`s are instantiated
with a random position and orientation, through the constructor on `Particles`. The phase
$\xi$ is also computed for each particle at this point (but is also updated for each
timestep).

On `Particles`, `to_timestepped()` iterates over each particle and integrates forward
their state in time. It then collects back a new collection of newly-integrated `Particle`s.
It also has `compute_instantaneous_order()`, which using the provided function from the
assignment, computes the instantaneous order of the collection of particles leveraging
a complex number crate.

`Particle` has a lot going on. Outside of the randomized initialization (which noted,
could be more efficient by pre-initializing the thread-local random number generator),
it has a Euclidean distance calculator that also takes into account the periodic
boundary conditions of the simulation domain. In general, periodic BCs are enforced
using a Euclidean remainder, which in Rust allows us to use `%` but with floats.
You may notice that many of the helper functions are annotated with `inline`, and
this is just to encourage compiler optimization by reducing function calls.
Both positional and orientation update helper functions exist, and this is where
the math is located for the forward integration. These are `compute_new_theta()` and
`compute_new_coords()`. It should be noted that we leverage Euler's formula to
decompose the real and imaginary parts. The timestep function, `to_timestepped()`,
calls each update method for the fields and samples a new random phase $\xi$. It
also enforces periodic boundary conditions using the remainder logic. Another
utility on this struct is `compute_idxs_closest()`. When called on a particle,
it inefficiently iterates over all particles besides itself and computes the
distances to each particle, then only grabbing the close particles given the
target radius. In practice this is `O(n^2)`. This can probably be reduced to
`O(n(log(n)))` if using a technique like k-d trees to decompose the domain,
but for simplicity and time the existing technique was used. However, `kiddo`
would be a good crate candidate to do this calc if we wanted to expand it in
the future.

#### Optimization
For the optimization problem, `optimize.rs` was leveraged. Inside, we use `argmin`,
a numerical optimization and root solving crate. From this, we use the Nelder-Mead
method to actually perform the optimization of the noise parameter. I like Nelder-Mead
because it's extremely robust, as it marches a simplex around the domain and doesn't
fall off numerical cliffs easily. It's also a multidimentional optimizer, which is
required for this problem. The gotcha of this optimizer is that the initial simplex
selection is important, and makes it sensitive to local minima. The residual function
fed to Nelder-Mead to optimize is simply taking a simulation slightly on each side
of the target noise, and minimizing when the delta of both sides is fairly large.
Decreasing the target noise offsets will incur greater accuracy at the cost of
requiring more iterations. A reasonable default of 5% on each side was selected.

#### Types
`types.rs` introduces zero-cost (removed by LLVM at compile-time) type wrappers that
give us type stability so we don't mix up floats when passing them around the codebase.
It also introduces a type alias for floats, such that the codebase can be swapped
between 64-bit and 32-bit if desired. I've found it's much easier to do this upfront
rather than go back and add it later. A convenience macro exists to create the
types, which also implements `+` between themselves and `*` with a float.

#### Math
Inside `math.rs` lives a convenience trait for math helpers was created to optimize
`x^2`. However, this is likely overkill and probably can be removed. Truthfully,
I thought I'd add more into this module.

#### Library and Python Bindings
Inside `lib.rs` lives the top-level library and PyO3's Python bindings.
