extern crate nalgebra as na;

pub type Scalar = f32;
pub type Vector3 = na::Vector3<Scalar>;
pub type Point3 = na::Point3<Scalar>;
pub type Color = Vector3;

pub trait Intersect {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<(Point3, Vector3, Scalar, &'a Material)>;
}

pub struct Light {
    pub position: Point3,
    pub color: Color,
}

impl Light {
    pub fn new(position: Point3, color: Color) -> Light {
        Light { position, color }
    }
}

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
}

pub struct Material {
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub shinyness: Scalar,
    pub mirror: Scalar,
}

impl Material {
    pub fn new(
        ambient_color: Color,
        diffuse_color: Color,
        specular_color: Color,
        shinyness: Scalar,
        mirror: Scalar,
    ) -> Material {
        Material {
            ambient_color,
            diffuse_color,
            specular_color,
            shinyness,
            mirror,
        }
    }
}
