//! Support function for 2D/3D geometry
//!

use glam::UVec2;

/// Rectangle region on the map
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    /// Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> UVec2 {
        UVec2::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn width(&self) -> u32 {
        if self.x2 >= self.x1 {
            self.x2 - self.x1
        } else {
            self.x1 - self.x2
        }
    }

    pub fn height(&self) -> u32 {
        if self.y2 >= self.y1 {
            self.y2 - self.y1
        } else {
            self.y1 - self.y2
        }
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let rect1 = Rect::new(10, 10, 40, 40);
        let rect2 = Rect::new(30, 30, 60, 60);
        assert!(rect1.intersect(&rect2));
    }

    #[test]
    fn test_size() {
        let rect1 = Rect::new(10, 10, 40, 30);
        assert_eq!(rect1.width(), 40);
        assert_eq!(rect1.height(), 30);
    }
}
