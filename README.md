# xprite

Pixel art editor with algorithmic tools.

To build:

```bash
cargo run --bin xprite-native
```

[![Build Status](https://travis-ci.org/rickyhan/xprite-editor.svg?branch=master)](https://travis-ci.org/rickyhan/xprite-editor)

## Bugs and Improvements

* [ ] investigate patterned dithering
* [ ] investigate other anti-aliasing algos
* [ ] Checkbox to select whether to use indexed color
* [ ] frame rate drops when marquee selection is active
* [ ] Pass in mutable buffer and minimize copying
* [ ] Layer merge down/up
* [ ] Foreground/background color
* [ ] Copy paste
* [ ] rotsprite
* [ ] Marquee tool
* [ ] Outline in selected area
* [ ] snapping with one button(shift)

## TODO

* [ ] refactor color tool(using palette crate)
* [ ] huge refactor to eliminate copying
* [ ] Performance is bad when drawing with 150x150 brush
* [ ] Deep learning based rotator
* [ ] Partial sorting curves(read TC Inglis thesis)
* [ ] Line modes(perfect span, predictable, continuous, bresenham)
* [ ] Replace pyo3 with RustPython
* [ ] Curves(increment modes: fibonacci, odd, even)

## WONTFIX

* [ ] Tabs
* [ ] Output svg
* [ ] Fix web target
* [ ] rect decomp (for efficient render)
* [ ] Bounding box in preview window

## Done

* [x] AA: finish integrating anti-aliasing
* [x] Autoshade: erode and then color with gradient
* [x] Autoshade: erosion modes: cumulative(step by step), vs erode original
* [x] selective outlining, anti-aliasing
* [x] Indexed Color mode
* [x] wave function collapse using wfc
* [x] Exporter with profile: (Output {fname}.1x.png, 2x, 3x, ase)
* [x] multi document
* [x] Diagonal(\) symmetry is broken
* [x] Symmetry tool
* [x] rotational symmetry
* [x] Add a mode in paintbucket for 8-way connected floodfill
* [x] Icon for color picker
* [x] hover show color in color picker
* [x] show selected color in palette
* [x] put info in panel (radius, aspect ratio, etc.)
* [x] Preview window preserve aspect ratio, modes(fill, 1x, 2x)
* [x] Set brush size
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
* [x] Symmetry
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
