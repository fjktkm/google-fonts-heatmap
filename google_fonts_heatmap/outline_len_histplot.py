from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")


def collect_all_command_counts(font_paths: list[Path]) -> list[int]:
    return _skrifa.glyph_command_counts(font_paths)


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
    plot_histogram(counts, out_dir, stem="outline_len_histplot")


if __name__ == "__main__":
    main()
