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

impl Ray {
    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        // Compute a vector from the beginning of the ray to the center of the sphere
        let offset = sphere.center - self.origin;

        // Project that vector onto the ray direction, to get the distance along the ray
        // to the point where the ray is closest to the sphere's center.
        let distance_along_ray = self.direction.dot(&offset);

        // Don't consider intersections "behind" the ray.
        if distance_along_ray < 0.0 {
            return false;
        }

        // Find the coordinates of that closest point
        let closest_point = self.origin + distance_along_ray * self.direction;

        // Find the distance from that closest point to the center of the sphere
        let ray_sphere_distance = (sphere.center - closest_point).length();

        // Check if that distance is less than the sphere's radius
        ray_sphere_distance <= sphere.radius
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
