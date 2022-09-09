use std::ops::Add;

/// A structure to represent 3D vectors
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    /// Calculate the length using Pythagoras' theorem
    fn length(&self) -> f32 {
        let sum_of_squares = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);

        sum_of_squares.sqrt()
    }
}

// When we want to share behaviours across multiple types, we use "traits". These
// work similarly to "interfaces" in other languages. There are some special traits
// which control the behaviour of operators, like the + operator. If we want to
// support such an operator on our own types, we need only implement the requisite
// trait.
// In this case, we implement the "Add" trait.
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
    let vec1 = Vec3 {
        x: 1.0,
        y: 4.0,
        z: 2.0,
    };
    let vec2 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 7.0,
    };

    // Now we can use the `+` operator on our vectors.
    let length = (vec1 + vec2).length();

    println!("Length: {length}");
}
