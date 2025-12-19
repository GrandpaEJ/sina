//! Text layout and shaping
//!
//! Provides text shaping with rustybuzz for complex scripts.

use crate::Point;
use super::Font;

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// A positioned glyph after shaping
#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    pub glyph_index: u16,
    pub character: char,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
    pub y_advance: f32,
}

/// Result of text shaping
pub struct ShapedText {
    pub glyphs: Vec<ShapedGlyph>,
    pub width: f32,
    pub height: f32,
}

/// Text layout engine
pub struct TextLayout {
    font: Font,
    font_size: f32,
}

impl TextLayout {
    /// Create a new text layout with font and size
    pub fn new(font: Font, font_size: f32) -> Self {
        Self { font, font_size }
    }
    
    /// Shape text using rustybuzz for complex script support
    pub fn shape(&self, text: &str) -> ShapedText {
        // Create rustybuzz face from font data
        let raw_face = self.font.face().raw_face();
        let face = rustybuzz::Face::from_slice(raw_face.data, 0)
            .expect("Failed to create rustybuzz face");
        
        // Create buffer for shaping
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(text);
        
        // Shape the text
        let output = rustybuzz::shape(&face, &[], buffer);
        
        let mut glyphs = Vec::new();
        let mut x_position = 0.0;
        let mut max_height = 0.0;
        
        let positions = output.glyph_positions();
        let infos = output.glyph_infos();
        
        for (pos, info) in positions.iter().zip(infos.iter()) {
            let character = text.chars().nth(info.cluster as usize).unwrap_or(' ');
            
            // rustybuzz returns values in font design units, scale to font size
            let scale = self.font_size / self.font.units_per_em() as f32;
            
            let x_advance = pos.x_advance as f32 * scale;
            let y_advance = pos.y_advance as f32 * scale;
            
            let shaped_glyph = ShapedGlyph {
                glyph_index: info.glyph_id as u16,
                character,
                x_offset: pos.x_offset as f32 * scale,
                y_offset: pos.y_offset as f32 * scale,
                x_advance,
                y_advance,
            };
            
            glyphs.push(shaped_glyph);
            x_position += x_advance;
            
            let glyph_height = self.font.line_height(self.font_size);
            if glyph_height > max_height {
                max_height = glyph_height;
            }
        }
        
        ShapedText {
            glyphs,
            width: x_position,
            height: max_height,
        }
    }
    
    /// Layout text at a position with alignment
    pub fn layout(&self, text: &str, position: Point, align: TextAlign) -> Vec<(Point, ShapedGlyph)> {
        let shaped = self.shape(text);
        
        let x_offset = match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => -shaped.width / 2.0,
            TextAlign::Right => -shaped.width,
        };
        
        let mut result = Vec::new();
        let mut x = position.x + x_offset;
        let y = position.y;
        
        for glyph in shaped.glyphs {
            let glyph_pos = Point::new(
                x + glyph.x_offset,
                y + glyph.y_offset
            );
            
            result.push((glyph_pos, glyph.clone()));
            x += glyph.x_advance;
        }
        
        result
    }
    
    /// Measure text dimensions
    pub fn measure(&self, text: &str) -> (f32, f32) {
        let shaped = self.shape(text);
        (shaped.width, shaped.height)
    }
    
    /// Get the font
    pub fn font(&self) -> &Font {
        &self.font
    }
    
    /// Get the font size
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_align() {
        assert_eq!(TextAlign::Left, TextAlign::Left);
        assert_ne!(TextAlign::Left, TextAlign::Center);
    }
}
