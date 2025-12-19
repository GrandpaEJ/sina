//! Text rendering, font loading, and typography
//!
//! Supports TrueType (.ttf) and OpenType (.otf) fonts using pure Rust.

mod font;
mod glyph;
mod layout;

pub use font::{Font, FontError};
pub use glyph::{GlyphCache, RasterizedGlyph};
pub use layout::{TextLayout, TextAlign, ShapedText};
