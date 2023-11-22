# EpicShot

Epic screenshot tool for linux.\
Currently tested only on `i3wm` on `Arch Linux`.

## Requirements

- Rust: `1.74.0+`
- OS Specific Requirements:
  - Arch Linux: `libxcb`, `xcb-proto`, `xclip`

## Installation

### Manual

```bash
git clone https://github.com/Nirlep5252/EpicShot
cd EpicShot
cargo install --path .
```

### Crates.io

SOON

### Package Manager

SOON

## Example Usage

Take screenshot of all screens and copy to clipboard.

```bash
epicshot --x11 --all --clipboard
```

Save screenshot to a specific file.

```bash
epicshot --x11 --all --save screenshot.png
```
