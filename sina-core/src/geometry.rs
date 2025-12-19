//! Geometric primitives and transformations

use glam::{Vec2, Mat3};

/// 2D point
pub type Point = Vec2;

/// 2D size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// Axis-aligned rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::new(x, y, width, height)
    }

    pub fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        }
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }
}

/// 3x3 transformation matrix for 2D affine transforms
pub type Matrix3x3 = Mat3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_basics() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.bottom(), 70.0);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(rect.contains(Point::new(50.0, 50.0)));
        assert!(!rect.contains(Point::new(150.0, 50.0)));
    }
}
