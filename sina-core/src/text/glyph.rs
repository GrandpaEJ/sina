//! Glyph rasterization and caching

use crate::Color;
use image::GenericImageView;
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GlyphFormat {
    /// Grayscale alpha mask (standard text)
    Alpha,
    /// RGBA color bitmap (colored emojis)
    Rgba,
}

/// A rasterized glyph with pixel data
#[derive(Clone)]
pub struct RasterizedGlyph {
    /// Pixel data
    /// - For Alpha: grayscale values 0-255
    /// - For Rgba: standard RGBA bytes
    pub pixels: Vec<u8>,

    /// Format of the pixel data
    pub format: GlyphFormat,

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
        let (metrics, pixels) = font.fontdue_font().rasterize(character, font_size);

        let glyph = RasterizedGlyph {
            pixels,
            width: metrics.width,
            height: metrics.height,
            bearing_x: metrics.xmin as f32,
            bearing_y: metrics.ymin as f32,
            advance: metrics.advance_width,
            format: GlyphFormat::Alpha,
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

        let face = font.face();
        let glyph_id = ttf_parser::GlyphId(glyph_index);

        // Try to load a color bitmap (CBDT/CBLC/SBIX)
        // 1. Try to get raster image at requested size directly
        let target_ppem = font_size as u16;
        let mut raster_image_opt = face.glyph_raster_image(glyph_id, target_ppem);

        // If exact size not found, try common emoji bitmap sizes
        if raster_image_opt.is_none() {
            for &size in &[109, 128, 96, 64, 32] {
                if let Some(img) = face.glyph_raster_image(glyph_id, size) {
                    raster_image_opt = Some(img);
                    break;
                }
            }
        }

        // 2. Try to get raster image
        if let Some(raster_image) = raster_image_opt {
            // Use the actual size of the found bitmap for scaling
            // ttf-parser might return a bitmap even if the requested size doesn't match perfectly
            let found_ppem = raster_image.pixels_per_em;

            // 3. Decode image
            if let Ok(img) = image::load_from_memory(raster_image.data) {
                // 4. Resize to requested font size
                let (w, h) = img.dimensions();

                // Target height is usually close to font_size, or em size.
                let scale = font_size / found_ppem as f32;
                let target_w = (w as f32 * scale).round() as u32;
                let target_h = (h as f32 * scale).round() as u32;

                let resized = img.resize(target_w, target_h, image::imageops::FilterType::Lanczos3);
                let rgba_data = resized.to_rgba8().into_vec();
                let (new_w, new_h) = resized.dimensions();

                // Get vector metrics for advance/bearing
                let units_per_em = face.units_per_em() as f32;
                let advance_width = face.glyph_hor_advance(glyph_id).unwrap_or(0);
                let advance = (advance_width as f32 / units_per_em) * font_size;

                // FIX: Position the emoji on the baseline.
                // Previous attempt used descender, which might be too low.
                // Most bitmaps are full-height or centered, but anchoring bottom to baseline is a safe default.
                // A small negative bearing (descent) might be appropriate but 0.0 is safer than full descender.
                let bearing_y = 0.0; // Align bottom to baseline

                // Use vector side bearing if available, else scaled bitmap x
                let lsb = face.glyph_hor_side_bearing(glyph_id).unwrap_or(0);
                let bearing_x = (lsb as f32 / units_per_em) * font_size;

                if cfg!(debug_assertions) {
                    // Check center pixel of the resized image to verify color
                    // Removed debug print
                }

                let glyph = RasterizedGlyph {
                    pixels: rgba_data,
                    width: new_w as usize,
                    height: new_h as usize,
                    bearing_x,
                    bearing_y,
                    advance,
                    format: GlyphFormat::Rgba,
                };

                // Cache and return
                if self.cache.len() >= self.max_size {
                    if let Some(k) = self.cache.keys().next().copied() {
                        self.cache.remove(&k);
                    }
                }
                self.cache.insert(key, glyph.clone());
                return Some(glyph);
            }
        }

        // 3. Try COLR/CPAL (Layered Vectors)
        // FIXME: ttf-parser 0.20 API mismatch for COLR/CPAL tables.
        // Fields `cpal` not found on tables(), and `RgbaColor` fields mismatch.
        // Disabling COLR support for now. CBDT bitmaps (Noto Color Emoji) are supported.
        /*
        if let (Some(colr), Some(cpal)) = (face.tables().colr, face.tables().cpal) {
             let mut layers = colr.get(glyph_id);
             // ... implementation commented out ...
        }
        */

        // Rasterize using fontdue by index (fallback for non-bitmap glyphs)
        let (metrics, pixels) = font
            .fontdue_font()
            .rasterize_indexed(glyph_index, font_size);

        let glyph = RasterizedGlyph {
            pixels,
            width: metrics.width,
            height: metrics.height,
            bearing_x: metrics.xmin as f32,
            bearing_y: metrics.ymin as f32,
            advance: metrics.advance_width,
            format: GlyphFormat::Alpha,
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
            format: GlyphFormat::Alpha,
        };

        let color = Color::rgb(255, 0, 0);
        let rgba = glyph.to_rgba(color);

        assert_eq!(rgba.len(), 12); // 3 pixels * 4 channels
        assert_eq!(rgba[0..4], [255, 0, 0, 255]); // First pixel
        assert_eq!(rgba[4..8], [255, 0, 0, 128]); // Second pixel
        assert_eq!(rgba[8..12], [255, 0, 0, 0]); // Third pixel
    }
}
