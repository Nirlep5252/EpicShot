# EpicShot

Epic screenshot tool for Linux.\
Currently tested only on `i3wm` on `Arch Linux`.

## Requirements

- Rust: `1.74.0+`
- OS Specific Requirements:
  - Arch Linux: `libxcb`, `libxrandr`, `xclip`

## Installation

### Crates.io

```bash
cargo install epicshot
```

### Manual

```bash
git clone https://github.com/Nirlep5252/EpicShot
cd EpicShot
cargo install --path .
```

### Package Manager

SOON

## Example Usage

Take screenshot of your entire screen (including all monitors) and copy to clipboard.

```bash
epicshot --x11 --all --clipboard
```

Take screenshot of a particular monitor and copy to clipboard.

```bash
epicshot --x11 --monitor 0 --clipboard
```

Save screenshot to a specific file.

```bash
epicshot --x11 --all --save screenshot.png
```
