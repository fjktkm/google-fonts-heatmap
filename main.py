import logging
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from fontTools.ttLib import TTFont
from tqdm import tqdm

logging.getLogger("fontTools").setLevel(logging.ERROR)

ROOT_DIR = Path("./fonts")
CHARACTER_SIZE = 0x10000

sns.set_theme(style="white")


def extract_cps(fp: Path) -> list[int]:
    font = TTFont(fp, lazy=True)
    cmap = font.getBestCmap() or {}
    cps = [cp for cp in cmap if cp < CHARACTER_SIZE]
    font.close()
    return cps


def load_fonts_codepoints(font_paths: list[Path]) -> list[list[int]]:
    with ProcessPoolExecutor() as ex:
        fonts_cps = list(
            tqdm(
                ex.map(extract_cps, font_paths),
                total=len(font_paths),
                unit="font",
            ),
        )
    return sorted(fonts_cps, key=len, reverse=True)


def plot_jointplot(fonts_cps: list[list[int]], out_dir: Path, stem: str) -> None:
    x_vals = np.concatenate(
        [np.asarray(cps, dtype=np.int32) for cps in fonts_cps],
    )
    y_vals = np.concatenate(
        [np.full(len(cps), i, dtype=np.int32) for i, cps in enumerate(fonts_cps)],
    )

    g = sns.jointplot(
        x=x_vals,
        y=y_vals,
        height=7,
        kind="scatter",
        joint_kws={"s": 0.01, "edgecolor": "none", "rasterized": True},
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
    plot_jointplot(fonts_cps, out_dir, stem="google_fonts_heatmap")


if __name__ == "__main__":
    main()
