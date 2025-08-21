# 🔥 Glyph Coverage Heatmap for Google Fonts

## 📌 1. Overview

![](https://fjktkm.com/google-fonts-heatmap/google_fonts_heatmap.png)

[Google Fonts](https://github.com/google/fonts) に含まれるすべてのフォントについて，文字の収録状況をヒートマップで可視化します．

- **縦軸**：フォント（対応文字数の多い順）
- **横軸**：コードポイント（対応フォント数の多い順）

## 📥 2. Download

ダウンロードは [こちら](https://github.com/fjktkm/google-fonts-heatmap/releases/latest/download/google_fonts_heatmap.zip)

Please download from [here](https://github.com/fjktkm/google-fonts-heatmap/releases/latest/download/google_fonts_heatmap.zip)

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

### 5.2. Generate Heatmap

ヒートマップを生成します．
次のコマンドを実行してください．

```bash
uv run main.py
```

生成したヒートマップは `output` ディレクトリに保存されます．

## 📑 6. Citation

If you find this repository useful in your research, please cite:

```
@misc{fujioka2025googlefontsheatmap,
  author       = {{Takumu Fujioka}},
  title        = {{google-fonts-heatmap}: Glyph Coverage Heatmap for Google Fonts},
  howpublished = {GitHub repository, \url{https://github.com/fjktkm/google-fonts-heatmap}},
  year         = {2025},
}
```
