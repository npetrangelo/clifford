use std::ops::{Add, Mul, Neg};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const X: Vector = Vector { x: 1.0, y: 0.0, z: 0.0 };
pub const Y: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
pub const Z: Vector = Vector { x: 0.0, y: 0.0, z: 1.0 };

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

use crate::bivector::Bivector;

impl Mul for Vector {
    type Output = (f64, Bivector);

    fn mul(self, other: Vector) -> Self::Output {
        (self.x * other.x + self.y * other.y + self.z * other.z,
        Bivector {
            yz: self.y * other.z - self.z * other.y,
            zx: self.z * other.x - self.x * other.z,
            xy: self.x * other.y - self.y * other.x,
        })
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bivector::{YZ, ZX, XY};

    #[test]
    fn test_add() {
        let v1 = Vector { x: 1.0, y: 1.0, z: 0.0 };
        let v2 = Vector { x: 0.0, y: 1.0, z: 3.0 };
        assert_eq!(v1 + v2, Vector { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn test_scale() {
        let v = Vector { x: 0.0, y: 1.0, z: 3.0 };
        assert_eq!(2.0 * v, Vector { x: 0.0, y: 2.0, z: 6.0 });
    }

    #[test]
    fn test_mul() {
        let (dot, wedge) = X * (X + Y);
        assert_eq!(dot, 1.0);
        assert_eq!(wedge, XY);
    }
}
