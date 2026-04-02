use super::{
    texture::TextureWrapper,
    types::{Intersect, Material, Point2, Point3, Ray, Scalar, Vector3},
};

extern crate nalgebra as na;

/// A sphere in 3D space
#[derive(PartialEq, Clone, Debug)]
pub struct Sphere {
    /// Center of the sphere
    center: Point3,
    /// Radius of the sphere
    radius: Scalar,
    /// PHONG material of the sphere
    material: Material,
}

/// A sphere in 3D space
#[derive(PartialEq, Clone, Debug)]
pub struct TextureSphere {
    /// Center of the sphere
    center: Point3,
    /// Radius of the sphere
    radius: Scalar,
    /// texture of the sphere
    texture: TextureWrapper,
}

impl Sphere {
    /// Create a new sphere at `center` with `radius` and `material`.
    pub fn new(center: Point3, radius: Scalar, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

/// Numerical error tolerance
const EPSILON: Scalar = 1e-5;

fn intersect(ray: &Ray, center: &Point3, radius: Scalar) -> Option<(Point3, Vector3, Scalar)> {
    let co = ray.origin - center;

    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&co);
    let c = co.dot(&co) - (radius * radius);
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

            if c >= 0.0 {
                return Some((isect_pt, (isect_pt - center) / radius, t));
            } else {
                return Some((isect_pt, -(isect_pt - center) / radius, t));
            }
        }
    }

    None
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, Material)> {
        match intersect(ray, &self.center, self.radius) {
            Some((isect_pt, normal, t)) => Some((isect_pt, normal, t, self.material.clone())),
            None => None,
        }
    }
}

impl std::fmt::Display for Sphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(sphere center: {}, radius: {}, material: {})",
            self.center, self.radius, self.material
        )
    }
}

impl PartialOrd for Sphere {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl TextureSphere {
    /// Create a new sphere at `center` with `radius` and `texture`.
    pub fn new(center: Point3, radius: Scalar, texture: TextureWrapper) -> TextureSphere {
        TextureSphere {
            center,
            radius,
            texture,
        }
    }
}

impl Intersect for TextureSphere {
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, Material)> {
        match intersect(ray, &self.center, self.radius) {
            Some((isect_pt, normal, t)) => {
                let n_isect_pt = (isect_pt - self.center) / self.radius;
                let uv: Point2 = Point2::new(
                    0.5 + (n_isect_pt.z.atan2(n_isect_pt.x) / (2.0 * std::f64::consts::PI)),
                    0.5 - (n_isect_pt.y).asin() / std::f64::consts::PI,
                );
                Some((isect_pt, normal, t, self.texture.material_at(uv)))
            }
            None => None,
        }
    }
}

impl std::fmt::Display for TextureSphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(sphere center: {}, radius: {}, texture: {})",
            self.center, self.radius, self.texture
        )
    }
}

impl PartialOrd for TextureSphere {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
