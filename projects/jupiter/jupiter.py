import spiceypy as spice
from datetime import datetime
import numpy as np
from tqdm import tqdm
from scipy.optimize import minimize_scalar
from pprint import pprint
import json
import os

spice.furnsh("kernels/naif0012.tls")  # Leap seconds kernel
spice.furnsh("kernels/pck00011.tpc")  # Planetary constants kernel
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

"""
All moons with a semi-major axis < 15 million KM, discovered before 1990. This
should make a view that doesn't dwarf the closer moons, and avoid the worst of
the crowding. We also track the Sun and Earth to provide compass needles.
"""
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

START_DT, END_DT = (
    datetime.fromisoformat("2025-01-01 00:00:00"),
    datetime.fromisoformat("2026-01-01 00:00:00"),
)


def spice_et_to_datetime(et):
    return datetime.fromisoformat(spice.et2utc(et, "ISOC", 6))


def locate(body, time, observer="JUPITER BARYCENTER"):
    """
    Celestial body location oracle.
    """
    frame = "J2000"
    # If we want to show realistic views relative to earth, this should be
    # "LT+S". For now let's keep it NONE since it makes the math easier.
    abcorr = "LT+S"

    state, _ = spice.spkezr(body, time, frame, abcorr, observer)
    return state[:3]


def dist_diff_squared(a, b, p):
    """
    Squared difference between the distances from p to a, and from p to b.
    """
    return (np.linalg.norm(p - a) - np.linalg.norm(p - b)) ** 2


def F(params, t, pos):
    """
    Model a potentially-rotated ellipse coordinate
    R_x*cos(alpha)*sin(theta) + R_y*sin(alpha)*cos(theta) + C_x
        Where alpha = m*t + b
    Parameters: [R_x, R_y, m, b, theta, C_x]
    See: https://math.stackexchange.com/a/2647450/554354
    """
    (R_x, R_y, m, b, theta, C_x) = params
    return (
        R_x * np.cos(m * t + b) * np.sin(theta)
        + R_y * np.sin(m * t + b) * np.cos(theta)
        + C_x
        - pos
    )


def minimize_spice(f):
    """
    Generally optimize an arbitrary function within the standard time-range.
    Input: a function to optimize at some time. Output: a SPICE ET timestamp.
    """
    # We normalize the bounds to make the optimizer behave nicely, then
    # un-normalize the final result at the end.
    start, end = spice.str2et(START_DT.isoformat()), spice.str2et(END_DT.isoformat())
    t_range = end - start
    scale = lambda t: start + t * t_range

    result = minimize_scalar(
        lambda t: f(scale(t)),
        bounds=(0.0, 1.0),
    )
    result = minimize_scalar(
        lambda t: f(scale(t)), bounds=(result.x - 0.01, result.x + 0.01)
    )
    result.x = scale(result.x)
    return result


def define_body_point(body, et):
    pos = locate(body, et)
    return {
        "time": spice_et_to_datetime(et),
        "pos": pos,
        "norm": np.linalg.norm(pos),
    }


def tweak_solution(solution, evaluator):
    """
    Overcome some of the shortcomings of the optimizer by adjusting individual
    microseconds of the solution, verifying our local minimum is as accurate as
    possible
    """
    best_time, best_norm = solution, evaluator(solution)
    deltas = [10**epsilon for epsilon in range(3, -7, -1)]
    for delta in deltas:
        while (ev := evaluator(best_time + delta)) < best_norm:
            best_time, best_norm = best_time + delta, ev
        while (ev := evaluator(best_time - delta)) < best_norm:
            best_time, best_norm = best_time - delta, ev
    return best_time


def find_apoapse_and_periapse(body):
    apoapse = minimize_spice(lambda t: np.linalg.norm(locate(body, t)))
    ax = tweak_solution(apoapse.x, lambda t: np.linalg.norm(locate(body, t)))
    apoapse_def = define_body_point(body, ax)
    # We want to push for the periapse to oppose the apoapse, even if the
    # apoapse is slightly off. This empirically leads to more accurate results
    # once we start using the center of these values.
    periapse = minimize_spice(lambda t: np.dot(apoapse_def["pos"], locate(body, t)))
    px = tweak_solution(
        periapse.x, lambda t: np.dot(apoapse_def["pos"], locate(body, t))
    )

    return {
        "apoapse": apoapse_def,
        "periapse": define_body_point(body, px),
    }


def find_semi_minor_point(apoapse, periapse, body):
    minor = minimize_spice(
        lambda t: dist_diff_squared(apoapse["pos"], periapse["pos"], locate(body, t))
    )

    return {"minor": define_body_point(body, minor.x)}


def solve_ellipse(body):
    result = {}
    result.update(find_apoapse_and_periapse(body))
    result.update(find_semi_minor_point(result["apoapse"], result["periapse"], body))

    result["center"] = 0.5 * (result["apoapse"]["pos"] + result["periapse"]["pos"])
    result["semi_major_axis"] = np.linalg.norm(
        result["apoapse"]["pos"] - result["center"]
    )
    result["semi_minor_axis"] = np.linalg.norm(
        result["minor"]["pos"] - result["center"]
    )

    # If the orbit has extremely low eccentricity, we may get a minor axis
    # that's larger than the major axis, causing undefined eccentricity.
    result["semi_major_axis"], result["semi_minor_axis"] = (
        max(result["semi_major_axis"], result["semi_minor_axis"]),
        min(result["semi_major_axis"], result["semi_minor_axis"]),
    )

    result["eccentricity"] = np.sqrt(
        1 - result["semi_minor_axis"] ** 2 / result["semi_major_axis"] ** 2
    )

    return result


def into_json_like(solution):
    if isinstance(solution, datetime):
        return solution.timestamp()
    elif isinstance(solution, np.float64):
        return float(solution)
    elif isinstance(solution, np.ndarray):
        return solution.tolist()
    elif isinstance(solution, dict):
        return {k: into_json_like(s) for k, s in solution.items()}
    else:
        return solution


def _debug_render(body, solution):
    import matplotlib.pyplot as plt

    # Generate a numpy array of 3D points
    points = np.array(
        [
            locate(body, spice.str2et(t.isoformat()))
            for t in np.linspace(START_DT, END_DT, 1000)
        ]
    )

    # Extract X, Y, Z coordinates
    x, y, z = points[:, 0], points[:, 1], points[:, 2]

    # Create a 3D plot
    fig = plt.figure()

    ax = fig.add_subplot(111, projection="3d")
    ax.set_box_aspect((np.ptp(x), np.ptp(y), np.ptp(z)))

    ax.plot(x, y, z, linestyle="-", color="b")

    c, a, p, m = (
        solution["center"],
        solution["apoapse"]["pos"],
        solution["periapse"]["pos"],
        solution["minor"]["pos"],
    )
    ax.plot([0, a[0]], [0, a[1]], [0, a[2]], color="red")
    ax.plot([-m[0], 0, m[0]], [-m[1], 0, m[1]], [-m[2], 0, m[2]], color="orange")
    ax.plot([0, p[0]], [0, p[1]], [0, p[2]], color="yellow")
    ax.scatter([c[0]], [c[1]], [c[2]], color="blue")

    # Labels
    ax.set_xlabel("X Axis")
    ax.set_ylabel("Y Axis")
    ax.set_zlabel("Z Axis")
    ax.set_title("3D Line Plot")

    # Show plot
    plt.show()


if __name__ == "__main__":
    data = {body: solve_ellipse(body) for body in BODIES}

    os.mkdir("output")
    with open("output/output.json", "w") as out:
        json.dump(into_json_like(data), out, indent=2)

    # _debug_render(body, solution)
