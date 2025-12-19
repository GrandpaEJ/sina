//! Font loading and management
//!
//! Supports TrueType (.ttf) and OpenType (.otf) font formats.

use std::sync::Arc;
use thiserror::Error;

/// Font loading errors
#[derive(Debug, Error)]
pub enum FontError {
    #[error("Failed to parse font: {0}")]
    ParseError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Glyph not found for character: {0}")]
    GlyphNotFound(char),
    
    #[error("Invalid font index: {0}")]
    InvalidIndex(u32),
}

/// A loaded font supporting TrueType and OpenType formats
pub struct Font {
    /// Font data (shared reference for efficiency)
    data: Arc<Vec<u8>>,
    
    /// Parsed font face
    face: ttf_parser::Face<'static>,
    
    /// Fontdue font for rasterization
    fontdue_font: fontdue::Font,
}

impl Font {
    /// Load a font from a file path
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, FontError> {
        let data = std::fs::read(path)?;
        Self::from_bytes(data)
    }
    
    /// Load a font from bytes
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, FontError> {
        let data = Arc::new(data);
        
        // Parse with ttf-parser for metadata
        let face = {
            // SAFETY: We ensure the data lives as long as the Font struct
            let data_ref: &[u8] = unsafe {
                std::slice::from_raw_parts(data.as_ptr(), data.len())
            };
            ttf_parser::Face::parse(data_ref, 0)
                .map_err(|e| FontError::ParseError(format!("{:?}", e)))?
        };
        
        let fontdue_font = fontdue::Font::from_bytes(
            data.as_ref().as_slice(),
            fontdue::FontSettings::default()
        ).map_err(|e| FontError::ParseError(e.to_string()))?;
        
        // SAFETY: Transmute to 'static lifetime - we guarantee Font owns the data
        let face = unsafe {
            std::mem::transmute::<ttf_parser::Face<'_>, ttf_parser::Face<'static>>(face)
        };
        
        Ok(Self {
            data,
            face,
            fontdue_font,
        })
    }
    
    /// Get the font family name
    pub fn family_name(&self) -> Option<String> {
        self.face.names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::FAMILY)
            .and_then(|name| name.to_string())
    }
    
    /// Get the font's units per em
    pub fn units_per_em(&self) -> u16 {
        self.face.units_per_em()
    }
    
    /// Get the ascender (height above baseline)
    pub fn ascender(&self) -> i16 {
        self.face.ascender()
    }
    
    /// Get the descender (depth below baseline)
    pub fn descender(&self) -> i16 {
        self.face.descender()
    }
    
    /// Get the line gap
    pub fn line_gap(&self) -> i16 {
        self.face.line_gap()
    }
    
    /// Calculate line height at given font size
    pub fn line_height(&self, font_size: f32) -> f32 {
        let units_per_em = self.units_per_em() as f32;
        let ascent = self.ascender() as f32;
        let descent = self.descender().abs() as f32;
        let line_gap = self.line_gap() as f32;
        
        ((ascent + descent + line_gap) / units_per_em) * font_size
    }
    
    /// Get glyph index for a character
    pub fn glyph_index(&self, character: char) -> Option<u16> {
        self.face.glyph_index(character).map(|id| id.0)
    }
    
    /// Get horizontal advance for a glyph at given font size
    pub fn glyph_advance(&self, glyph_id: u16, font_size: f32) -> f32 {
        let glyph_id = ttf_parser::GlyphId(glyph_id);
        if let Some(advance) = self.face.glyph_hor_advance(glyph_id) {
            let units_per_em = self.units_per_em() as f32;
            (advance as f32 / units_per_em) * font_size
        } else {
            0.0
        }
    }
    
    /// Get fontdue font reference for rasterization
    pub(crate) fn fontdue_font(&self) -> &fontdue::Font {
        &self.fontdue_font
    }
    
    /// Get ttf-parser face reference
    pub(crate) fn face(&self) -> &ttf_parser::Face {
        &self.face
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        // Clone by recreating from shared data
        Self::from_bytes((*self.data).clone())
            .expect("Font clone should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_font_basics() {
        // This test would need an actual font file
        // For now, just verify the API compiles
        assert!(true);
    }
}
