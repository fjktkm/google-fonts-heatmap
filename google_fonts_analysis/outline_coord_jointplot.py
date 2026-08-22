from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_analysis import _skrifa

ROOT_DIR = Path("data/google/fonts")

sns.set_theme(style="white")


SAMPLE_RATE = 0.001


def collect_all_coordinates(font_paths: list[Path]) -> np.ndarray:
    coords = _skrifa.glyph_outline_coordinates(font_paths, SAMPLE_RATE)
    return np.asarray(coords, dtype=np.float32)


def plot_jointplot(coords: np.ndarray, out_dir: Path, stem: str) -> None:
    xs = coords[:, 0]
    ys = coords[:, 1]

    g = sns.jointplot(
        x=xs,
        y=ys,
        height=7,
        kind="hist",
        xlim=(-1, 2),
        ylim=(-1, 2),
        joint_kws={"binwidth": 3 / 128},
        marginal_kws={"binwidth": 3 / 64},
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
    font_paths = [
        path
        for directory in ("apache", "ofl", "ufl")
        for path in ROOT_DIR.glob(f"{directory}/*/*.[tToO][tT][fF]")
        if path != ROOT_DIR / "ofl/adobeblank/AdobeBlank-Regular.ttf"
    ]
    coords = collect_all_coordinates(font_paths)
    out_dir = Path("output")
    plot_jointplot(coords, out_dir, stem="outline_coord_jointplot")


if __name__ == "__main__":
    main()
