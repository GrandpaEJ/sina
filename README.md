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
    let mut surface = CpuSurface::new(800, 600);
    surface.canvas().clear(Color::WHITE);

    // Draw shapes
    let paint = Paint::with_color(Color::RED);
    surface.canvas().draw_rect(Rect::new(100.0, 100.0, 200.0, 150.0), &paint);

    surface.save_png("output.png")?;
    Ok(())
}
```

### Drawing Text

```rust
use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut surface = CpuSurface::new(800, 400);
    let font = Font::from_file("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")?;

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

- **sina**: All-in-one rendering engine
  - Core types (geometry, color, paint, path, surface)
  - CPU backend (raqote software rasterizer)
  - Text rendering (fontdue + rustybuzz)
  - GPU backend (wgpu - planned)

## � Examples

```bash
cargo run --example basic_shapes
cargo run --example text_rendering  # requires font file
```

## 📚 Documentation

- **[Features](FEATURES.md)** - Comprehensive feature list and status
- **[Roadmap](TODO.md)** - Development roadmap and planned features
- [API Docs](https://docs.rs/sina) - Full API documentation
- [GitHub](https://github.com/GrandpaEJ/sina) - Source code

## 🛠️ Pure Rust Stack

| Crate          | Purpose                  |
| -------------- | ------------------------ |
| **lyon**       | Vector path tessellation |
| **raqote**     | CPU 2D rasterization     |
| **wgpu**       | GPU acceleration         |
| **ttf-parser** | Font metadata            |
| **fontdue**    | Glyph rasterization      |
| **rustybuzz**  | Text shaping             |
| **glam**       | Mathematics              |
| **image**      | PNG encoding             |

## 🧪 Building & Testing

```bash
# Build
cargo build --release

# Test
cargo test --workspace
```

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🤝 Contributing

Contributions welcome! Please submit a Pull Request.

---

**Sina** - سینا - A treasure trove of 2D graphics capabilities! 🎨
