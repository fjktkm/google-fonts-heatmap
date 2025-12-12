from __future__ import annotations

import logging
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path
from typing import TYPE_CHECKING

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from fontTools.pens.basePen import BasePen
from fontTools.ttLib import TTFont
from tqdm import tqdm

if TYPE_CHECKING:
    from fontTools.ttLib.ttGlyphSet import _TTGlyph, _TTGlyphSet

logging.getLogger("fontTools").setLevel(logging.ERROR)

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")

Point = tuple[float, float]


class PointCollectPen(BasePen):
    def __init__(self, glyph_set: _TTGlyphSet) -> None:
        super().__init__(glyph_set)
        self.points: list[Point] = []

    def _moveTo(self, pt: Point) -> None:
        self.points.append(pt)

    def _lineTo(self, pt: Point) -> None:
        self.points.append(pt)

    def _curveToOne(
        self,
        pt1: Point,
        pt2: Point,
        pt3: Point,
    ) -> None:
        self.points.append(pt1)
        self.points.append(pt2)
        self.points.append(pt3)

    def _qCurveToOne(
        self,
        pt1: Point,
        pt2: Point,
    ) -> None:
        self.points.append(pt1)
        self.points.append(pt2)

    def _closePath(self) -> None:
        return

    def _endPath(self) -> None:
        return


def glyph_coordinates_from_font(fp: Path) -> np.ndarray:
    font = TTFont(fp, lazy=True)
    glyph_set: _TTGlyphSet = font.getGlyphSet()
    upem = getattr(font["head"], "unitsPerEm", 1)
    coords: list[Point] = []
    for glyph_name in glyph_set:
        glyph: _TTGlyph = glyph_set[glyph_name]
        pen = PointCollectPen(glyph_set)
        glyph.draw(pen)
        coords.extend(pen.points)
    font.close()
    if not coords:
        return np.empty((0, 2), dtype=np.float32)
    arr = np.asarray(coords, dtype=np.float32)
    arr /= np.float32(upem)
    return arr


def collect_all_coordinates(font_paths: list[Path]) -> np.ndarray:
    with ProcessPoolExecutor() as ex:
        per_font_arrays = list(
            tqdm(
                ex.map(glyph_coordinates_from_font, font_paths),
                total=len(font_paths),
                unit="font",
            ),
        )
    per_font_arrays = [arr for arr in per_font_arrays if arr.size > 0]
    if not per_font_arrays:
        return np.empty((0, 2), dtype=np.float32)
    return np.concatenate(per_font_arrays, axis=0)


def plot_jointplot(coords: np.ndarray, out_dir: Path, stem: str) -> None:
    if coords.size == 0:
        return
    ratio = 0.0001

    num_coords = coords.shape[0]
    k = int(num_coords * ratio)

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
