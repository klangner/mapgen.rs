//! Support function for 2D geometry
//! 

/// Position on the map
#[derive(Default, PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    /// Create new point
    pub fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }

    /// Create new point from i32 coords
    pub fn new_i32(x: i32, y: i32) -> Point {
        Point::new(x as usize, y as usize)
    }

    /// Euclidean distance to a given point
    pub fn distance_to(self, point: &Point) -> f32 {
        let a = (self.x as f32 - point.x as f32).powf(2.0);
        let b = (self.y as f32 - point.y as f32).powf(2.0);
        (a + b).sqrt()
    }
}

/// Rectangle region on the map
#[derive(PartialEq, Copy, Clone)]
pub struct Rect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Rect {
    pub fn new(x:i32, y: i32, width:i32, height:i32) -> Rect {
        Rect{x1:x, y1:y, x2:x+width, y2:y+height}
    }

    /// Returns true if this overlaps with other
    pub fn intersect(&self, other:&Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> Point {
        Point::new_i32((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
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
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 7);
        let distance = p1.distance_to(&p2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_intersect() {
        let rect1 = Rect::new(10, 10, 40, 40);
        let rect2 = Rect::new(30, 30, 60, 60);
        assert!(rect1.intersect(&rect2));
    }
}