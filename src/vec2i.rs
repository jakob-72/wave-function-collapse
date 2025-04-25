use std::ops::{Add, Sub};

pub const UP: Vec2i = Vec2i { x: 0, y: -1 };
pub const RIGHT: Vec2i = Vec2i { x: 1, y: 0 };

pub const DOWN: Vec2i = Vec2i { x: 0, y: 1 };

pub const LEFT: Vec2i = Vec2i { x: -1, y: 0 };

/// Convenience struct for 2D integer Vectors/Points.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Vec2i {
    pub fn inv(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vec1 = Vec2i { x: 1, y: 2 };
        let vec2 = Vec2i { x: 3, y: 4 };
        let result = vec1 + vec2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec2i { x: 5, y: 7 };
        let vec2 = Vec2i { x: 2, y: 3 };
        let result = vec1 - vec2;
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 4);
    }

    #[test]
    fn test_vec2i_equality() {
        let vec1 = Vec2i { x: 1, y: 2 };
        let vec2 = Vec2i { x: 1, y: 2 };
        let vec3 = vec1 + UP;
        assert_eq!(vec1, vec2);
        assert_ne!(vec1, vec3);
    }

    #[test]
    fn test_vec2i_inv() {
        assert_eq!(UP.inv(), DOWN);
        assert_eq!(DOWN.inv(), UP);
        assert_eq!(LEFT.inv(), RIGHT);
        assert_eq!(RIGHT.inv(), LEFT);
    }
}
