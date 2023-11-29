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
epicshot --x11 --clipboard --all
```

Take screenshot of a particular monitor and copy to clipboard.

```bash
epicshot --x11 --clipboard --monitor 0
```

Take screenshot of a particular window and copy to clipboard.

```bash
epicshot --x11 --clipboard --window <WINDOW ID>
# You can get the window ID using `xwininfo`
```

Take screenshot of a selection and copy to clipboard.

```bash
epicshot --x11 --clipboard --selection "0 0 500 700"
# The selection input is "x y width height"
```

Save screenshot to a specific file.

```bash
epicshot --x11 --all --save screenshot.png
```
