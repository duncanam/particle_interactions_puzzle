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
