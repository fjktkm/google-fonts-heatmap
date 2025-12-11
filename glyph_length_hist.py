import itertools
import logging
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from fontTools.pens.basePen import BasePen
from fontTools.ttLib import TTFont
from fontTools.ttLib.ttGlyphSet import _TTGlyph, _TTGlyphSet
from tqdm import tqdm

logging.getLogger("fontTools").setLevel(logging.ERROR)

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")

Point = tuple[float, float]


class GlyphCommandCountPen(BasePen):
    """Pen that counts draw commands invoked for a glyph."""

    def __init__(self, glyph_set: _TTGlyphSet) -> None:
        super().__init__(glyph_set)
        self.count = 0

    def _moveTo(self, pt: Point) -> None:
        self.count += 1

    def _lineTo(self, pt: Point) -> None:
        self.count += 1

    def _curveToOne(self, pt1: Point, pt2: Point, pt3: Point) -> None:
        self.count += 1

    def _qCurveToOne(self, pt1: Point, pt2: Point | None) -> None:
        self.count += 1

    def _closePath(self) -> None:
        self.count += 1

    def _endPath(self) -> None:
        self.count += 1


def glyph_command_counts_from_font(fp: Path) -> list[int]:
    font = TTFont(fp, lazy=True)
    glyph_set: _TTGlyphSet = font.getGlyphSet()
    counts: list[int] = []
    for glyph_name in glyph_set:
        glyph: _TTGlyph = glyph_set[glyph_name]
        pen = GlyphCommandCountPen(glyph_set)
        glyph.draw(pen)
        counts.append(pen.count)
    font.close()
    return counts


def collect_all_command_counts(font_paths: list[Path]) -> list[int]:
    with ProcessPoolExecutor() as ex:
        per_font = list(
            tqdm(
                ex.map(glyph_command_counts_from_font, font_paths),
                total=len(font_paths),
                unit="font",
            ),
        )
    return list(itertools.chain.from_iterable(per_font))


def plot_histogram(counts: list[int], out_dir: Path, stem: str) -> None:
    arr = np.asarray(counts, dtype=np.int32)
    shifted = arr + 1

    fig, ax = plt.subplots(figsize=(8, 5))
    sns.histplot(shifted, bins=64, log_scale=True, ax=ax)

    ax.set_ylabel("Glyph count")
    ax.set_xlabel("Glyph outline length (log scale)")
    ax.set_title("Glyph outline lengths in Google Fonts")

    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    fig.savefig(png_path, dpi=350)
    fig.savefig(pdf_path, dpi=350)
    plt.close(fig)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    counts = collect_all_command_counts(font_paths)
    out_dir = Path("output")
    plot_histogram(counts, out_dir, stem="outline_len_hist")


if __name__ == "__main__":
    main()
