from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")


def collect_outline_stats(font_paths: list[Path]) -> np.ndarray:
    stats = _skrifa.glyph_command_and_path_counts(font_paths)
    return np.asarray(stats, dtype=np.int32)


def plot_jointplot(stats: np.ndarray, out_dir: Path, stem: str) -> None:
    shifted = stats + 1
    x_vals = shifted[:, 0]
    y_vals = shifted[:, 1]

    g = sns.jointplot(
        x=x_vals,
        y=y_vals,
        kind="hist",
        height=7,
        joint_kws={"bins": 24, "log_scale": True},
        marginal_kws={"bins": 24, "log_scale": True},
    )
    g.ax_marg_x.set_yticks([])
    g.ax_marg_y.set_xticks([])
    g.ax_joint.set_xlim(left=1)
    g.ax_joint.set_ylim(bottom=1)
    g.set_axis_labels(
        "Max path outline length per glyph (log scale)",
        "Path count (log scale)",
    )
    g.figure.suptitle("Max path outline lengths and path counts in Google Fonts")
    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    g.figure.savefig(png_path, dpi=350)
    g.figure.savefig(pdf_path, dpi=350)
    plt.close(g.figure)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    stats = collect_outline_stats(font_paths)
    out_dir = Path("output")
    plot_jointplot(stats, out_dir, stem="outline_len_path_jointplot")


if __name__ == "__main__":
    main()
