// Comments written using "triple slash" are documentation comments. Documentation
// for a crate can be generated automatically using `cargo doc`:
/// A structure to represent 3D vectors
struct Vec3 {
    // We define three floating-point fields for X/Y/Z components of the vector.
    // `f32` means a 32-bit float, also known as a "single" in some languages.
    x: f32,
    y: f32,
    z: f32,
}

// Implement an inherent method for this structure.
impl Vec3 {
    /// Calculate the length using Pythagoras' theorem
    // The `self` parameter is written explicitly, even for inherent methods. There
    // are several forms which the `self` parameter can take: this form means that
    // the method does not modify the structure.
    fn length(&self) -> f32 {
        // Fields are accessed via the `self` parameter. Simply writing `x` would not
        // work.
        let sum_of_squares = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);

        // The last expression in a function is returned implicitly.
        sum_of_squares.sqrt()
    }
}

// This is the entry point for our program
fn main() {
    // Construct an instance of our vector type
    let vec = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 6.0,
    };
    let length = vec.length();

    // Print out the length of the vector
    println!("Length: {length}");
}
