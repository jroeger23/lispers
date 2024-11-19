use super::{
    scene::Scene,
    types::{Color, Point3, Ray, Scalar, Vector3},
};
use image::RgbImage;
use rayon::prelude::*;

/// A camera that can render a scene.
pub struct Camera {
    /// Position of the camera's eye.
    position: Point3,
    /// The lower left point of the image plane.
    lower_left: Point3,
    /// The direction of the x-axis on the image plane. (length is equal to the image width)
    x_dir: Vector3,
    /// The direction of the y-axis on the image plane. (length is equal to the image height)
    y_dir: Vector3,
    /// The width of the image. [px]
    width: usize,
    /// The height of the image. [px]
    height: usize,
}

impl Camera {
    /// Create a new camera at `position` looking at `center` with `up` as the up vector.
    /// The camera has a field of view of `fovy` degrees and an image size of `width` x `height`.
    pub fn new(
        position: Point3,
        center: Point3,
        up: Vector3,
        fovy: Scalar,
        width: usize,
        height: usize,
    ) -> Camera {
        let view = (center - position).normalize();
        let dist = (center - position).norm();
        let aspect = width as Scalar / height as Scalar;

        let im_height = 2.0 * dist * (fovy.to_radians() / 2.0).tan();
        let im_width = aspect * im_height;

        let x_dir = view.cross(&up).normalize() * im_width;
        let y_dir = x_dir.cross(&view).normalize() * im_height;

        let lower_left = center - 0.5 * x_dir - 0.5 * y_dir;

        Camera {
            position,
            lower_left,
            x_dir,
            y_dir,
            width,
            height,
        }
    }

    /// Get a ray pointing from the camera to a relative position on the image plane.
    /// `x` and `y` are expected to be in the range `[0, 1]`.
    pub fn ray_at_relative(&self, x: Scalar, y: Scalar) -> Ray {
        let x_dir = self.x_dir * x;
        let y_dir = self.y_dir * y;
        Ray::new(
            self.position,
            (self.lower_left + x_dir + y_dir - self.position).normalize(),
        )
    }

    /// Get a ray pointing from the camera to a pixel on the image plane.
    /// `x` and `y` are expected to be in the range `[0, width-1]` and `[0, height-1]` respectively.
    pub fn ray_at(&self, x: usize, y: usize) -> Ray {
        let x = x as Scalar / self.width as Scalar;
        let y = y as Scalar / self.height as Scalar;
        self.ray_at_relative(x, 1.0 - y)
    }

    /// Render the scene from the camera's perspective.
    /// - `depth` is the maximum number of reflections to calculate.
    /// - `subp` is the number of subpixels to use for antialiasing.
    pub fn render(&self, scene: &Scene, depth: u32, subp: u32) -> RgbImage {
        let dx = 1.0 / self.width as Scalar;
        let dy = 1.0 / self.height as Scalar;
        let dsx = dx / subp as Scalar;
        let dsy = dy / subp as Scalar;
        let mut img = RgbImage::new(self.width as u32, self.height as u32);

        img.enumerate_rows_mut().par_bridge().for_each(|(_, row)| {
            for (x, y, pixel) in row {
                let y = y as Scalar * dy;
                let x = x as Scalar * dx;
                let mut color = Color::new(0.0, 0.0, 0.0);
                for sx in 0..subp {
                    for sy in 0..subp {
                        color += scene.trace(
                            &self.ray_at_relative(
                                x + sx as Scalar * dsx,
                                1.0 - (y + sy as Scalar * dsy),
                            ),
                            depth,
                        );
                    }
                }
                color *= 255.0 / (subp * subp) as Scalar;
                *pixel = [color.x as u8, color.y as u8, color.z as u8].into();
            }
        });
        img
    }
}
