from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")


def collect_all_coordinates(font_paths: list[Path]) -> np.ndarray:
    coords = _skrifa.glyph_outline_coordinates(font_paths)
    return np.asarray(coords, dtype=np.float32)


def plot_jointplot(coords: np.ndarray, out_dir: Path, stem: str) -> None:
    ratio = 0.001

    num_coords = coords.shape[0]
    k = max(1, int(num_coords * ratio))

    rng = np.random.default_rng()
    idx = rng.choice(num_coords, size=k, replace=False)

    xs = coords[idx, 0]
    ys = coords[idx, 1]

    g = sns.jointplot(
        x=xs,
        y=ys,
        height=7,
        kind="hist",
        joint_kws={"bins": 512},
        marginal_kws={"bins": 64},
    )
    g.set_axis_labels("X coordinate", "Y coordinate")
    g.figure.suptitle("Glyph outline coordinates in Google Fonts")
    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    g.figure.savefig(png_path, dpi=350)
    g.figure.savefig(pdf_path, dpi=350)
    plt.close(g.figure)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    coords = collect_all_coordinates(font_paths)
    out_dir = Path("output")
    plot_jointplot(coords, out_dir, stem="outline_coord_jointplot")


if __name__ == "__main__":
    main()
