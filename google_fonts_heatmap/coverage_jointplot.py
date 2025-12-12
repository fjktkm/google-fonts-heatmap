from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")
CHARACTER_SIZE = 0x10000

sns.set_theme(style="white")


def load_fonts_codepoints(font_paths: list[Path]) -> list[list[int]]:
    fonts_cps = _skrifa.coverage_bmp(font_paths, CHARACTER_SIZE)
    return sorted(fonts_cps, key=len, reverse=True)


def plot_jointplot(fonts_cps: list[list[int]], out_dir: Path, stem: str) -> None:
    x_vals = np.concatenate([np.asarray(cps, dtype=np.int32) for cps in fonts_cps])
    y_vals = np.concatenate(
        [np.full(len(cps), i, dtype=np.int32) for i, cps in enumerate(fonts_cps)],
    )

    g = sns.jointplot(
        x=x_vals,
        y=y_vals,
        height=7,
        kind="hist",
        joint_kws={"bins": 512},
        marginal_kws={"bins": 64},
    )

    g.ax_marg_x.set_yticks([])
    g.ax_marg_y.set_xticks([])
    g.ax_joint.set_xlim(-0.5, CHARACTER_SIZE - 0.5)
    g.ax_joint.set_ylim(-0.5, len(fonts_cps) - 0.5)
    g.set_axis_labels("Code point (Unicode value)", "Fonts (sorted by coverage)")
    g.figure.suptitle("Google Fonts Coverage (Basic Multilingual Plane)")
    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    g.figure.savefig(png_path, dpi=350)
    g.figure.savefig(pdf_path, dpi=350)
    plt.close(g.figure)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    fonts_cps = load_fonts_codepoints(font_paths)
    out_dir = Path("output")
    plot_jointplot(fonts_cps, out_dir, stem="coverage_jointplot")


if __name__ == "__main__":
    main()
