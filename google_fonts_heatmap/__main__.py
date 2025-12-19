from google_fonts_heatmap import (
    coverage_jointplot,
    outline_command_barplot,
    outline_coord_jointplot,
    outline_len_histplot,
    upem_countplot,
    weight_countplot,
)

MODULES = [
    outline_len_histplot,
    outline_coord_jointplot,
    outline_command_barplot,
    coverage_jointplot,
    upem_countplot,
    weight_countplot,
]


def main() -> None:
    for module in MODULES:
        module.main()


if __name__ == "__main__":
    main()
