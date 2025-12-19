//! Color representation and manipulation

use bytemuck::{Pod, Zeroable};

/// RGBA color with 8-bit channels
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color from RGBA values (0-255)
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new color from RGB values with full opacity
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Create a color from normalized RGBA values (0.0-1.0)
    pub fn rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }

    /// Convert to premultiplied alpha
    pub fn premultiply(&self) -> Self {
        let alpha = self.a as f32 / 255.0;
        Self {
            r: (self.r as f32 * alpha) as u8,
            g: (self.g as f32 * alpha) as u8,
            b: (self.b as f32 * alpha) as u8,
            a: self.a,
        }
    }

    /// Common colors
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgba(255, 128, 64, 200);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 200);
    }

    #[test]
    fn test_premultiply() {
        let color = Color::rgba(255, 128, 64, 128);
        let premul = color.premultiply();
        assert_eq!(premul.a, 128);
        assert!(premul.r < color.r);
    }
}
