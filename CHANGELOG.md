# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Documentation reorganization (FEATURES.md, TODO.md)

## [0.1.0] - 2025-12-19

### Added

#### Core Graphics

- Geometry primitives: `Point`, `Rect`, `Size`, `Matrix3x3`
- `Color` type with RGBA support and premultiplied alpha
- Path building with Bezier curves (quadratic, cubic)
- `Paint` system with fill/stroke modes and blend modes
- `Surface` and `Canvas` trait abstractions for backend-agnostic rendering

#### CPU Backend

- Software rasterizer using `raqote`
- Shape rendering: rectangles, circles, lines
- Path rendering with anti-aliasing
- PNG export functionality
- Image compositing

#### Text Rendering

- TrueType (.ttf) font loading with `ttf-parser`
- OpenType (.otf) font support
- Glyph rasterization using `fontdue`
- LRU glyph cache (default 1000 glyphs)
- Text shaping with `rustybuzz` for complex scripts
- Support for Arabic, Hebrew, Devanagari, CJK scripts
- Text layout with alignment options (Left, Center, Right)
- `draw_text()` method in Canvas API

#### Examples

- `basic_shapes` - Demonstrates shape rendering
- `text_rendering` - Demonstrates font loading and text drawing

#### Project Structure

- Consolidated single-crate architecture (`sina`)
- Modular organization with submodules:
  - `cpu` - CPU rasterization backend
  - `text` - Text rendering and typography
  - `gpu` - GPU backend (placeholder)
  - `effects` - Effects and filters (placeholder)

#### Dependencies (Pure Rust)

- `lyon_path` 1.0 - Vector path representation
- `lyon_tessellation` 1.0 - Path tessellation
- `raqote` 0.8 - 2D rasterization
- `ttf-parser` 0.20 - Font metadata parsing
- `fontdue` 0.8 - Glyph rasterization
- `rustybuzz` 0.12 - Text shaping
- `unicode-bidi` 0.3 - BiDirectional text support
- `glam` 0.25 - Vector/matrix mathematics
- `image` 0.24 - Image encoding
- `rayon` 1.8 - Parallelism
- `wgpu` 22 - GPU backend (placeholder)

#### Documentation

- Comprehensive README with quick start examples
- FEATURES.md with detailed feature list
- TODO.md with development roadmap
- API documentation with doc comments
- MIT/Apache-2.0 dual licensing

#### Testing

- 11 unit tests covering core functionality
- Test coverage for color, geometry, paint, path, and text modules

### Changed

- N/A (initial release)

### Deprecated

- N/A (initial release)

### Removed

- N/A (initial release)

### Fixed

- N/A (initial release)

### Security

- N/A (initial release)

## [0.0.0] - Project Initialization

### Added

- Initial project setup
- Workspace structure
- Basic Cargo configuration

---

## Version History

- **[0.1.0]** - First public release with core graphics and text rendering
- **[0.0.0]** - Project initialization

[Released]:
[0.1.0]: https://github.com/GrandpaEJ/sina/releases/tag/v0.1.0
