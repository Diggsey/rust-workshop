use serde::{Deserialize, Serialize};

use crate::Vec3;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3,
    /// Direction should always be a unit vector (have length 1)
    pub direction: Vec3,
}
#[derive(Debug)]
pub struct Intersection {
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

impl Ray {
    pub fn intersect_sphere(&self, sphere: &Sphere) -> Option<Intersection> {
        // Compute a vector from the beginning of the ray to the center of the sphere
        let offset = sphere.center - self.origin;

        // Project that vector onto the ray direction, to get the distance along the ray
        // to the point where the ray is closest to the sphere's center.
        let distance_along_ray = self.direction.dot(&offset);

        // Don't consider intersections "behind" the ray.
        if distance_along_ray < 0.0 {
            return None;
        }

        // Find the coordinates of that closest point
        let closest_point = self.origin + distance_along_ray * self.direction;

        // Find the distance from that closest point to the center of the sphere
        let ray_sphere_distance = (sphere.center - closest_point).length();

        // Check if that distance is less than the sphere's radius
        if ray_sphere_distance <= sphere.radius {
            // Use pythagoras' theorem to find out how far the closest point is into the sphere
            let distance_into_sphere = (sphere.radius.powi(2) - ray_sphere_distance.powi(2)).sqrt();
            // Subtract that distance from our original distance calculation to find where the ray
            // first entered the sphere.
            let distance = distance_along_ray - distance_into_sphere;

            // Now we have that distance, we can calculate the position of intersection
            let position = self.origin + distance * self.direction;

            // And with the position, we can subtract the sphere's center and normalize
            let normal = (1.0 / sphere.radius) * (position - sphere.center);
            Some(Intersection {
                distance,
                position,
                normal,
            })
        } else {
            None
        }
    }

    // By deferring to our new function, we can reuse our tests
    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        self.intersect_sphere(sphere).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_intersection() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -2.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.5,
        };
        assert!(ray.intersects_sphere(&sphere));
    }

    #[test]
    fn offset_intersection() {
        let ray = Ray {
            origin: Vec3::new(10.0, 5.0, -12.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {
            center: Vec3::new(10.0, 5.0, 20.0),
            radius: 0.5,
        };
        assert!(ray.intersects_sphere(&sphere));
    }

    #[test]
    fn offset_miss() {
        let ray = Ray {
            origin: Vec3::new(11.0, 5.0, -12.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {
            center: Vec3::new(10.0, 5.0, 20.0),
            radius: 0.5,
        };
        assert!(!ray.intersects_sphere(&sphere));
    }

    #[test]
    fn miss_behind() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 1.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.5,
        };
        assert!(!ray.intersects_sphere(&sphere));
    }

    #[test]
    fn diagonal_intersection() {
        let ray = Ray {
            origin: Vec3::new(1.0, 2.0, -2.0),
            direction: Vec3::new(3.0 / 5.0, 4.0 / 5.0, 0.0),
        };
        let sphere = Sphere {
            center: Vec3::new(4.0, 6.7, -1.8),
            radius: 0.5,
        };
        assert!(ray.intersects_sphere(&sphere));
    }

    #[test]
    fn diagonal_miss() {
        let ray = Ray {
            origin: Vec3::new(1.0, 2.0, -2.0),
            direction: Vec3::new(3.0 / 5.0, 4.0 / 5.0, 0.0),
        };
        let sphere = Sphere {
            center: Vec3::new(4.0, 6.7, -1.6),
            radius: 0.5,
        };
        assert!(!ray.intersects_sphere(&sphere));
    }
}
