# Sina - سینا

A modern 2D graphics rendering engine written in pure Rust, inspired by Skia.

## Features

- **Pure Rust**: No C/C++ dependencies, fully safe Rust
- **Multi-backend**: CPU (software) and GPU (hardware-accelerated) rendering
- **Vector Graphics**: Path rendering with Bezier curves
- **Text Rendering**: Font loading, shaping, and emoji support (planned)
- **Modern API**: Clean, ergonomic graphics API

## Quick Start

```rust
use sina_core::{Color, Paint, Point, Rect, Surface};
use sina_cpu::CpuSurface;

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

    // Save to PNG
    surface.save_png("output.png")?;
    Ok(())
}
```

## Project Structure

**Single unified crate** with modular architecture:

- **sina-core**: All-in-one rendering engine
  - Core types (geometry, color, paint, path, surface)
  - CPU backend (software rasterizer)
  - GPU backend (WebGPU - planned)
  - Text rendering (planned)
  - Effects and filters (planned)

## Building

```bash
cargo build --release
```

## Running Examples

```bash
cargo run --bin basic_shapes
```

## Status

🚧 **Early Development** - Core API and CPU backend functional

- [x] Core geometry types
- [x] Path system with lyon integration
- [x] Paint and stroke styles
- [x] CPU rasterizer with raqote
- [x] Basic shapes (rect, circle, line)
- [ ] GPU backend with wgpu
- [ ] Text rendering
- [ ] Emoji support
- [ ] Gradients and effects

## License

MIT OR Apache-2.0
