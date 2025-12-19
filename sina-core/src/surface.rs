//! Surface and canvas abstractions for drawing

use crate::{Color, Paint, Path, Point, Rect};

/// Generic surface for rendering
pub trait Surface {
    /// Get the width in pixels
    fn width(&self) -> u32;

    /// Get the height in pixels
    fn height(&self) -> u32;

    /// Get a mutable canvas for drawing
    fn canvas(&mut self) -> &mut dyn Canvas;

    /// Save the surface to a PNG file
    fn save_png(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

/// Canvas for drawing operations
pub trait Canvas {
    /// Clear the entire canvas with a color
    fn clear(&mut self, color: Color);

    /// Save the current drawing state
    fn save(&mut self);

    /// Restore the previous drawing state
    fn restore(&mut self);

    /// Draw a filled or stroked path
    fn draw_path(&mut self, path: &Path, paint: &Paint);

    /// Draw a rectangle
    fn draw_rect(&mut self, rect: Rect, paint: &Paint);

    /// Draw a circle
    fn draw_circle(&mut self, center: Point, radius: f32, paint: &Paint);

    /// Draw a line
    fn draw_line(&mut self, from: Point, to: Point, paint: &Paint);

    /// Draw text at a position
    fn draw_text(
        &mut self,
        text: &str,
        position: Point,
        font: &crate::text::Font,
        font_size: f32,
        paint: &Paint,
    );
}
