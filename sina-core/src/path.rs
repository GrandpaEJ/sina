//! Path representation and building

use crate::geometry::Point;
use lyon_path::Path as LyonPath;

/// 2D vector path
#[derive(Debug, Clone)]
pub struct Path {
    inner: LyonPath,
}

impl Path {
    /// Create a new path builder
    pub fn builder() -> PathBuilder {
        PathBuilder::new()
    }

    /// Get the underlying lyon path
    pub fn lyon_path(&self) -> &LyonPath {
        &self.inner
    }
}

/// Path builder for constructing paths
pub struct PathBuilder {
    builder: lyon_path::path::Builder,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            builder: LyonPath::builder(),
        }
    }

    /// Move to a point without drawing
    pub fn move_to(mut self, point: Point) -> Self {
        self.builder.begin(lyon_path::geom::point(point.x, point.y));
        self
    }

    /// Draw a line to a point
    pub fn line_to(mut self, point: Point) -> Self {
        self.builder
            .line_to(lyon_path::geom::point(point.x, point.y));
        self
    }

    /// Draw a quadratic Bezier curve
    pub fn quad_to(mut self, control: Point, to: Point) -> Self {
        self.builder.quadratic_bezier_to(
            lyon_path::geom::point(control.x, control.y),
            lyon_path::geom::point(to.x, to.y),
        );
        self
    }

    /// Draw a cubic Bezier curve
    pub fn cubic_to(mut self, control1: Point, control2: Point, to: Point) -> Self {
        self.builder.cubic_bezier_to(
            lyon_path::geom::point(control1.x, control1.y),
            lyon_path::geom::point(control2.x, control2.y),
            lyon_path::geom::point(to.x, to.y),
        );
        self
    }

    /// Close the current sub-path
    pub fn close(mut self) -> Self {
        self.builder.end(true);
        self
    }

    /// Build the final path
    pub fn build(self) -> Path {
        Path {
            inner: self.builder.build(),
        }
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_builder() {
        let path = Path::builder()
            .move_to(Point::new(0.0, 0.0))
            .line_to(Point::new(100.0, 0.0))
            .line_to(Point::new(100.0, 100.0))
            .close()
            .build();

        assert!(path.lyon_path().iter().count() > 0);
    }
}
