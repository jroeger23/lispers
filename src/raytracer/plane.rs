use super::types::{Intersect, Material, Point3, Scalar, Vector3};

extern crate nalgebra as na;

/// An infinite plane in 3D space.
#[derive(PartialEq, Clone, Debug)]
pub struct Plane {
    /// The position of the plane.
    position: Point3,
    /// The normal of the plane.
    normal: Vector3,
    /// The material of the plane.
    material: Material,
}

/// A infinite checkerboard plane in 3D space.
#[derive(PartialEq, Clone, Debug)]
pub struct Checkerboard {
    /// The base plane containing the "white" material
    base: Plane,
    /// An alternative "black" material
    material_alt: Material,
    /// The scale of the checkerboard (side-length of each square)
    scale: f64,
    /// A projection matrix to map 3D points to the 2D plane space.
    projection_matrix: na::Matrix2x3<Scalar>,
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

impl Checkerboard {
    /// Create a new Checkerboard Plane.
    /// - `position` is the position of the plane.
    /// - `normal` is the normal of the plane.
    /// - `material1` is the material of the "white" squares.
    /// - `material2` is the material of the "black" squares.
    /// - `scale` is the side-length of each square.
    /// - `up` is "y" direction on the plane in 3D-Space.
    pub fn new(
        position: Point3,
        normal: Vector3,
        material1: Material,
        material2: Material,
        scale: f64,
        up: Vector3,
    ) -> Checkerboard {
        let right = up.cross(&normal).normalize();
        Checkerboard {
            base: Plane::new(position, normal, material1),
            material_alt: material2,
            scale,
            projection_matrix: na::Matrix3x2::from_columns(&[right, up]).transpose(),
        }
    }
}

impl Intersect for Plane {
    fn intersect(
        &self,
        ray: &super::types::Ray,
    ) -> Option<(
        Point3,
        Vector3,
        super::types::Scalar,
        super::types::Material,
    )> {
        let denom = self.normal.dot(&ray.direction);
        if denom != 0.0 {
            let d = self.normal.dot(&self.position.coords);
            let t = (d - self.normal.dot(&ray.origin.coords)) / denom;

            if t > 1e-5 {
                let point = ray.origin + ray.direction * t;
                return Some((point, self.normal, t, self.material.clone()));
            }
        }
        None
    }
}

impl Intersect for Checkerboard {
    fn intersect(
        &self,
        ray: &super::types::Ray,
    ) -> Option<(
        Point3,
        Vector3,
        super::types::Scalar,
        super::types::Material,
    )> {
        if let Some((point, normal, t, material)) = self.base.intersect(ray) {
            let v3 = point - self.base.position;
            let v2 = self.projection_matrix * v3;

            if ((v2.x / self.scale).round() % 2.0 == 0.0)
                == ((v2.y / self.scale).round() % 2.0 == 0.0)
            {
                Some((point, normal, t, material.clone()))
            } else {
                Some((point, normal, t, self.material_alt.clone()))
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(plane position: {}, normal: {}, material: {})",
            self.position, self.normal, self.material
        )
    }
}

impl std::fmt::Display for Checkerboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(checkerboard position: {}, normal: {}, material1: {}, material2: {}, scale: {})",
            self.base.position, self.base.normal, self.base.material, self.material_alt, self.scale
        )
    }
}

impl PartialOrd for Plane {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl PartialOrd for Checkerboard {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
