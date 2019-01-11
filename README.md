# xprite

Pixel art editor with algorithmic tools.

[![Build Status](https://travis-ci.org/rickyhan/xprite-editor.svg?branch=master)](https://travis-ci.org/rickyhan/xprite-editor)

## Bugs and Improvements

* [ ] Output svg
* [ ] Marquee
* [ ] Stroke caching
* [ ] Filled ellipse
* [ ] Dynamic brush size
* [ ] Tabs
* [ ] Preview bounding box
* [ ] Connected component algorithm
* [ ] ImageImguiRenderer (based on cairo_imgui)
* [ ] Fix web target
* [x] Change ImDrawIdx to u32

## Done
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
