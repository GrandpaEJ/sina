# Sina - سینا

[![Crates.io](https://img.shields.io/crates/v/sina.svg)](https://crates.io/crates/sina)
[![Documentation](https://docs.rs/sina/badge.svg)](https://docs.rs/sina)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A modern 2D graphics rendering engine written in **pure Rust**, inspired by Skia.

## ✨ Features

- ✅ **Pure Rust**: No C/C++ dependencies, fully memory-safe
- ✅ **Vector Graphics**: Path rendering with Bezier curves
- ✅ **Text Rendering**: TrueType (.ttf) and OpenType (.otf) font support
- ✅ **Complex Scripts**: Arabic, Hebrew, CJK, and more via rustybuzz
- ✅ **Multiple Backends**: CPU (software) and GPU (WebGPU - planned)
- ✅ **High Performance**: SIMD optimizations and glyph caching
- ✅ **Clean API**: Ergonomic, Rust-idiomatic interface

## 🚀 Quick Start

Add Sina to your `Cargo.toml`:

```toml
[dependencies]
sina = "0.1.0"
```

### Drawing Shapes

```rust
use sina::{Color, Paint, Point, Rect, Surface, CpuSurface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a surface
    let mut surface = CpuSurface::new(800, 600);

    // Clear background
    surface.canvas().clear(Color::WHITE);

    // Draw a red rectangle
    let paint = Paint::with_color(Color::RED);
    surface.canvas().draw_rect(
        Rect::new(100.0, 100.0, 200.0, 150.0),
        &paint,
    );

    // Draw a blue circle
    let paint = Paint::with_color(Color::BLUE);
    surface.canvas().draw_circle(
        Point::new(450.0, 200.0),
        75.0,
        &paint,
    );

    // Save to PNG
    surface.save_png("output.png")?;
    Ok(())
}
```

### Drawing Text

```rust
use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut surface = CpuSurface::new(800, 400);
    surface.canvas().clear(Color::WHITE);

    // Load a font
    let font = Font::from_file("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")?;

    // Draw text
    let paint = Paint::with_color(Color::rgb(40, 40, 40));
    surface.canvas().draw_text(
        "Hello, Sina! مرحبا שלום",
        Point::new(50.0, 150.0),
        &font,
        48.0,
        &paint,
    );

    surface.save_png("text.png")?;
    Ok(())
}
```

## 📦 Project Structure

**Single unified crate** with modular architecture:

- **sina**: All-in-one rendering engine
  - Core types (geometry, color, paint, path, surface)
  - CPU backend (software rasterizer with raqote)
  - Text rendering (TrueType/OpenType with fontdue + rustybuzz)
  - GPU backend (WebGPU with wgpu - planned)
  - Effects and filters (planned)

## 🎯 Features

### Core Graphics

- [x] Geometry primitives (Point, Rect, Size)
- [x] Color management with RGBA
- [x] Path building with Bezier curves
- [x] Paint styles (fill, stroke, blend modes)
- [x] Backend abstraction (Surface/Canvas traits)

### Rendering

- [x] CPU rasterizer with anti-aliasing
- [x] Shapes: rectangles, circles, lines
- [x] Vector path rendering
- [x] Image export (PNG)
- [ ] GPU acceleration with WebGPU
- [ ] Gradients
- [ ] Blur and filters

### Text

- [x] TrueType (.ttf) font loading
- [x] OpenType (.otf) font support
- [x] Glyph rasterization with caching
- [x] Complex script shaping (Arabic, Hebrew, Devanagari, CJK)
- [x] Unicode support
- [x] Text layout with alignment
- [ ] Color emoji (COLR/CPAL)
- [ ] Font fallback chains

## 🎨 Examples

```bash
# Draw basic shapes
cargo run --example basic_shapes

# Text rendering (requires font file)
cargo run --example text_rendering
```

## 🧪 Building

```bash
cargo build --release
```

## 🧑‍💻 Running Tests

```bash
cargo test --workspace
```

## 📚 Documentation

- [API Documentation](https://docs.rs/sina)
- [GitHub Repository](https://github.com/GrandpaEJ/sina)

## 🛠️ Pure Rust Dependencies

| Crate          | Purpose                      |
| -------------- | ---------------------------- |
| **lyon**       | Vector path tessellation     |
| **raqote**     | CPU 2D rasterization         |
| **wgpu**       | GPU acceleration (WebGPU)    |
| **ttf-parser** | Font metadata parsing        |
| **fontdue**    | Glyph rasterization          |
| **rustybuzz**  | Text shaping (HarfBuzz port) |
| **glam**       | Vector/matrix mathematics    |
| **image**      | PNG encoding                 |

## 🗺️ Roadmap

### v0.2.0

- [ ] GPU backend with wgpu
- [ ] Linear and radial gradients
- [ ] Color emoji support

### v0.3.0

- [ ] Blur and shadow effects
- [ ] Image filters
- [ ] Clipping and masking

### Future

- [ ] SVG rendering
- [ ] Animation support
- [ ] Multi-threaded rendering

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

**Sina** - سینا - A treasure trove of 2D graphics capabilities! 🎨
