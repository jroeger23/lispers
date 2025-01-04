use std::fmt::{Debug, Display};

use as_any::AsAny;

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
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, Material)>;
}

/// A point light source
#[derive(Clone, Debug, PartialEq, Copy)]
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

impl PartialOrd for Light {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
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
#[derive(Clone, Debug, PartialEq, Copy)]
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

impl PartialOrd for Material {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

//////// Display traits ////////////////////////////////////////////////////////////////////////////

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(light position: {}, color: {})",
            self.position, self.color
        )
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(material ambient_color: {}, diffuse_color: {}, specular_color: {}, shininess: {}, mirror: {})",
            self.ambient_color, self.diffuse_color, self.specular_color, self.shininess, self.mirror
        )
    }
}

// RTWrapper ///////////////////////////////////////////////////////////////////////////////////////

/// A trait used for Objects, which can be stored inside of the Scene, are Intersectable and are ForeignData compatible.
pub trait RTObject: Intersect + Display + Debug + AsAny + Sync + Send + 'static {
    /// Convert the object to a Box<dyn Any> allowing downcasts to Self
    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any>;
    /// Explicitly compare the object with another RTObject for object safety
    fn eq_impl(&self, other: &dyn RTObject) -> bool;
    /// Explicitly clone the object for object safety
    fn clone_impl(&self) -> Box<dyn RTObject>;
}

impl<T: Intersect + Display + Debug + PartialEq + Clone + Sync + Send + 'static> RTObject for T {
    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn eq_impl(&self, other: &dyn RTObject) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
    fn clone_impl(&self) -> Box<dyn RTObject> {
        Box::new(self.clone())
    }
}

impl PartialEq for dyn RTObject {
    fn eq(&self, other: &Self) -> bool {
        self.eq_impl(other)
    }
}

/// The RTObjectWrapper is a wrapper around a Box<dyn RTObject> to make it ForeignData compatible
/// (not depending on the concrete type of the object).
pub struct RTObjectWrapper(Box<dyn RTObject>);

impl RTObjectWrapper {
    /// Create a new RTObjectWrapper from a Box<dyn RTObject>
    pub fn new<T: RTObject>(value: Box<T>) -> RTObjectWrapper {
        RTObjectWrapper(value)
    }
    /// Create a new RTObjectWrapper from a RTObject
    pub fn from<T: RTObject>(value: T) -> RTObjectWrapper {
        RTObjectWrapper::new(Box::new(value))
    }
    /// Get the inner box as Box<dyn Any> allowing downcasts to the concrete type
    pub fn as_any_box(self) -> Box<dyn std::any::Any> {
        self.0.as_any_box()
    }
}

impl Clone for RTObjectWrapper {
    fn clone(&self) -> Self {
        RTObjectWrapper(self.0.clone_impl())
    }
}

impl PartialEq for RTObjectWrapper {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl Display for RTObjectWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTObjectWrapper({})", self.0)
    }
}

impl Debug for RTObjectWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTObjectWrapper({:?})", self.0)
    }
}

impl Intersect for RTObjectWrapper {
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, Material)> {
        self.0.intersect(ray)
    }
}

impl PartialOrd for RTObjectWrapper {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

#[test]
fn test_rt_wrapper_expr_conversion() {
    use super::sphere::Sphere;
    use lispers_core::lisp::expression::{Expression, ForeignDataWrapper};
    let sphere = Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        1.0,
        Material::new(
            Color::new(0.0, 0.0, 0.0),
            Color::new(0.0, 0.0, 0.0),
            Color::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
        ),
    );

    let sphere = RTObjectWrapper::new(Box::new(sphere));

    let expr: Expression = ForeignDataWrapper::new(sphere.clone()).into();

    let sphere2: ForeignDataWrapper<RTObjectWrapper> = expr.try_into().unwrap();

    assert_eq!(sphere, *sphere2.0);
}
