//! Paint styles and stroke configuration

use crate::Color;

/// Blend modes for compositing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Source over destination (default)
    SrcOver,
    /// Source
    Src,
    /// Destination
    Dst,
    /// Clear
    Clear,
    /// Multiply
    Multiply,
    /// Screen
    Screen,
    /// Overlay
    Overlay,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::SrcOver
    }
}

/// Stroke line cap style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Stroke line join style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// Stroke style configuration
#[derive(Debug, Clone)]
pub struct StrokeStyle {
    pub width: f32,
    pub cap: LineCap,
    pub join: LineJoin,
    pub miter_limit: f32,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            width: 1.0,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
            miter_limit: 4.0,
        }
    }
}

/// Paint style for drawing operations
#[derive(Debug, Clone)]
pub struct Paint {
    pub color: Color,
    pub anti_alias: bool,
    pub blend_mode: BlendMode,
    pub stroke: Option<StrokeStyle>,
}

impl Paint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_anti_alias(&mut self, enabled: bool) {
        self.anti_alias = enabled;
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode;
    }

    pub fn set_stroke(&mut self, stroke: StrokeStyle) {
        self.stroke = Some(stroke);
    }

    pub fn set_fill(&mut self) {
        self.stroke = None;
    }

    pub fn is_fill(&self) -> bool {
        self.stroke.is_none()
    }

    pub fn is_stroke(&self) -> bool {
        self.stroke.is_some()
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            anti_alias: true,
            blend_mode: BlendMode::SrcOver,
            stroke: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paint_defaults() {
        let paint = Paint::new();
        assert_eq!(paint.color, Color::BLACK);
        assert!(paint.anti_alias);
        assert!(paint.is_fill());
    }

    #[test]
    fn test_stroke_config() {
        let mut paint = Paint::new();
        paint.set_stroke(StrokeStyle {
            width: 5.0,
            ..Default::default()
        });
        assert!(paint.is_stroke());
    }
}
