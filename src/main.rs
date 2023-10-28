use std::ops::Add;
use std::ops::Mul;
use vector::Vector;
use bivector::Bivector;

mod vector;
mod bivector;

fn main() {
    println!("Hello, world!");
}

impl Mul<Bivector> for Vector {
    type Output = (Vector, f64);

    fn mul(self, other: Bivector) -> Self::Output {
        (
            Vector {
                x: self.z * other.zx,
                y: self.x * other.xy,
                z: self.y * other.yz,
            },
            self.x * other.yz + self.y * other.zx + self.z * other.xy,
        )
    }
}

impl Mul<Vector> for Bivector {
    type Output = (Vector, f64);

    fn mul(self, other: Vector) -> Self::Output {
        (
            Vector {
                x: self.xy * other.y,
                y: self.yz * other.z,
                z: self.zx * other.x,
            },
            self.xy * other.z + self.yz * other.x + self.zx * other.y,
        )
    }
}

#[derive(Copy, Clone)]
struct VGA (
    pub f64,
    pub Vector,
    pub Bivector,
    pub f64
);

impl Default for VGA {
    fn default() -> Self {
        VGA (
            0.0,
            Vector { ..Default::default() },
            Bivector { ..Default::default() },
            0.0,
        )
    }
}

impl Add for VGA {
    type Output = Self;

    fn add(self, other: VGA) -> <Self as Add<VGA>>::Output {
        VGA (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl Mul for VGA {
    type Output = Self;

    fn mul(self, other: VGA) -> <Self as Mul<VGA>>::Output {
        let (vdot, vwedge) = self.1 * other.1;
        let (bdot, bwedge) = self.2 * other.2;
        let (vb, t1) = self.1 * other.2;
        let (bv, t2) = self.2 * other.1;
        VGA (
            self.0 * other.0 + vdot + bdot + self.3 * other.3,
            self.0 * other.1 + other.0 * self.1 + vb + bv,
            self.0 * other.2 + other.0 * self.2 + vwedge + bwedge,
            t1 + t2,
        )
    }
}