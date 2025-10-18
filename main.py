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
CHARACTER_SIZE = 0x1000


def extract_cps(fp: Path) -> list[int]:
    font = TTFont(fp, lazy=True)

    reader = getattr(font, "reader", None)
    tables = getattr(reader, "tables", {})
    if reader and tables and "post" in tables:
        del tables["post"]

    cmap = font.getBestCmap() or {}
    cps = list(cmap.keys())
    font.close()
    return cps


def compute_heatmap(font_paths: list[Path]) -> np.ndarray:
    n_fonts = len(font_paths)

    with ProcessPoolExecutor() as ex:
        fonts_cps = list(
            tqdm(ex.map(extract_cps, font_paths), total=n_fonts, unit="font"),
        )

    union_cps: set[int] = set()

    for cps in fonts_cps:
        union_cps.update(cps)

    sorted_cps = np.array(sorted(union_cps), dtype=np.int32)
    cp_to_idx = {cp: i for i, cp in enumerate(sorted_cps.tolist())}
    raw = np.zeros((n_fonts, len(sorted_cps)), dtype=bool)

    for i, cps in enumerate(fonts_cps):
        idxs = [cp_to_idx[cp] for cp in cps]
        raw[i, idxs] = True

    return raw


def crop_heatmap(hm: np.ndarray, character_size: int) -> np.ndarray:
    order_cps = np.argsort(hm.sum(axis=0))[::-1]
    hm = hm[:, order_cps]
    hm = hm[:, :character_size]
    order_fonts = np.argsort(hm.sum(axis=1))[::-1]
    return hm[order_fonts]


def plot_jointplot(hm: np.ndarray, out_dir: Path, stem: str) -> None:
    sns.set_theme(style="white")

    n_rows, n_cols = hm.shape
    r, c = np.where(hm)

    g = sns.jointplot(
        x=c,
        y=r,
        height=7,
        kind="hist",
        joint_kws={"bins": (n_cols, n_rows), "cmap": "light:b", "alpha": 0.75},
        marginal_kws={"linewidth": 0},
    )

    for ax in (g.ax_marg_x, g.ax_marg_y):
        for p in ax.patches:
            p.set_rasterized(True)

    for col in g.ax_joint.collections:
        col.set_rasterized(True)
    for im in g.ax_joint.images:
        im.set_rasterized(True)

    g.ax_marg_x.set_xlim(-0.5, n_cols - 0.5)
    g.ax_marg_y.set_ylim(-0.5, n_rows - 0.5)
    g.set_axis_labels(
        "Code point index (sorted by font support)",
        "Fonts (sorted by coverage)",
    )
    g.figure.suptitle(f"Google Fonts Coverage (Top {n_cols} code points)")
    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    g.figure.savefig(png_path, dpi=350)
    g.figure.savefig(pdf_path, dpi=350)
    plt.close(g.figure)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    heatmap = compute_heatmap(font_paths)
    heatmap = crop_heatmap(heatmap, CHARACTER_SIZE)
    out_dir = Path("output")
    plot_jointplot(heatmap, out_dir, stem="google_fonts_heatmap")


if __name__ == "__main__":
    main()
