//! Text rendering, font loading, and typography
//!
//! Supports TrueType (.ttf) and OpenType (.otf) fonts using pure Rust.

mod font;
mod glyph;
mod layout;
mod emoji;
mod bitmap;
mod variable;

pub use font::{Font, FontError};
pub use glyph::{GlyphCache, RasterizedGlyph, GlyphFormat};
pub use layout::{TextLayout, TextAlign, ShapedText};
pub use emoji::{ColorEmojiRenderer, ColorLayer};
pub use bitmap::{BitmapFontRenderer, BitmapGlyph};
pub use variable::{VariableFontManager, VariationAxis, Variation};
