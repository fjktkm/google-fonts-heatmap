# 🔥 Heatmap of Google Fonts

## 📌 1. Overview

Google Fonts に含まれるすべてのフォントについて，文字の収録状況をヒートマップで可視化します．

- **縦軸**：フォント（対応文字数の多い順）
- **横軸**：コードポイント（対応フォント数の多い順）

| Plane Ceiling | Unicode Ceiling | Heatmap |
| :---: | :---: | :---: |
| General Scripts | 0x1FFF | ![0x1FFF](https://fjktkm.com/google-fonts-heatmap/google_font_heatmap_0x1FFF.png) |
| CJK Symbols and Punctuation | 0x33FF | ![0x33FF](https://fjktkm.com/google-fonts-heatmap/google_font_heatmap_0x33FF.png) |
| CJK Unified Ideographs | 0x9FFF | ![0x9FFF](https://fjktkm.com/google-fonts-heatmap/google_font_heatmap_0x9FFF.png) |
| Basic Multilingual Plane | 0xFFFF | ![0xFFFF](https://fjktkm.com/google-fonts-heatmap/google_font_heatmap_0xFFFF.png) |
| Supplementary Multilingual Plane | 0x1FFFF | ![0x1FFFF](https://fjktkm.com/google-fonts-heatmap/google_font_heatmap_0x1FFFF.png) |
| Private Use Plane | 0x10FFFF | ![0x10FFFF](https://fjktkm.com/google-fonts-heatmap/google_font_heatmap_0x10FFFF.png) |

## ✅ 2. Requirements

推奨する開発環境の構築手順に必要なものは次のとおりです：

- GitHub Desktop
- Visual Studio Code
- Remote Development Extension Pack
- Docker

## 📦 3. Installation

### 3.1. Clone the repository

GitHub Desktop でリポジトリをクローンしてください．

### 3.2 Open in Visual Studio Code

リポジトリを Visual Studio Code で開いてください．

### 3.3. Open Remote Container

Visual Studio Code でリポジトリを開いたら，右下に表示されるポップアップから「コンテナーで再度開く」というボタンを選択してください．
もしポップアップが表示されない場合は，左下の「><」アイコンをクリックして「コンテナーで再度開く」を選択してください．
これにより自動で開発環境が構築されます．

## 🚀 4. Usage

### 4.1. Download Google Fonts

Google Fonts のフォントをダウンロードします．
次のコマンドを実行してください．

```bash
sh download.sh
```

### 4.2. Generate Heatmap

ヒートマップを生成します．
次のコマンドを実行してください．

```bash
uv run main.py
```

生成したヒートマップは `output` ディレクトリに保存されます．
