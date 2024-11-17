use super::{
    scene::Scene,
    types::{Point3, Ray, Scalar, Vector3},
};
// use image::Rgb32FImage;

pub struct Camera {
    position: Point3,
    up: Vector3,
    right: Vector3,
    upper_left: Point3,
    x_dir: Vector3,
    y_dir: Vector3,
    width: usize,
    height: usize,
}

impl Camera {
    pub fn new(
        position: Point3,
        direction: Vector3,
        up: Vector3,
        fovy: Scalar,
        width: usize,
        height: usize,
    ) -> Camera {
        let aspect_ratio = width as Scalar / height as Scalar;
        let fovx = fovy * aspect_ratio;
        let right = direction.cross(&up).normalize();
        let x_dir = right * (fovx / 2.0).tan();
        let y_dir = -up * (fovy / 2.0).tan();
        let upper_left = position + direction - x_dir + y_dir;

        Camera {
            position,
            up,
            right,
            upper_left,
            x_dir,
            y_dir,
            width,
            height,
        }
    }
}

impl Camera {
    pub fn ray_at_relative(&self, x: Scalar, y: Scalar) -> Ray {
        let x_dir = self.x_dir * x;
        let y_dir = self.y_dir * y;
        Ray::new(
            self.position,
            (self.upper_left + x_dir - y_dir - self.position).normalize(),
        )
    }

    pub fn ray_at(&self, x: usize, y: usize) -> Ray {
        let x = x as Scalar / self.width as Scalar;
        let y = y as Scalar / self.height as Scalar;
        self.ray_at_relative(x, y)
    }

    // pub fn render(&self, scene: &Scene, depth: u32) -> Rgb32FImage {
    //     Rgb32FImage::from_fn(self.width, self.height, |x, y| {
    //         let ray = self.ray_at(x, y);
    //         let color = scene.trace(&ray, depth);
    //         color.into()
    //     })
    // }
}
