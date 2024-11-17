extern crate nalgebra as na;

/// The Scalar type to use for raytracing (f32 may result in acne effects)
pub type Scalar = f64;
/// The Vector3 type to use for raytracing
pub type Vector3 = na::Vector3<Scalar>;
/// The Point3 type to use for raytracing
pub type Point3 = na::Point3<Scalar>;
/// The Color type to use for raytracing
pub type Color = Vector3;

/// A trait indicating, that an object can be intersected by a ray
pub trait Intersect {
    /// Intersect the object with a ray.
    /// Returns None if the ray does not intersect the object.
    /// Otherwise the intersection point, a normal vector at the intersection point,
    /// the distance from the ray origin to the intersection point and
    /// the material of the object are returned.
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, &'a Material)>;
}

/// A point light source
pub struct Light {
    /// Position of the light source
    pub position: Point3,
    /// Light color
    pub color: Color,
}

impl Light {
    /// Create a new light source at position with color
    pub fn new(position: Point3, color: Color) -> Light {
        Light { position, color }
    }
}

/// A ray with origin and direction
pub struct Ray {
    /// Ray origin
    pub origin: Point3,
    /// Ray direction
    pub direction: Vector3,
}

impl Ray {
    /// Create a new ray with origin and direction
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
}

/// A Material used for PHONG shading
pub struct Material {
    /// Ambient color, aka color without direct or indirect light
    pub ambient_color: Color,
    /// Diffuse color, aka color with direct light and reflected light
    pub diffuse_color: Color,
    /// Specular color, aka color of the highlights from direct light sources
    pub specular_color: Color,
    /// A shininess factor, used to calculate the size of the highlights. `pow(angle, shininess) * specular_color = intensity`
    pub shininess: Scalar,
    /// A mirror factor, used to calculate the reflection of the object. `self_color * reflected_color = final_color`
    pub mirror: Scalar,
}

impl Material {
    /// Create a new material with ambient, diffuse, specular color, shininess and mirror factor.
    /// - `ambient_color` is the color of the object without direct or indirect light
    /// - `diffuse_color` is the color of the object with direct light and reflected light
    /// - `specular_color` is the color of the highlights from direct light sources
    /// - `shininess` is a factor used to calculate the size of the highlights. `pow(angle, shininess) * specular_color = intensity`
    /// - `mirror` is a factor used to calculate the reflection of the object. `self_color * reflected_color = final_color`
    pub fn new(
        ambient_color: Color,
        diffuse_color: Color,
        specular_color: Color,
        shininess: Scalar,
        mirror: Scalar,
    ) -> Material {
        Material {
            ambient_color,
            diffuse_color,
            specular_color,
            shininess,
            mirror,
        }
    }
}
