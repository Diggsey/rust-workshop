mod vec;

use vec::Vec3;

fn main() {
    let vec1 = Vec3::new(1.0, 4.0, 2.0);
    let vec2 = Vec3::new(1.0, 2.0, 7.0);

    let length = (vec1 + vec2).length();

    println!("Length: {length}");
}
