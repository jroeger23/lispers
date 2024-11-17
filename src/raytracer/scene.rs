use super::types::Color;
use super::types::Intersect;
use super::types::Light;
use super::types::Material;
use super::types::Point3;
use super::types::Ray;
use super::types::Vector3;
use super::vec::mirror;
use super::vec::reflect;
extern crate nalgebra as na;

/// A scene is a collection of objects and lights, and provides a method to trace a ray through the scene.
pub struct Scene {
    /// The ambient light of the scene
    ambient: Color,
    /// The objects in the scene
    objects: Vec<Box<dyn Intersect>>,
    /// The lights in the scene
    lights: Vec<Light>,
}

impl Scene {
    /// Create a new empty scene
    pub fn new() -> Scene {
        Scene {
            ambient: na::Vector3::new(0.0, 0.0, 0.0),
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    /// Set the ambient light of the scene
    pub fn set_ambient(&mut self, ambient: Color) {
        self.ambient = ambient;
    }

    /// Add an object to the scene
    pub fn add_object(&mut self, obj: Box<dyn Intersect>) {
        self.objects.push(obj);
    }

    /// Add a light to the scene
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Trace a ray through the scene and return the color of the ray.
    /// - `ray` is the ray to be traced
    /// - `depth` is the maximum recursion depth aka the number of reflections
    pub fn trace(&self, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return na::Vector3::new(0.0, 0.0, 0.0);
        }

        match self
            .objects
            .iter()
            .filter_map(|obj| obj.intersect(ray))
            .min_by(|(_, _, t1, _), (_, _, t2, _)| t1.partial_cmp(t2).unwrap())
        {
            Some((isect_pt, isect_norm, _, material)) => {
                // Lighting of material at the intersection point
                let color = self.lighting(-&ray.direction, material, isect_pt, isect_norm);

                // Calculate reflections, if the material has mirror properties
                if material.mirror > 0.0 {
                    let new_ray = Ray {
                        origin: isect_pt,
                        direction: reflect(ray.direction, isect_norm),
                    };
                    return (1.0 - material.mirror) * color
                        + material.mirror * self.trace(&new_ray, depth - 1);
                } else {
                    return color;
                }
            }
            _ => {
                return na::Vector3::new(0.0, 0.0, 0.0);
            }
        }
    }

    /// Calculate Phong lighting from a `view` on a `material` at an intersection point `isect_pt` with a normal `isect_norm`.
    fn lighting(
        &self,
        view: Vector3,
        material: &Material,
        isect_pt: Point3,
        isect_norm: Vector3,
    ) -> Color {
        // Start with ambient lighting
        let mut color = material.ambient_color.component_mul(&self.ambient);

        for light in &self.lights {
            // Cast Shadow-Ray
            let shadow_ray = Ray {
                origin: isect_pt,
                direction: (light.position - isect_pt).normalize(),
            };
            if self
                .objects
                .iter()
                .any(|obj| obj.intersect(&shadow_ray).is_some())
            {
                continue;
            }

            // Diffuse
            let l = (light.position - isect_pt).normalize();
            let cos_theta = l.dot(&isect_norm);
            if cos_theta > 0.0 {
                color += material.diffuse_color.component_mul(&light.color) * cos_theta;

                // Specular
                let r = mirror(l, isect_norm);
                let cos_alpha = r.dot(&view);
                if cos_alpha > 0.0 {
                    color += material.specular_color.component_mul(&light.color)
                        * cos_alpha.powf(material.shininess);
                }
            }
        }

        color
    }
}
