# 🔥 Glyph Coverage Heatmap for Google Fonts

## 📌 1. Overview

[![](https://fjktkm.github.io/google-fonts-heatmap/coverage_jointplot.png)](https://fjktkm.github.io/google-fonts-heatmap/coverage_jointplot.png)

[Google Fonts](https://github.com/google/fonts) に含まれるすべてのフォントについて，文字の収録状況をヒートマップで可視化します．

- **縦軸**：フォント（対応文字数の多い順）
- **横軸**：コードポイント（Unicode の順）

## 📥 2. Download

- All: [ZIP](https://github.com/fjktkm/google-fonts-heatmap/releases/latest/download/output.zip)
- Coverage Jointplot: [PNG](https://fjktkm.github.io/google-fonts-heatmap/coverage_jointplot.png) / [PDF](https://fjktkm.github.io/google-fonts-heatmap/coverage_jointplot.pdf)
- Outline Length Histogram: [PNG](https://fjktkm.github.io/google-fonts-heatmap/outline_len_histplot.png) / [PDF](https://fjktkm.github.io/google-fonts-heatmap/outline_len_histplot.pdf)
- UPEM Countplot: [PNG](https://fjktkm.github.io/google-fonts-heatmap/upem_countplot.png) / [PDF](https://fjktkm.github.io/google-fonts-heatmap/upem_countplot.pdf)
- Weight Countplot: [PNG](https://fjktkm.github.io/google-fonts-heatmap/weight_countplot.png) / [PDF](https://fjktkm.github.io/google-fonts-heatmap/weight_countplot.pdf)
- Outline Command Barplot: [PNG](https://fjktkm.github.io/google-fonts-heatmap/outline_command_barplot.png) / [PDF](https://fjktkm.github.io/google-fonts-heatmap/outline_command_barplot.pdf)

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

### 5.3. Generate Heatmap

ヒートマップを生成します．
次のコマンドを実行してください．

```bash
uv run google_fonts_heatmap/coverage_jointplot.py
```

生成したヒートマップは `output` ディレクトリに保存されます．

### 5.4. (Optional) Bonus Plots

おまけで以下の可視化も用意しています．

**Outline Length Histogram**：アウトラインの描画コマンド長の分布を確認するには次のコマンドを実行してください．

```bash
uv run google_fonts_heatmap/outline_len_histplot.py
```

**UPEM Count Plot**：フォントごとの unitsPerEm の頻度を調べるには次のコマンドを実行してください．

```bash
uv run google_fonts_heatmap/upem_countplot.py
```

**Weight Count Plot**：フォントの `usWeightClass` の分布を見るには次のコマンドを実行してください．

```bash
uv run google_fonts_heatmap/weight_countplot.py
```

**Outline Command Bar Plot**：アウトライン描画コマンドの種類ごとの総数を確認するには次のコマンドを実行してください．

```bash
uv run google_fonts_heatmap/outline_command_barplot.py
```

## 📑 6. Citation

If you find this repository useful in your work, please consider citing the following BibTeX entry:

```
@misc{fujioka2025googlefontsheatmap,
  author       = {{Takumu Fujioka}},
  title        = {{google-fonts-heatmap}: Glyph Coverage Heatmap for Google Fonts},
  howpublished = {GitHub repository, \url{https://github.com/fjktkm/google-fonts-heatmap}},
  year         = {2025},
}
```
