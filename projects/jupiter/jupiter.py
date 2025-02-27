import spiceypy as spice
import numpy as np
import json
import os

spice.furnsh("kernels/naif0012.tls")  # Leap seconds kernel
spice.furnsh("kernels/pck00011.tpc")  # Planetary constants kernel
spice.furnsh("kernels/gm_de440.tpc")  # Gravitational constants
# Jupiter and moons SPK kernels
spice.furnsh("kernels/jup344.bsp")
spice.furnsh("kernels/jup346.bsp")
spice.furnsh("kernels/jup365.bsp")

"""
Ultimately, we want to get accurate parametric equations for the relative
positions of all the moons (from above) so we can render them on the map. We
could try and find the functions and constants within SPICE to get the functions
exactly, but instead we use SPICE as an oracle to figure out what the ellipses
are supposed to look like.
"""

# All moons with a semi-major axis < 15 million KM, discovered before 1990. This
# should make a view that doesn't dwarf the closer moons, and avoid the worst of
# the crowding. We also track the Sun and Earth to provide compass needles.
BODIES = [
    "ADRASTEA",
    "AMALTHEA",
    "CALLISTO",
    "EUROPA",
    "GANYMEDE",
    "IO",
    "METIS",
    # "SUN", # that's no moon...
    "THEBE",
]

EPOCH = spice.str2et("2025-01-01 00:00:00")


def compute_data_for(moon):
    # Find where the moon is at the epoch time relative to Jupiter
    state, _ = spice.spkezr(moon, EPOCH, "J2000", "NONE", "JUPITER")
    # Compute the orbital characteristics based on that moment
    mu = spice.bodvrd("JUPITER", "GM", 1)
    params = spice.oscltx(state, EPOCH, mu[1][0])

    return {
        "perifocal_distance": params[0],
        "eccentricity": params[1],
        "inclination": params[2],
        "long_asc_node": params[3],
        "arg_periapsis": params[4],
        "mean_anomaly_at_epoch": params[5],
        "epoch": params[6],
        "gravitational_parameter": params[7],
        "true_anomaly_at_epoch": params[8],
        "semi_major_axis": params[9],
        "semi_minor_axis": params[9] * np.sqrt(1 - params[1]),
        "period": params[10],
        "time": spice.et2utc(EPOCH, 'ISOC', 0),
    }


def save(data):
    os.makedirs("output", exist_ok=True)
    with open("output/output.json", "w") as out:
        json.dump(data, out, indent=2)


def debug_validate(data):
    """
    check against (major, eccentricity) pairs from Wikipedia -- we consistently
    land within 900 km, which is good enough since our display will have a
    resolution a little over 1000 km, and the numbers are always changing
    slightly.
    """
    expected_values = {
        "EUROPA": (671100, 0.0090),
        "CALLISTO": (1882700, 0.0074),
        "IO": (421800, 0.0041),
        "GANYMEDE": (1070400, 0.0013),
        "AMALTHEA": (181400, 0.0032),
        "THEBE": (221900, 0.0175),
        "METIS": (128000, 0.0002),
        "ADRASTEA": (129000, 0.0015),
    }

    for k, v in expected_values.items():
        print(
            k + ":" + " " * (8 - len(k)),
            f"Δa = {round(data[k]['semi_major_axis'] - v[0], 1):>6}" + "km",
        )
        min_ax = v[0] * np.sqrt(1 - v[1])
        print(
            " " * 9, f"Δb = {round(data[k]['semi_minor_axis'] - min_ax, 1):>6}" + "km"
        )


if __name__ == "__main__":
    data = {body: compute_data_for(body) for body in BODIES}
    save(data)
    debug_validate(data)
