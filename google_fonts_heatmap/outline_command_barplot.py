from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")

COMMAND_LABELS = ["moveTo", "lineTo", "quadTo", "curveTo", "closePath"]

sns.set_theme(style="white")


def collect_command_breakdown(font_paths: list[Path]) -> np.ndarray:
    totals, glyphs = _skrifa.outline_command_breakdown(font_paths)
    return np.asarray(totals, dtype=np.float64) / max(glyphs, 1)


def plot_command_barplot(
    per_glyph: np.ndarray,
    out_dir: Path,
    stem: str,
) -> None:
    fig, ax = plt.subplots(figsize=(8, 5))

    sns.barplot(x=COMMAND_LABELS, y=per_glyph, ax=ax)
    ax.set_ylabel("Commands per glyph (average)")
    ax.set_xlabel("Command")
    ax.set_title("Average outline command breakdown in Google Fonts")

    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    fig.savefig(png_path, dpi=350)
    fig.savefig(pdf_path, dpi=350)
    plt.close(fig)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    per_glyph = collect_command_breakdown(font_paths)
    plot_command_barplot(per_glyph, Path("output"), stem="outline_command_barplot")


if __name__ == "__main__":
    main()
