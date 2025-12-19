//! Glyph rasterization and caching

use crate::Color;
use std::collections::HashMap;

/// A rasterized glyph with pixel data
#[derive(Clone)]
pub struct RasterizedGlyph {
    /// Pixel data (grayscale alpha values 0-255)
    pub pixels: Vec<u8>,
    
    /// Glyph width in pixels
    pub width: usize,
    
    /// Glyph height in pixels
    pub height: usize,
    
    /// Horizontal bearing (offset from origin)
    pub bearing_x: f32,
    
    /// Vertical bearing (offset from baseline)
    pub bearing_y: f32,
    
    /// Horizontal advance to next glyph
    pub advance: f32,
}

impl RasterizedGlyph {
    /// Create colored RGBA pixels from grayscale alpha
    pub fn to_rgba(&self, color: Color) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(self.pixels.len() * 4);
        
        for &alpha in &self.pixels {
            rgba.push(color.r);
            rgba.push(color.g);
            rgba.push(color.b);
            rgba.push(((alpha as u16 * color.a as u16) / 255) as u8);
        }
        
        rgba
    }
}

/// Cache key for glyph lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GlyphKey {
    glyph_index: u16,
    font_size_scaled: u32, // Font size * 100 to handle fractional sizes
}

/// LRU cache for rasterized glyphs
pub struct GlyphCache {
    cache: HashMap<GlyphKey, RasterizedGlyph>,
    max_size: usize,
}

impl GlyphCache {
    /// Create a new glyph cache with specified capacity
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }
    
    /// Get or rasterize a glyph
    pub fn get_or_rasterize(
        &mut self,
        font: &super::Font,
        character: char,
        font_size: f32,
    ) -> Option<RasterizedGlyph> {
        let glyph_index = font.glyph_index(character)?;
        let key = GlyphKey {
            glyph_index,
            font_size_scaled: (font_size * 100.0) as u32,
        };
        
        // Check cache first
        if let Some(glyph) = self.cache.get(&key) {
            return Some(glyph.clone());
        }
        
        // Rasterize using fontdue
        let (metrics, pixels) = font.fontdue_font()
            .rasterize(character, font_size);
        
        let glyph = RasterizedGlyph {
            pixels,
            width: metrics.width,
            height: metrics.height,
            bearing_x: metrics.xmin as f32,
            bearing_y: metrics.ymin as f32,
            advance: metrics.advance_width,
        };
        
        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            if let Some(key) = self.cache.keys().next().copied() {
                self.cache.remove(&key);
            }
        }
        
        self.cache.insert(key, glyph.clone());
        Some(glyph)
    }
    
    /// Get or rasterize a glyph by its index
    pub fn get_or_rasterize_indexed(
        &mut self,
        font: &super::Font,
        glyph_index: u16,
        font_size: f32,
    ) -> Option<RasterizedGlyph> {
        let key = GlyphKey {
            glyph_index,
            font_size_scaled: (font_size * 100.0) as u32,
        };
        
        // Check cache first
        if let Some(glyph) = self.cache.get(&key) {
            return Some(glyph.clone());
        }
        
        // Rasterize using fontdue by index
        let (metrics, pixels) = font.fontdue_font()
            .rasterize_indexed(glyph_index, font_size);
        
        let glyph = RasterizedGlyph {
            pixels,
            width: metrics.width,
            height: metrics.height,
            bearing_x: metrics.xmin as f32,
            bearing_y: metrics.ymin as f32,
            advance: metrics.advance_width,
        };
        
        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            if let Some(key) = self.cache.keys().next().copied() {
                self.cache.remove(&key);
            }
        }
        
        self.cache.insert(key, glyph.clone());
        Some(glyph)
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    /// Get current cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for GlyphCache {
    fn default() -> Self {
        Self::new(1000) // Default cache size: 1000 glyphs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_glyph_cache() {
        let mut cache = GlyphCache::new(2);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
    
    #[test]
    fn test_rgba_conversion() {
        let glyph = RasterizedGlyph {
            pixels: vec![255, 128, 0],
            width: 3,
            height: 1,
            bearing_x: 0.0,
            bearing_y: 0.0,
            advance: 10.0,
        };
        
        let color = Color::rgb(255, 0, 0);
        let rgba = glyph.to_rgba(color);
        
        assert_eq!(rgba.len(), 12); // 3 pixels * 4 channels
        assert_eq!(rgba[0..4], [255, 0, 0, 255]); // First pixel
        assert_eq!(rgba[4..8], [255, 0, 0, 128]); // Second pixel
        assert_eq!(rgba[8..12], [255, 0, 0, 0]); // Third pixel
    }
}
