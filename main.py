import logging
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from fontTools.ttLib import TTFont
from tqdm import tqdm

logging.getLogger("fontTools").setLevel(logging.ERROR)

LICENSE_DIRS = ("apache", "ofl", "ufl")
FONT_ROOT = Path("./fonts")
EXCLUDE_KEYWORDS = ("adobeblank",)


def extract_cps(fp: Path) -> list[int]:
    font = TTFont(fp, lazy=True)
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


def crop_heatmap(hm: np.ndarray) -> np.ndarray:
    font_counts = hm.sum(axis=1)
    cp_counts = hm.sum(axis=0)
    valid_fonts = font_counts > 0
    valid_cps = cp_counts > 0
    hm = hm[valid_fonts][:, valid_cps]

    order_cps = np.argsort(hm.sum(axis=0))[::-1]
    order_fonts = np.argsort(hm.sum(axis=1))[::-1]
    hm = hm[np.ix_(order_fonts, order_cps)]

    n_rows, n_cols = hm.shape
    side = min(n_rows, n_cols)
    hm = hm[:side, :side]

    order_cps_sq = np.argsort(hm.sum(axis=0))[::-1]
    order_fonts_sq = np.argsort(hm.sum(axis=1))[::-1]
    return hm[np.ix_(order_fonts_sq, order_cps_sq)]


def plot_jointplot(hm: np.ndarray, out_dir: Path, stem: str) -> None:
    sns.set_theme(style="white")

    n_rows, n_cols = hm.shape
    r, c = np.where(hm)

    g = sns.jointplot(
        x=c,
        y=r,
        height=7,
        kind="hist",
        joint_kws={"bins": (n_cols, n_rows), "cmap": "light:b"},
    )

    for ax in (g.ax_marg_x, g.ax_marg_y):
        for p in ax.patches:
            p.set_alpha(1.0)
            p.set_linewidth(0)
            p.set_rasterized(True)

    for col in g.ax_joint.collections:
        col.set_rasterized(True)
    for im in g.ax_joint.images:
        im.set_rasterized(True)

    g.ax_marg_x.set_xlim(-0.5, n_cols - 0.5)
    g.ax_marg_y.set_ylim(-0.5, n_rows - 0.5)
    g.set_axis_labels("Code-point (font support↓)", "Font (coverage↓)")
    g.figure.suptitle("Glyph Coverage Heatmap for Google Fonts")
    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    g.figure.savefig(png_path, dpi=350)
    g.figure.savefig(pdf_path, dpi=350)
    plt.close(g.figure)


def main() -> None:
    font_paths = [
        fp
        for lic in LICENSE_DIRS
        for fp in (FONT_ROOT / lic).rglob("*.ttf")
        if not any(kw in str(fp) for kw in EXCLUDE_KEYWORDS)
    ]

    heatmap = compute_heatmap(font_paths)
    heatmap = crop_heatmap(heatmap)
    out_dir = Path("output")
    plot_jointplot(
        heatmap,
        out_dir,
        stem="google_fonts_heatmap",
    )


if __name__ == "__main__":
    main()
