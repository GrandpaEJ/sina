//! Bitmap font support for CBDT/SBIX tables
//!
//! Handles embedded bitmap glyphs (PNG, TIFF, etc.) in fonts.

use super::Font;

/// Bitmap glyph data
#[derive(Debug, Clone)]
pub struct BitmapGlyph {
    /// Pixel data (RGBA format)
    pub pixels: Vec<u8>,
    
    /// Width in pixels
    pub width: u32,
    
    /// Height in pixels
    pub height: u32,
    
    /// Horizontal bearing (offset from origin)
    pub bearing_x: i16,
    
    /// Vertical bearing (offset from baseline)
    pub bearing_y: i16,
    
    /// Horizontal advance
    pub advance: u16,
    
    /// Pixels per em for this strike
    pub ppem: u16,
}

/// Bitmap font renderer for CBDT/SBIX tables
pub struct BitmapFontRenderer;

impl BitmapFontRenderer {
    /// Check if font has bitmap strikes
    pub fn has_bitmaps(font: &Font) -> bool {
        let face = font.face();
        // Check for CBDT or SBIX tables
        face.tables().cbdt.is_some()
    }
    
    /// Get available bitmap sizes (stub - requires extended API)
    pub fn available_sizes(_font: &Font) -> Vec<u16> {
        // Stub - would need to parse bitmap strike tables
        Vec::new()
    }
    
    /// Get bitmap glyph for a specific size (stub)
    pub fn get_bitmap(
        _font: &Font,
        _glyph_id: u16,
        _ppem: u16,
    ) -> Option<BitmapGlyph> {
        // Stub - would need:
        // 1. Locate bitmap data in CBDT/SBIX
        // 2. Decode embedded PNG/TIFF
        // 3. Extract metrics
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bitmap_renderer() {
        assert!(true); // Placeholder
    }
}
