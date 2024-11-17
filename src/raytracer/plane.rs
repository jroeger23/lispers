use super::types::{Intersect, Material, Point3, Vector3};

extern crate nalgebra as na;

/// An infinite plane in 3D space.
pub struct Plane {
    /// The position of the plane.
    position: Point3,
    /// The normal of the plane.
    normal: Vector3,
    /// The material of the plane.
    material: Material,
}

impl Plane {
    /// Create a new plane.
    /// - `position` is the position of the plane.
    /// - `normal` is the normal of the plane.
    /// - `material` is the material of the plane.
    pub fn new(position: Point3, normal: Vector3, material: Material) -> Plane {
        Plane {
            position,
            normal,
            material,
        }
    }
}

impl Intersect for Plane {
    fn intersect<'a>(
        &'a self,
        ray: &super::types::Ray,
    ) -> Option<(
        Point3,
        Vector3,
        super::types::Scalar,
        &'a super::types::Material,
    )> {
        let denom = self.normal.dot(&ray.direction);
        if denom != 0.0 {
            let d = self.normal.dot(&self.position.coords);
            let t = (d - self.normal.dot(&ray.origin.coords)) / denom;

            if t > 1e-5 {
                let point = ray.origin + ray.direction * t;
                return Some((point, self.normal, t, &self.material));
            }
        }
        None
    }
}
