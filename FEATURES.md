# Features

Comprehensive feature list for the Sina 2D rendering engine.

---

## Core Graphics

- [x] **Geometry Primitives**

  - Point (2D vector)
  - Size (width/height)
  - Rect (axis-aligned rectangle)
  - Matrix3x3 (2D transformations)

- [x] **Color Management**

  - RGBA 8-bit color
  - Premultiplied alpha
  - Color space conversions
  - Common color constants

- [x] **Path Building**

  - Move, line, quad, cubic commands
  - Bezier curves (quadratic, cubic)
  - Path closing and sub-paths
  - Integration with lyon for tessellation

- [x] **Paint Styles**

  - Fill and stroke modes
  - Blend modes (SrcOver, Multiply, Screen, etc.)
  - Stroke styles (caps, joins, width)
  - Anti-aliasing support

- [x] **Backend Abstraction**
  - Surface trait for backend-agnostic rendering
  - Canvas trait for drawing operations
  - State save/restore (planned)

---

## Rendering

### CPU Backend (Raqote)

- [x] **Software Rasterization**

  - High-quality anti-aliasing
  - Scanline rasterization
  - Porter-Duff compositing

- [x] **Shape Rendering**

  - Rectangles (filled and stroked)
  - Circles (filled and stroked)
  - Lines with configurable stroke
  - Vector path rendering

- [x] **Image Export**
  - PNG encoding
  - Direct pixel buffer access

### GPU Backend (WebGPU)

- [ ] **Hardware Acceleration**

  - wgpu integration
  - Shader pipeline (WGSL)
  - Texture management
  - Command buffer recording

- [ ] **Advanced Rendering**
  - Multi-sample anti-aliasing (MSAA)
  - GPU tessellation with lyon
  - Efficient vertex buffer management

---

## Text Rendering

### Font Support

- [x] **Font Formats**

  - TrueType (.ttf)
  - OpenType (.otf)
  - Font collections (.ttc)

- [x] **Font Loading**

  - From file path
  - From byte array
  - Font metrics (ascender, descender, line height)

- [x] **Font Information**
  - Family name extraction
  - Units per em
  - Glyph indices
  - Advance widths

### Glyph Rasterization

- [x] **Fontdue Integration**

  - Pure Rust glyph rasterization
  - SIMD optimizations
  - High-quality rendering

- [x] **Glyph Caching**

  - LRU cache (configurable size)
  - Per-size caching
  - Memory-efficient storage

- [x] **Color Conversion**
  - Grayscale alpha to RGBA
  - Paint color application
  - Alpha blending support

### Text Layout

- [x] **Text Shaping**

  - rustybuzz integration (HarfBuzz port)
  - Complex script support
  - Glyph positioning

- [x] **Layout Features**

  - Text alignment (Left, Center, Right)
  - Positioned glyph output
  - Text measurement
  - Line height calculation

- [x] **Script Support**
  - Latin (all variants)
  - Arabic (RTL, contextual forms)
  - Hebrew (RTL)
  - Devanagari (complex conjuncts)
  - CJK (Chinese, Japanese, Korean)
  - And many more via rustybuzz

### Emoji (Planned)

- [ ] **Color Emoji**

  - COLR/CPAL table support
  - Multi-layer rendering
  - Alpha compositing

- [ ] **Bitmap Emoji**

  - CBDT/CBLC support
  - Embedded PNG images

- [ ] **Emoji Sequences**
  - ZWJ sequences (👨‍👩‍👧‍👦)
  - Skin tone modifiers
  - Emoji variation selectors

---

## Effects & Filters

### Gradients (Planned)

- [ ] **Linear Gradient**

  - Multi-stop support
  - Color interpolation
  - Transform support

- [ ] **Radial Gradient**

  - Circular and elliptical
  - Multi-stop support
  - Focal point control

- [ ] **Conic Gradient**
  - Angular color distribution
  - Center point control

### Filters (Planned)

- [ ] **Blur**

  - Gaussian blur
  - Box blur (fast approximation)
  - Separable implementation

- [ ] **Image Filters**

  - Brightness/contrast
  - Color matrix transforms
  - Convolution kernels

- [ ] **Shadow Effects**
  - Drop shadows
  - Inner shadows
  - Blur radius control

---

## Advanced Features (Planned)

### Clipping & Masking

- [ ] **Clipping**

  - Rectangular clips
  - Path-based clipping
  - Clip stack

- [ ] **Masking**
  - Alpha masks
  - Luminance masks
  - Inverted masks

### Transform Stack

- [ ] **Transformations**

  - Translation
  - Rotation
  - Scaling
  - Arbitrary matrix transforms

- [ ] **Transform State**
  - Push/pop transform stack
  - Concatenation
  - Inverse transforms

### Image Support

- [ ] **Image Loading**

  - PNG, JPEG, WebP
  - Direct pixel buffer
  - Lazy loading

- [ ] **Image Drawing**
  - Scaled rendering
  - Tiled patterns
  - Nine-patch support

---

## Performance Features

- [x] **Glyph Caching** - LRU cache for rendered glyphs
- [x] **SIMD Optimizations** - Via fontdue and glam
- [x] **Parallel Processing** - Rayon for CPU parallelism (ready)
- [ ] **GPU Acceleration** - WebGPU backend (planned)
- [ ] **Incremental Rendering** - Dirty region tracking (planned)

---

## Developer Experience

- [x] **Pure Rust** - No C/C++ dependencies
- [x] **Type Safety** - Strong typing throughout
- [x] **Clean API** - Ergonomic, intuitive interface
- [x] **Comprehensive Tests** - Unit tests for all modules
- [x] **Documentation** - Inline docs and examples
- [ ] **Benchmarks** - Performance testing suite (planned)
