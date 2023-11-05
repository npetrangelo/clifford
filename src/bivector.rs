use std::ops::{Add, Mul, Neg};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bivector {
    pub yz: f64,
    pub zx: f64,
    pub xy: f64,
}

pub const YZ: Bivector = Bivector { yz: 1.0, zx: 0.0, xy: 0.0 };
pub const ZX: Bivector = Bivector { yz: 0.0, zx: 1.0, xy: 0.0 };
pub const XY: Bivector = Bivector { yz: 0.0, zx: 0.0, xy: 1.0 };

impl Add for Bivector {
    type Output = Self;

    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        Self {
            yz: self.yz + other.yz,
            zx: self.zx + other.zx,
            xy: self.xy + other.xy,
        }
    }
}

impl Mul for Bivector {
    type Output = (f64, Self);

    fn mul(self, other: Self) -> Self::Output {
        (-self.xy * other.xy - self.yz * other.yz - self.zx * other.zx,
        Self {
            yz: self.xy * other.zx - self.zx * other.xy,
            zx: self.yz * other.xy - self.xy * other.yz,
            xy: self.zx * other.yz - self.yz * other.zx,
        })
    }
}

impl Mul<Bivector> for f64 {
    type Output = Bivector;

    fn mul(self, other: Bivector) -> Self::Output {
        Bivector {
            yz: self * other.yz,
            zx: self * other.zx,
            xy: self * other.xy,
        }
    }
}

impl Neg for Bivector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let b1 = Bivector { yz: 1.0, zx: 0.0, xy: 1.0 };
        let b2 = Bivector { yz: 1.0, zx: 3.0, xy: 0.0 };
        assert_eq!(b1 + b2, Bivector { yz: 2.0, zx: 3.0, xy: 1.0 });
    }

    #[test]
    fn test_scale() {
        let b = Bivector { yz: 1.0, zx: 3.0, xy: 0.0 };
        assert_eq!(2.0 * b, Bivector { yz: 2.0, zx: 6.0, xy: 0.0 });
    }

    #[test]
    fn test_mul() {
        let (dot, wedge) = XY * (XY + YZ);
        assert_eq!(dot, -1.0);
        assert_eq!(wedge, -ZX);
    }
}
