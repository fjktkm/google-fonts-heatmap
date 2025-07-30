import logging
from pathlib import Path

import numpy as np
from fontTools.ttLib import TTFont
from PIL import Image
from tqdm import tqdm

logging.getLogger("fontTools").setLevel(logging.ERROR)

LICENSE_DIRS = ("apache", "ofl", "ufl")
FONT_ROOT = Path("./fonts")
EXCLUDE_KEYWORDS = ("adobeblank",)
THRESHOLDS = [
    0x1FFF,
    0x33FF,
    0x9FFF,
    0xFFFF,
    0x1FFFF,
    0x10FFFF,
]
MAX_NAME_LEN = 20


def compute_heatmap(
    font_paths: list[Path],
    threshold: int,
    max_name_len: int = 20,
) -> np.ndarray:
    n_fonts = len(font_paths)
    raw = np.zeros((n_fonts, threshold + 1), dtype=bool)
    pbar = tqdm(font_paths, unit="font")

    for i, fp in enumerate(pbar):
        name = fp.name
        name = (
            name[:max_name_len]
            if len(name) > max_name_len
            else name.ljust(max_name_len)
        )
        pbar.set_description(name)

        font = TTFont(fp, lazy=True)
        cmap = font.getBestCmap() or {}
        cps = [cp for cp in cmap if 0 <= cp <= threshold]
        raw[i, cps] = True
        font.close()

    return raw


def create_image(
    heatmap: np.ndarray,
    threshold: int,
) -> Image.Image:
    hm = heatmap[:, : threshold + 1]

    font_counts = hm.sum(axis=1)
    cp_counts = hm.sum(axis=0)

    valid_fonts = font_counts > 0
    valid_cps = cp_counts > 0
    hm = hm[valid_fonts][:, valid_cps]

    order_fonts = np.argsort(font_counts[valid_fonts])[::-1]
    order_cps = np.argsort(cp_counts[valid_cps])[::-1]
    hm_sorted = hm[np.ix_(order_fonts, order_cps)]

    img = Image.fromarray(hm_sorted.astype(np.uint8) * 255)
    return img.convert("L").convert("1")


def main() -> None:
    font_paths = [
        fp
        for lic in LICENSE_DIRS
        for fp in (FONT_ROOT / lic).rglob("*.ttf")
        if not any(kw in str(fp) for kw in EXCLUDE_KEYWORDS)
    ]

    max_thr = max(THRESHOLDS)
    heatmap = compute_heatmap(
        font_paths,
        threshold=max_thr,
        max_name_len=MAX_NAME_LEN,
    )

    for thr in THRESHOLDS:
        img = create_image(heatmap, threshold=thr)
        out_dir = Path("output")
        out_dir.mkdir(exist_ok=True)
        output_path = out_dir / f"google_fonts_heatmap_0x{thr:06X}.png"
        img.save(output_path)
        print(f"Saved: {output_path} ({img.width}x{img.height})")

        min_side = min(img.width, img.height)
        crop = img.crop((0, 0, min_side, min_side))
        crop_output_path = out_dir / f"google_fonts_heatmap_0x{thr:06X}_crop.png"
        crop.save(crop_output_path)
        print(f"Saved: {crop_output_path} ({crop.width}x{crop.height})")


if __name__ == "__main__":
    main()
