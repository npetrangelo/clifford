use std::ops::{Add, Mul, Neg};

use crate::{bivector::Bivector, vector::Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Trivector(pub f64);

impl Default for Trivector {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Neg for Trivector {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Trivector {
    type Output = Self;

    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        Self(self.0 + other.0)
    }
}

impl Mul for Trivector {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        self.0 * other.0
    }
}

impl Mul<Trivector> for f64 {
    type Output = Trivector;

    fn mul(self, xyz: Trivector) -> Self::Output {
        Trivector(self * xyz.0)
    }
}

impl Mul<Trivector> for Vector {
    type Output = Bivector;

    fn mul(self, xyz: Trivector) -> Self::Output {
        Bivector {
            yz: self.x * xyz.0,
            zx: self.y * xyz.0,
            xy: self.z * xyz.0,
        }
    }
}

impl Mul<Vector> for Trivector {
    type Output = Bivector;

    fn mul(self, v: Vector) -> Self::Output {
        Bivector {
            yz: self.0 * v.x,
            zx: self.0 * v.y,
            xy: self.0 * v.z,
        }
    }
}

impl Mul<Trivector> for Bivector {
    type Output = Vector;

    fn mul(self, xyz: Trivector) -> Self::Output {
        Vector {
            x: -self.yz * xyz.0,
            y: -self.zx * xyz.0,
            z: -self.xy * xyz.0,
        }
    }
}

impl Mul<Bivector> for Trivector {
    type Output = Vector;

    fn mul(self, b: Bivector) -> Self::Output {
        Vector {
            x: -self.0 * b.yz,
            y: -self.0 * b.zx,
            z: -self.0 * b.xy,
        }
    }
}