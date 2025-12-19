//! # Sina Core
//!
//! Core data types and abstractions for the Sina 2D rendering engine.
//!
//! This crate provides fundamental types for:
//! - Geometry (Point, Rect, Size, Matrix)
//! - Colors and color spaces
//! - Paths and path building
//! - Paint styles and stroke settings
//! - Surface abstraction
//! - CPU and GPU rendering backends
//! - Text rendering and emoji support
//! - Effects and filters

pub mod color;
pub mod geometry;
pub mod paint;
pub mod path;
pub mod surface;

// Backends
pub mod cpu;
pub mod gpu;

// Features
pub mod effects;
pub mod text;

// Re-export commonly used types
pub use color::Color;
pub use geometry::{Matrix3x3, Point, Rect, Size};
pub use paint::{BlendMode, Paint, StrokeStyle};
pub use path::Path;
pub use surface::{Canvas, Surface};

// Re-export backends
pub use cpu::CpuSurface;
pub use gpu::GpuSurface;

// Re-export text rendering
pub use text::{
    BitmapFontRenderer, BitmapGlyph, ColorEmojiRenderer, ColorLayer, Font, FontError, GlyphCache,
    TextAlign, TextLayout, VariableFontManager, Variation, VariationAxis,
};
