use std::ops::Add;
use std::ops::Mul;
use vector::Vector;
use bivector::Bivector;
use trivector::Trivector;

mod vector;
mod bivector;
mod trivector;

fn main() {
    println!("Hello, world!");
}

impl Mul<Bivector> for Vector {
    type Output = (Vector, Trivector);

    fn mul(self, other: Bivector) -> Self::Output {
        (
            Vector {
                x: self.z * other.zx - self.x * other.zx,
                y: self.x * other.xy - self.y * other.xy,
                z: self.y * other.yz - self.z * other.yz,
            },
            Trivector(self.x * other.yz + self.y * other.zx + self.z * other.xy),
        )
    }
}

impl Mul<Vector> for Bivector {
    type Output = (Vector, Trivector);

    fn mul(self, other: Vector) -> Self::Output {
        (
            Vector {
                x: self.xy * other.y - self.xy * other.x,
                y: self.yz * other.z - self.yz * other.y,
                z: self.zx * other.x - self.zx * other.z,
            },
            Trivector(self.xy * other.z + self.yz * other.x + self.zx * other.y),
        )
    }
}

#[derive(Copy, Clone, Default)]
struct VGA (
    pub f64,
    pub Vector,
    pub Bivector,
    pub Trivector
);

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
            self.0 * other.1 + other.0 * self.1 + vb + bv + self.2 * self.3 + self.3 * self.2,
            self.0 * other.2 + other.0 * self.2 + vwedge + bwedge + self.1 * other.3 + self.3 * other.1,
            t1 + t2 + self.0 * other.3 + other.0 * self.3,
        )
    }
}