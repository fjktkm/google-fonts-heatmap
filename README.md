# Google Fonts Analysis

## 📌 1. Overview

[![](https://torchfont.github.io/google-fonts-analysis/coverage_jointplot.png)](https://torchfont.github.io/google-fonts-analysis/coverage_jointplot.png)

[Google Fonts](https://github.com/google/fonts) に含まれるフォントを対象に，文字の収録状況，アウトライン，メタデータを集計・可視化します．

- **縦軸**：フォント（対応文字数の多い順）
- **横軸**：コードポイント（Unicode の順）

## 📥 2. Download

- All: [ZIP](https://github.com/torchfont/google-fonts-analysis/releases/latest/download/output.zip)
- Coverage Jointplot: [PNG](https://torchfont.github.io/google-fonts-analysis/coverage_jointplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/coverage_jointplot.pdf)
- Outline Length Histogram: [PNG](https://torchfont.github.io/google-fonts-analysis/outline_len_histplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/outline_len_histplot.pdf)
- Outline Length / Path Count Jointplot: [PNG](https://torchfont.github.io/google-fonts-analysis/outline_len_path_jointplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/outline_len_path_jointplot.pdf)
- UPEM Countplot: [PNG](https://torchfont.github.io/google-fonts-analysis/upem_countplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/upem_countplot.pdf)
- Weight Countplot: [PNG](https://torchfont.github.io/google-fonts-analysis/weight_countplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/weight_countplot.pdf)
- Outline Command Barplot: [PNG](https://torchfont.github.io/google-fonts-analysis/outline_command_barplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/outline_command_barplot.pdf)
- Outline Coordinate Jointplot: [PNG](https://torchfont.github.io/google-fonts-analysis/outline_coord_jointplot.png) / [PDF](https://torchfont.github.io/google-fonts-analysis/outline_coord_jointplot.pdf)

## ✅ 3. Requirements

推奨する開発環境の構築手順に必要なものは次のとおりです：

- GitHub Desktop
- Visual Studio Code
- Remote Development Extension Pack
- Docker

## 📦 4. Installation

### 4.1. Clone the repository

GitHub Desktop でリポジトリをクローンしてください．

### 4.2. Open in Visual Studio Code

リポジトリを Visual Studio Code で開いてください．

### 4.3. Open Remote Container

Visual Studio Code でリポジトリを開いたら，右下に表示されるポップアップから「コンテナーで再度開く」というボタンを選択してください．
もしポップアップが表示されない場合は，左下の「><」アイコンをクリックして「コンテナーで再度開く」を選択してください．
これにより自動で開発環境が構築されます．

## 🚀 5. Usage

### 5.1. Download Google Fonts

Google Fonts のフォントをダウンロードします．
次のコマンドを実行してください．

```bash
sh download.sh
```

### 5.2. Build the Skrifa extension (first run only)

Rust 製の Skrifa バックエンドをビルドし，Python から利用できるようにします．
初回のみ，次のコマンドを実行してください．

```bash
uv run maturin develop --release
```

### 5.3. Generate visualizations

すべての可視化を生成します．
次のコマンドを実行してください．

```bash
uv run google_fonts_analysis
```

生成したヒートマップは `output` ディレクトリに保存されます．

### 5.4. Generate individual visualizations

おまけで以下の可視化も用意しています．

**Outline Length Histogram**：アウトラインの描画コマンド長の分布を確認するには次のコマンドを実行してください．

```bash
uv run google_fonts_analysis/outline_len_histplot.py
```

**Outline Length / Path Count Jointplot**：横軸にアウトライン長，縦軸にパス数（`closePath` コマンド数）を取った joint plot を描くには次のコマンドを実行してください．

```bash
uv run google_fonts_analysis/outline_len_path_jointplot.py
```

**UPEM Count Plot**：フォントごとの unitsPerEm の頻度を調べるには次のコマンドを実行してください．

```bash
uv run google_fonts_analysis/upem_countplot.py
```

**Weight Count Plot**：フォントの `usWeightClass` の分布を見るには次のコマンドを実行してください．

```bash
uv run google_fonts_analysis/weight_countplot.py
```

**Outline Command Bar Plot**：アウトライン描画コマンドの種類ごとの総数を確認するには次のコマンドを実行してください．

```bash
uv run google_fonts_analysis/outline_command_barplot.py
```

**Outline Coordinate Joint Plot**：グリフアウトライン上の座標分布を joint plot で可視化するには次のコマンドを実行してください．

```bash
uv run google_fonts_analysis/outline_coord_jointplot.py
```
ランダムサンプルした描画コマンドの引数の X 座標・Y 座標をヒストグラムとしてプロットし，頻度の高い領域を確認できます．

## 📑 6. Citation

If you find this repository useful in your work, please consider citing the following BibTeX entry:

```
@misc{fujioka2025googlefontsanalysis,
  author       = {{Takumu Fujioka}},
  title        = {{google-fonts-analysis}: Analysis and Visualization of Google Fonts},
  howpublished = {GitHub repository, \url{https://github.com/torchfont/google-fonts-analysis}},
  year         = {2025},
}
```
