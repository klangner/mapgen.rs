//! Support function for 2D geometry
//!

/// Position on the map
#[derive(Default, PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl Vec2u {
    /// Create new point
    pub fn new(x: u32, y: u32) -> Vec2u {
        Vec2u { x, y }
    }

    /// Euclidean distance to a given point
    pub fn distance_to(self, point: &Vec2u) -> f32 {
        let a = (self.x as f32 - point.x as f32).powf(2.0);
        let b = (self.y as f32 - point.y as f32).powf(2.0);
        (a + b).sqrt()
    }
}

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

    pub fn center(&self) -> Vec2u {
        Vec2u::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
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

/// Calculate abs difference value between 2 u32 values
/// Example:
/// ```
/// use mapgen::geometry::diff_abs;
///
/// assert_eq!(diff_abs(5, 3), 2);
/// assert_eq!(diff_abs(3, 5), 2);
/// ```
pub fn diff_abs(x: u32, y: u32) -> u32 {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Vec2u::new(10, 10);
        let p2 = Vec2u::new(14, 7);
        let distance = p1.distance_to(&p2);
        assert_eq!(distance, 5.0);
    }

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
