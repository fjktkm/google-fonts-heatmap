from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")


def collect_upems(font_paths: list[Path]) -> list[int]:
    upems = _skrifa.units_per_em(font_paths)
    return [int(value) for value in upems]


def plot_upem_count(upems: list[int], out_dir: Path, stem: str) -> None:
    arr = np.asarray(upems, dtype=np.int32)
    label_order = [str(v) for v in np.unique(arr)]

    fig, ax = plt.subplots(figsize=(8, 5))
    sns.countplot(
        x=[str(v) for v in arr],
        order=label_order,
        ax=ax,
    )
    plt.setp(ax.get_xticklabels(), rotation=90)

    ax.set_yscale("log")
    ax.set_ylabel("Font count")
    ax.set_xlabel("Units per em")
    ax.set_title("UPEM distribution in Google Fonts")

    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    fig.savefig(png_path, dpi=350)
    fig.savefig(pdf_path, dpi=350)
    plt.close(fig)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    upems = collect_upems(font_paths)
    out_dir = Path("output")
    plot_upem_count(upems, out_dir, stem="upem_countplot")


if __name__ == "__main__":
    main()
