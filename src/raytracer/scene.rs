use super::types::Color;
use super::types::Intersect;
use super::types::Light;
use super::types::Material;
use super::types::Point3;
use super::types::Ray;
use super::types::Scalar;
use super::types::Vector3;
use super::vec::reflect;
extern crate nalgebra as na;

pub struct Scene {
    objects: Vec<Box<dyn Intersect>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Intersect>) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

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
            None => {
                return na::Vector3::new(0.0, 0.0, 0.0);
            }
            Some((isect_pt, isect_norm, isect_dist, material)) => {
                // Lighting of material at the intersection point
                let color =
                    self.lighting(-&ray.direction, material, isect_pt, isect_norm, isect_dist);

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
        }
    }

    fn lighting(
        &self,
        view: Vector3,
        material: &Material,
        isect_pt: Point3,
        isect_norm: Vector3,
        isect_dist: Scalar,
    ) -> Color {
        let mut color: Color = na::Vector3::new(0.0, 0.0, 0.0);

        for light in &self.lights {
            let l = (isect_pt - light.position).normalize();
            let cos_theta = l.dot(&isect_norm);

            if cos_theta > 0.0 {
                // Diffuse
                color += material.diffuse_color.component_mul(&light.color) * cos_theta;
            }
        }

        color
    }
}
