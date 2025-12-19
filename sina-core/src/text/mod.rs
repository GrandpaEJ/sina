//! Text rendering, font loading, and typography
//!
//! Supports TrueType (.ttf) and OpenType (.otf) fonts using pure Rust.

mod bitmap;
mod emoji;
mod font;
mod glyph;
mod layout;
mod variable;

pub use bitmap::{BitmapFontRenderer, BitmapGlyph};
pub use emoji::{ColorEmojiRenderer, ColorLayer};
pub use font::{Font, FontError};
pub use glyph::{GlyphCache, GlyphFormat, RasterizedGlyph};
pub use layout::{ShapedText, TextAlign, TextLayout};
pub use variable::{VariableFontManager, Variation, VariationAxis};
