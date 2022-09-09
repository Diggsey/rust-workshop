use std::ops::{Add, Mul, Sub};

/// A structure to represent 3D vectors
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Construct a new Vec3 given three components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculate the length using Pythagoras' theorem
    pub fn length(&self) -> f32 {
        let sum_of_squares = self.dot(self);

        sum_of_squares.sqrt()
    }
    /// Calculate the dot product of two vectors
    /// This gives an indication of how "aligned" the two vectors are.
    /// A positive value means the vectors point in a similar direction.
    /// A negative value means the vectors point in opposite directions.
    /// A zero value means the vectors are perpendicular.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add for Vec3 {
    /// The result of adding two 3D vectors is another 3D vector.
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        // Vectors are added component-wise
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    /// The result of subtracting two 3D vectors is another 3D vector.
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        // Vectors are added component-wise
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_lengths() {
        assert_eq!(Vec3::new(1.0, 2.0, 2.0).length(), 3.0);
        assert_eq!(Vec3::new(2.0, 10.0, 11.0).length(), 15.0);
    }

    // Create some constants so that we can reuse the values
    // across our addition and subtraction tests.
    const A: Vec3 = Vec3::new(1.0, 2.0, 4.0);
    const B: Vec3 = Vec3::new(5.0, 3.0, 7.0);
    const C: Vec3 = Vec3::new(6.0, 5.0, 11.0);

    #[test]
    fn an_addition() {
        assert_eq!(A + B, C);
    }

    #[test]
    fn a_subtraction() {
        assert_eq!(C - B, A);
    }

    #[test]
    fn a_multiplication() {
        assert_eq!(2.0 * Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    }
}