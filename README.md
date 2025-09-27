# SWIRL

## Synopsis
The _Simple WebAssembly Interaction and Rendering Library_ is a 2D game framework
that builds on the media capabilities of modern browsers.

## Brief History
_Swirl_ was created for (and is copyright of)
[MultiSLOT LLC](https://multislot.com/index.html). Notably, it was written in the
space of over a year; similar developments often require small teams a year 
of two to reach equivalent functionality.

## Highlights
_Swirl_ is written mostly in **Rust**, along with some necessary polyglot glue. It
compiles to _WebAssembly_ as one of its main objectives was superior performance.
In most demos, it did use from **10** to **25 times** less CPU than competing
_JavaScript_ libraries.

For rendering, the library taps into the capabilities of the `HTMLCanvasElement`.

Aside from [Scene Graph](https://en.wikipedia.org/wiki/Scene_graph) capabilities,
_Swirl_ provides media loaders, sound playback, drawing primitives such as sprites,
text, buttons (all with corresponding
[Affine Transformations](https://mathworld.wolfram.com/AffineTransformation.html)
over homogeneous coordinates), multi-image movies, RGBA/HSLA/named color models and
automatic constrained layouts.

## Status
Development has now stopped without foreseeable continuation.

Basic functionality in _Swirl_ is fully implemented and extensively tested,
though it was also meant to incorporate [Spine](https://esotericsoftware.com/)
animations. Initial efforts for an asset compiler (to be written in **Haskell**)
can be found in the codebase.

## Purpose of this repository
It exists solely as a code showcase. This is a proprietary product.

