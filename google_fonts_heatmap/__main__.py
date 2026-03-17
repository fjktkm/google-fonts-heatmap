from google_fonts_heatmap import (
    coverage_jointplot,
    outline_command_barplot,
    outline_coord_jointplot,
    outline_len_histplot,
    outline_len_path_jointplot,
    upem_countplot,
    weight_countplot,
)

MODULES = [
    outline_len_histplot,
    outline_len_path_jointplot,
    outline_coord_jointplot,
    outline_command_barplot,
    coverage_jointplot,
    upem_countplot,
    weight_countplot,
]


def main() -> None:
    for module in MODULES:
        name = module.__name__.split(".")[-1]
        print(name, flush=True)
        module.main()


if __name__ == "__main__":
    main()
