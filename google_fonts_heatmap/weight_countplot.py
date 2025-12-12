from pathlib import Path

import matplotlib.pyplot as plt
import seaborn as sns

from google_fonts_heatmap import _skrifa

ROOT_DIR = Path("./fonts")

sns.set_theme(style="white")


def collect_weights(font_paths: list[Path]) -> list[int]:
    weights = _skrifa.weight_classes(font_paths)
    return [int(value) for value in weights]


def plot_weight_countplot(weights: list[int], out_dir: Path, stem: str) -> None:
    labels = [str(value) for value in weights]

    fig, ax = plt.subplots(figsize=(8, 5))
    sns.countplot(x=labels, ax=ax, order=sorted(set(labels), key=int))
    ax.set_xlabel("OS/2 usWeightClass")
    ax.set_ylabel("Font count")
    ax.set_title("Font weight distribution in Google Fonts")
    plt.tight_layout()

    out_dir.mkdir(exist_ok=True)
    png_path = out_dir / f"{stem}.png"
    pdf_path = out_dir / f"{stem}.pdf"
    fig.savefig(png_path, dpi=350)
    fig.savefig(pdf_path, dpi=350)
    plt.close(fig)


def main() -> None:
    font_paths = list(ROOT_DIR.rglob("*.[tToO][tT][fF]"))
    weights = collect_weights(font_paths)
    plot_weight_countplot(weights, Path("output"), stem="weight_countplot")


if __name__ == "__main__":
    main()
