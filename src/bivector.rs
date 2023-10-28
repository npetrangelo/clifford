use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bivector {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

pub const I: Bivector = Bivector { i: 1.0, j: 0.0, k: 0.0 };
pub const J: Bivector = Bivector { i: 0.0, j: 1.0, k: 0.0 };
pub const K: Bivector = Bivector { i: 0.0, j: 0.0, k: 1.0 };

impl Default for Bivector {
    fn default() -> Self {
        Self { i: 0.0, j: 0.0, k: 0.0, }
    }
}

impl Add for Bivector {
    type Output = Self;

    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl Mul for Bivector {
    type Output = (f64, Self);

    fn mul(self, other: Self) -> Self::Output {
        (-self.i * other.i - self.j * other.j - self.k * other.k,
        Self {
            i: self.j * other.k - self.k * other.j,
            j: self.k * other.i - self.i * other.k,
            k: self.i * other.j - self.j * other.i,
        })
    }
}

impl Mul<Bivector> for f64 {
    type Output = Bivector;

    fn mul(self, other: Bivector) -> Self::Output {
        Bivector {
            i: self * other.i,
            j: self * other.j,
            k: self * other.k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let b1 = Bivector { i: 1.0, j: 1.0, k: 0.0 };
        let b2 = Bivector { i: 0.0, j: 1.0, k: 3.0 };
        assert_eq!(b1 + b2, Bivector { i: 1.0, j: 2.0, k: 3.0 });
    }

    #[test]
    fn test_scale() {
        let b = Bivector { i: 0.0, j: 1.0, k: 3.0 };
        assert_eq!(2.0 * b, Bivector { i: 0.0, j: 2.0, k: 6.0 });
    }

    #[test]
    fn test_mul() {
        let (dot, wedge) = I * (I + J);
        assert_eq!(dot, -1.0);
        assert_eq!(wedge, K);
    }
}
