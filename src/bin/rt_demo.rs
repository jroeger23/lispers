use lispers::raytracer::{
    camera::Camera,
    plane::Plane,
    scene::Scene,
    sphere::Sphere,
    types::{Color, Light, Material, Point3, Vector3},
};
extern crate nalgebra as na;
use std::sync::Arc;

fn main() {
    let mut scene = Scene::new();

    scene.set_ambient(Color::new(0.2, 0.2, 0.2));

    scene.add_light(Light {
        position: Point3::new(4.0, 7.0, 10.0),
        color: Color::new(1.0, 1.0, 1.0),
    });
    scene.add_light(Light {
        position: Point3::new(-2.0, 7.0, 10.0),
        color: Color::new(1.0, 1.0, 1.0),
    });

    scene.add_object(Arc::new(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Material::new(
            Color::new(0.5, 0.5, 0.5),
            Color::new(0.5, 0.5, 0.5),
            Color::new(0.0, 0.0, 0.0),
            0.0,
            0.6,
        ),
    )));

    scene.add_object(Arc::new(Sphere::new(
        Point3::new(-2.0, 0.0, 1.0),
        1.0,
        Material::new(
            Color::new(0.0, 1.0, 0.0),
            Color::new(0.0, 1.0, 0.0),
            Color::new(0.6, 0.6, 0.6),
            20.0,
            0.3,
        ),
    )));

    scene.add_object(Arc::new(Sphere::new(
        Point3::new(0.2, -0.5, -0.2),
        0.5,
        Material::new(
            Color::new(0.0, 0.0, 1.0),
            Color::new(0.0, 0.0, 1.0),
            Color::new(0.6, 0.6, 0.6),
            20.0,
            0.3,
        ),
    )));

    scene.add_object(Arc::new(Sphere::new(
        Point3::new(-0.5, 0.5, -2.0),
        1.5,
        Material::new(
            Color::new(1.0, 0.0, 0.0),
            Color::new(1.0, 0.0, 0.0),
            Color::new(0.6, 0.6, 0.6),
            20.0,
            0.3,
        ),
    )));

    let camera = Camera::new(
        Point3::new(0.0, 0.7, 5.0),
        Point3::new(-1.0, -0.5, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
        400,
        300,
    );

    let fname = "demo-scene.png";
    match camera.render(&scene, 5, 2).save(fname) {
        Ok(_) => println!("Image saved to {}", fname),
        Err(e) => println!("Error saving image: {}", e),
    }
}
