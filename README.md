# xprite

Pixel art editor with algorithmic tools.

To build:

```bash
git submodule update --init --recursive
```

[![Build Status](https://travis-ci.org/rickyhan/xprite-editor.svg?branch=master)](https://travis-ci.org/rickyhan/xprite-editor)

## Bugs and Improvements

* [ ] Marquee
* [ ] Fix eraser !moved
* [ ] Partial sorting curves
* [ ] Pixel perfect line(read TC Inglis thesis)
* [ ] Line modes(perfect span, predictable, continuous, bresenham)
* [ ] Curves(increment modes: fibonacci, odd, even)
* [ ] Fix ellipse( i.center stroke; ii. filled )
* [ ] Dynamic brush size
* [ ] Tabs
* [ ] Preview bounding box
* [x] Connected component algorithm
* [ ] Fix zoom
* [ ] Save/load .aseprite
* [ ] Fix web target
* [ ] Output 2x size

## Done

* [x] Stroke caching
* [ ] Output svg
* [x] ImageImguiRenderer(integrated to imgui)
* [x] Change ImDrawIdx to u32
* [x] Floodfill crashing when index is negative
* [x] Simplify drawlist
* [x] Ignore canvas OOB

## Didn't work

* [ ] rect decomp (for efficient render)

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
* [ ] Vector tools
* [ ] Select/Marquee
* [ ] Pattern Brush
* [ ] Texture Synthesis

1. Layers
* [ ] Layer grouping

1. Animation support
* [ ] Celluloid
* [ ] Preview window

1. Collaborative edit
