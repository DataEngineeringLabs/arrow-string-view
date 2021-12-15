import json
import os
import matplotlib.pyplot as plt
import matplotlib.ticker as mtick


def set_size(width, fraction=1):
    """Set figure dimensions to avoid scaling in LaTeX.

    Parameters
    ----------
    width: float
            Document textwidth or columnwidth in pts
    fraction: float, optional
            Fraction of the width which you wish the figure to occupy

    Returns
    -------
    fig_dim: tuple
            Dimensions of figure in inches
    """
    # Width of figure (in pts)
    fig_width_pt = width * fraction

    # Convert from pt to inches
    inches_per_pt = 1 / 72.27

    # Golden ratio to set aesthetic figure height
    # https://disq.us/p/2940ij3
    golden_ratio = (5 ** 0.5 - 1) / 2

    # Figure width in inches
    fig_width_in = fig_width_pt * inches_per_pt
    # Figure height in inches
    fig_height_in = fig_width_in * golden_ratio

    fig_dim = (fig_width_in, fig_height_in)

    return fig_dim


def _read_reports(bench: str):
    root = f"target/criterion/{bench}/"

    result = []
    for original_path, dirs, files in os.walk(root):
        path = original_path.split(os.sep)
        if path[-1] != "new":
            continue
        path = path[-4:-1]
        task = path[0]
        type = path[1]
        size = int(path[2])

        with open(os.path.join(original_path, "estimates.json")) as f:
            data = json.load(f)

        ms = data["mean"]["point_estimate"] / 1000
        result.append(
            {
                "task": task,
                "type": type,
                "size": size,
                "time": ms,
            }
        )
    return result


def plot(result, choices, title, filename, to_stdout=False):
    x = [2 ** x["size"] for x in result if x["type"] == choices[0][0]]
    x = sorted(x)

    fig, ax = plt.subplots(1, 1, figsize=set_size(512))
    for (choice, name) in choices:
        values = [r for r in result if r["type"] == choice]
        values = sorted(values, key=lambda r: int(r["size"]))
        values = [r["time"] for r in values]
        ax.plot(x, values, "-o", label=name)

        if to_stdout:
            print(name)
            print("size, time (ms)")
            for (v1, v2) in zip(x, values):
                print(f"{v1}, {v2}")

    ax.set(xlabel="size", ylabel="time (ms)", title=title)
    ax.xaxis.set_major_formatter(mtick.ScalarFormatter(useMathText=True))
    ax.grid()
    ax.legend()

    fig.savefig(filename)

result = (
    _read_reports("take")
)

print(result)

plot(
    result,
    [
        ("array", "array"),
        ("view", "view"),
    ],
    "Take N random elements from an Arrow Utf8Array of\n"
    "N vs a \"sequence view\" of N elements, each with \"size\" elements of of 0-20 bytes",
    "array_vs_view.png",
    True,
)
