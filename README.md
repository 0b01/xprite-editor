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
* [ ] Curves(increment modes: fibonacci, odd, even)
* [ ] Set brush size
* [ ] Preview bounding box
* [ ] Fix zoom
* [ ] Save/load .aseprite
* [ ] Output 2x size
* [ ] Preview window bounding box, modes(fill, 1x, 2x)

## Done

* [ ] Tabs
* [ ] Output svg
* [ ] Fix web target
* [ ] rect decomp (for efficient render)
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
