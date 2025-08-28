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


