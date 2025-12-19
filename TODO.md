# TODO / Roadmap

Development roadmap and planned features for Sina.

---

## v0.2.0 - GPU & Effects

### GPU Backend

- [ ] **wgpu Integration**

  - Initialize GPU device and queue
  - Create render pipeline
  - Shader compilation (WGSL)
  - Texture management

- [ ] **GPU Surface Implementation**

  - Implement Surface trait
  - Implement Canvas trait
  - Command buffer recording
  - Present to screen/texture

- [ ] **Path Tessellation**
  - Lyon GPU tessellation
  - Vertex buffer generation
  - Index buffer optimization
  - Efficient batching

### Gradients

- [ ] **Linear Gradient**

  - Gradient shader (WGSL)
  - Color stop interpolation
  - Transform support
  - CPU fallback

- [ ] **Radial Gradient**
  - Circular gradient shader
  - Focal point support
  - Multi-stop colors
  - CPU fallback

### Color Emoji

- [ ] **COLR/CPAL Support**

  - Parse COLR table
  - Extract color layers
  - Layer compositing
  - Alpha blending

- [ ] **Bitmap Emoji**
  - CBDT/CBLC table parsing
  - Embedded PNG extraction
  - Size selection

---

## v0.3.0 - Advanced Rendering

### Effects & Filters

- [ ] **Blur Filter**

  - Gaussian blur implementation
  - Box blur (fast path)
  - Separable blur passes
  - GPU acceleration

- [ ] **Shadow Effects**

  - Drop shadow
  - Inner shadow
  - Blur radius control
  - Offset and color

- [ ] **Image Filters**
  - Brightness/Contrast
  - Saturation/Hue
  - Color matrix
  - Convolution kernels

### Clipping & Masking

- [ ] **Clipping Regions**

  - Rectangular clips
  - Path-based clipping
  - Clip stack management
  - Intersection/union

- [ ] **Alpha Masking**
  - Mask from image
  - Mask from path
  - Inverted masks
  - Mask caching

### Transform Stack

- [ ] **Full Transform Support**

  - Save/restore state
  - Transform concatenation
  - Inverse transforms
  - Transform stack

- [ ] **Transform Operations**
  - Translate
  - Rotate
  - Scale
  - Skew
  - Arbitrary matrix

---

## v0.4.0 - Image & SVG

### Image Support

- [ ] **Image Loading**

  - PNG decoder
  - JPEG decoder
  - WebP support
  - Direct pixel buffer

- [ ] **Image Rendering**

  - Scaled drawing
  - Tiled patterns
  - Nine-patch support
  - Image caching

- [ ] **Image Filters**
  - Resize with quality
  - Format conversion
  - Color space conversion

### SVG Rendering

- [ ] **SVG Parser**

  - Parse SVG paths
  - Parse SVG styles
  - Transform support
  - Group hierarchies

- [ ] **SVG Elements**
  - Paths
  - Rectangles/Circles
  - Text
  - Gradients
  - Filters

---

## Future / Backlog

### Text Enhancements

- [ ] **Font Fallback**

  - Fallback chain
  - Automatic substitution
  - Coverage detection

- [ ] **Advanced Typography**

  - Kerning improvements
  - Ligatures
  - OpenType features
  - Variable fonts

- [ ] **Text Decorations**

  - Underline
  - Strikethrough
  - Overline
  - Custom decorations

- [ ] **Vertical Text**
  - Top-to-bottom layout
  - Vertical metrics
  - CJK vertical support

### Performance

- [ ] **Benchmarking Suite**

  - Rendering benchmarks
  - Text shaping benchmarks
  - Glyph cache benchmarks
  - Memory profiling

- [ ] **Optimizations**

  - Multi-threaded rendering
  - Incremental rendering
  - Dirty region tracking
  - Command batching

- [ ] **GPU Optimizations**
  - Persistent buffers
  - Compute shaders
  - Indirect drawing
  - Texture atlases

### Platform Support

- [ ] **Window Integration**

  - winit integration
  - Raw window handle
  - Event handling
  - DPI scaling

- [ ] **Web Support**
  - WebAssembly target
  - Canvas backend
  - WebGPU in browser
  - WASM examples

### Animation

- [ ] **Animation Framework**

  - Keyframe animations
  - Easing functions
  - Path animations
  - Transform animations

- [ ] **Particle Systems**
  - Particle emitters
  - Physics simulation
  - GPU-accelerated particles

### 3D Integration

- [ ] **2D on 3D**
  - Render to texture
  - 3D transforms
  - Perspective correction

---

## Community & Documentation

### Documentation

- [ ] **Tutorial Series**

  - Getting started guide
  - Shape rendering tutorial
  - Text rendering guide
  - Advanced techniques

- [ ] **API Examples**

  - More example programs
  - Code snippets
  - Best practices
  - Performance tips

- [ ] **Book/Guide**
  - Comprehensive guide
  - Architecture overview
  - Implementation details
  - Contributing guide

### Tooling

- [ ] **Hot Reload**

  - Live preview
  - Asset reloading
  - Shader hot reload

- [ ] **Profiling**
  - Built-in profiler
  - GPU timeline
  - Memory tracking
  - Performance visualization

### Testing

- [ ] **Visual Regression Tests**

  - Baseline images
  - Pixel-perfect comparison
  - Automated CI testing

- [ ] **Fuzzing**
  - Font parsing fuzzing
  - Path fuzzing
  - Shader compilation fuzzing

---

## Research / Exploration

- [ ] **Signed Distance Fields (SDF)**

  - SDF text rendering
  - Vector SDF
  - Effects via SDF

- [ ] **Compute Shaders**

  - GPU-accelerated filters
  - Image processing
  - Particle systems

- [ ] **Multi-GPU Support**

  - Device selection
  - Work distribution
  - Synchronization

- [ ] **Rust GPU Integration**
  - rust-gpu for shaders
  - Type-safe shaders
  - Shared CPU/GPU code
