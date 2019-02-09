# xprite

Pixel art editor with algorithmic tools.

To build:

```bash
git submodule update --init --recursive
```

[![Build Status](https://travis-ci.org/rickyhan/xprite-editor.svg?branch=master)](https://travis-ci.org/rickyhan/xprite-editor)

## Bugs and Improvements

* [ ] Marquee
* [ ] Partial sorting curves
* [ ] Pixel perfect line(read TC Inglis thesis)
* [ ] Line modes(perfect span, predictable, continuous, bresenham)
* [x] Set brush size
* [ ] Replace pyo3 with RustPython
* [ ] Preview bounding box
* [ ] Export with profile: (Output {fname}.1x.png, 2x, 3x, ase)
* [ ] Preview window bounding box, modes(fill, 1x, 2x)
* [ ] Curves(increment modes: fibonacci, odd, even)

## WONTFIX

* [ ] Tabs
* [ ] Output svg
* [ ] Fix web target
* [ ] rect decomp (for efficient render)

## Done

* [x] Save/load .aseprite
* [x] Fix zoom
* [x] Fix ellipse( i.center stroke; ii. filled )
* [x] Connected component algorithm
* [x] Stroke caching
* [x] Fix eraser !moved
* [x] ImageImguiRenderer(integrated to imgui)
* [x] Change ImDrawIdx to u32
* [x] Floodfill crashing when index is negative
* [x] Simplify drawlist
* [x] Ignore canvas OOB

## Milestones

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
* [ ] Symmetry
* [ ] Select/Marquee
* [ ] Copy paste
* [ ] Layer grouping

1. Others
* [ ] Pattern Brush
* [ ] Texture Synthesis

1. Animation support
* [ ] Celluloid
* [ ] Preview window

1. Collaborative edit
