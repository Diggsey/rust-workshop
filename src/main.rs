mod geom;
mod vec;

use geom::{Ray, Sphere};
use vec::Vec3;

fn main() {
    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, -2.0),
        direction: Vec3::new(0.0, 0.0, 1.0),
    };
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 0.5,
    };

    let intersects = ray.intersects_sphere(&sphere);

    println!("Intersects: {intersects}");
}
