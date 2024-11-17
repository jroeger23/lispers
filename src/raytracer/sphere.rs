use super::types::{Intersect, Material, Point3, Ray, Scalar, Vector3};

extern crate nalgebra as na;

pub struct Sphere {
    center: Point3,
    radius: Scalar,
    material: Material,
}

/// Numerical error tolerance
const EPSILON: Scalar = 1e-5;

impl Intersect for Sphere {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, &'a Material)> {
        let co = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&co);
        let c = co.dot(&co) - (self.radius * self.radius);
        let d = b * b - 4.0 * a * c;

        if d >= 0.0 {
            let e = d.sqrt();
            let t1 = (-b - e) / (2.0 * a);
            let t2 = (-b + e) / (2.0 * a);
            let mut t = Scalar::MAX;

            if t1 > EPSILON && t1 < t {
                t = t1;
            }
            if t2 > EPSILON && t2 < t {
                t = t2;
            }

            if t < Scalar::MAX {
                let isect_pt: Point3 = ray.origin + ray.direction * t;
                return Some((
                    isect_pt,
                    (isect_pt - self.center) / self.radius,
                    t,
                    &self.material,
                ));
            }
        }

        None
    }
}
