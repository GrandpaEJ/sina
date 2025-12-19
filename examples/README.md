# Sina Examples

This directory contains organized examples demonstrating Sina's rendering capabilities.

## Structure

Examples are organized by feature category:

### Shapes (`src/shapes/`)

- **basic_shapes.rs** - Rectangles, circles, lines, basic primitives
- **vector_paths.rs** - Bezier curves, triangles, stars, hearts
- **complex_shapes.rs** - Patterns, grids, overlapping shapes
- **gradients.rs** - Horizontal, vertical, radial, and multi-color gradients
- **patterns.rs** - Checkerboard, dots, diagonal lines, honeycomb
- **polygons.rs** - Regular polygons (pentagon, hexagon, octagon) and stars

### Text (`src/text/`)

- **text_samples.rs** - Comprehensive showcase of text rendering features
- **text_rendering.rs** - Basic text rendering
- **multi_font.rs** - Comparison of multiple font families
- **multilingual.rs** - Unicode and international text support
- **debug_text.rs** - Color contrast tests

### Misc (`src/misc/`)

- **benchmark.rs** - Performance benchmarking
- **font_formats.rs** - Advanced font feature demonstrations
- **test_metrics.rs** - Internal testing utilities

## Running Examples

Run any example with:

```bash
cargo run --bin <example_name>
```

For example:

```bash
cargo run --bin gradients
cargo run --bin text_samples
cargo run --bin polygons
```

## Output

All examples save their output to `examples/output/` organized by category:

- `examples/output/shapes/` - Shape rendering examples
- `examples/output/text/` - Text rendering examples
- `examples/output/misc/` - Benchmarks and utilities

To regenerate all examples:

```bash
# Shape examples
cargo run --bin basic_shapes
cargo run --bin vector_paths
cargo run --bin complex_shapes
cargo run --bin gradients
cargo run --bin patterns
cargo run --bin polygons

# Text examples
cargo run --bin text_samples
cargo run --bin multi_font
cargo run --bin multilingual

# Misc
cargo run --bin benchmark
```

## Features Demonstrated

### Rendering

- ✅ Primitive shapes (rectangles, circles, lines)
- ✅ Vector paths (Bezier curves, polygons)
- ✅ Gradient effects (simulated)
- ✅ Pattern generation
- ✅ Alpha blending and transparency

### Text

- ✅ TrueType/OpenType font loading
- ✅ Multiple font families
- ✅ Variable font sizes
- ✅ Color variations
- ✅ Unicode support
- ✅ International scripts

### Performance

- ✅ CPU rendering with raqote
- ✅ PNG export
- ✅ PNG export
- ✅ Performance benchmarking

### Emoji Support

Sina currently supports **CBDT/CBLC** (Google/Android style) bitmap color emojis (e.g., Noto Color Emoji) and has stubs for **COLR/CPAL** (Microsoft style) vector emojis.

```mermaid
flowchart TD
    A[Start: Character & Font] --> B[Load Font (ttf-parser)]
    B --> C[Map Unicode to Glyph Index]
    C --> D{Has Embedded Bitmap?}

    D -- Yes (CBDT/SBIX) --> E[Extract Bitmap Image]
    E --> F[Decode PNG/JPG (image crate)]
    F --> G[Resize to Target Font Size]
    G --> H[Store as RGBA Glyph]

    D -- No (Vector) --> I[Extract Outline (fontdue)]
    I --> J[Rasterize to Alpha Mask]
    J --> K[Store as Alpha Glyph]

    H --> L[Draw Text Loop]
    K --> L

    L --> M{Glyph Format?}
    M -- RGBA --> N[Direct Pixel Copy]
    M -- Alpha --> O[Apply Paint Color & Blend]

    N --> P[Final Canvas]
    O --> P
```

### Output Example

![Emoji Rendering](output/text/emoji_test.png)
