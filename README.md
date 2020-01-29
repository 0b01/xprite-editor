# xprite

[![License: GPL](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Build Status](https://travis-ci.org/0b01/xprite-editor.svg?branch=master)](https://travis-ci.org/rickyhan/xprite-editor)
[![Lines of code](https://tokei.rs/b1/github/rickyhan/xprite-editor)](https://github.com/rickyhan/xprite-editor).


Pixel art editor with algorithmic tools.

# Features

* Support for aseprite

* Pixel art algorithms

* Customizable exporter workflow

* Wave function collapse based texture synthesis

# Build

To build:

```bash
git submodule update --init --recursive
cargo run --bin xprite-native --release
```

# Contributions Welcome

If you find xprite useful, feel free to add features you want.

Also checkout the projects tab.

# Milestones

1. Finding the right abstractions
* [x] Canvas
* [x] Renderer
* [x] Layer

1. Core functionalities
* [x] Hotkeys
* [x] Save
* [x] Load
* [x] Python Scripting
* [x] Palette

1. Basic tools (Release target)
* [x] Pencil
* [x] Line
* [x] Color Picker
* [x] Paint Bucket
* [x] Eraser
* [x] Shapes - Rect
* [x] Shapes - Circle
* [x] Vector tools
* [x] Symmetry
* [ ] Select/Marquee
* [ ] Copy paste
* [x] Layer groups

1. Animation support
* [ ] Celluloid
* [x] Preview window

1. Others
* [ ] Pattern Brush
* [x] Texture Synthesis

1. Collaborative editing
