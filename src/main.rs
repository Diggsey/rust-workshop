use std::ops::Add;

/// A structure to represent 3D vectors
#[derive(PartialEq, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    /// Construct a new Vec3 given three components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculate the length using Pythagoras' theorem
    fn length(&self) -> f32 {
        let sum_of_squares = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);

        sum_of_squares.sqrt()
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

fn main() {
    let vec1 = Vec3::new(1.0, 4.0, 2.0);
    let vec2 = Vec3::new(1.0, 2.0, 7.0);

    let length = (vec1 + vec2).length();

    println!("Length: {length}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_lengths() {
        assert_eq!(Vec3::new(1.0, 2.0, 2.0).length(), 3.0);
        assert_eq!(Vec3::new(2.0, 10.0, 11.0).length(), 15.0);
    }

    #[test]
    fn an_addition() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 4.0) + Vec3::new(5.0, 3.0, 7.0),
            Vec3::new(6.0, 5.0, 11.0)
        );
    }
}
