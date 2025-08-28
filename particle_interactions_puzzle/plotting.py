import matplotlib.pyplot as plt


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
