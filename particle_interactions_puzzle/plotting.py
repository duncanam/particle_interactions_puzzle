import matplotlib.pyplot as plt
import numpy as np

from particle_interactions_puzzle.particle_interactions_puzzle import Simulation


def plot_simulation_timestep(sim):
    """Plot the simulation's current timestep"""
    data = sim.get_data()

    fig, ax = plt.subplots()
    ax.quiver(
        data.x,
        data.y,
        data.u,
        data.v,
        angles="xy",
        scale_units="xy",
        scale=5,
        headlength=12,
        headwidth=8,
        headaxislength=10,
        color="black",
        pivot="middle",
    )
    ax.set_aspect("equal")
    ax.set_xlim(0, sim.boundary_side_length)
    ax.set_ylim(0, sim.boundary_side_length)
    plt.title(f"Particle Simulation, t={sim.current_time:.2f}")
    plt.show()


def compute_stationary_order_parameter(
    noise, num_particles, domain_size, particle_distance_threshold, velocity, timestep
):
    """Compute stationary order parameter as a function of noise"""
    sim = Simulation(
        num_particles,
        domain_size,
        noise,
        velocity,
        timestep,
        particle_distance_threshold,
    )

    return sim.compute_stationary_order_parameter()


def plot_stationary_order_parameter(
    num_particles, domain_size, particle_distance_threshold, velocity, timestep
):
    """Plot the stationary order parameter as a function of noise"""
    noise = np.linspace(0.001, 0.999, 20)

    def stationary_order(eta):
        return compute_stationary_order_parameter(
            eta,
            num_particles,
            domain_size,
            particle_distance_threshold,
            velocity,
            timestep,
        )

    stationary_order_params = [stationary_order(eta) for eta in noise]

    plt.plot(noise, stationary_order_params)
    plt.title("Order Parameter vs. Noise")
    plt.xlabel("$\\eta$")
    plt.ylabel("$\\psi$")
    plt.show()
