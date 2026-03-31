use std::{fmt::Display, path::Path};

use super::{
    scene::Scene,
    types::{Color, Point3, Ray, Scalar, Vector3},
    RTError,
};
use image::RgbImage;
use lispers_core::lisp::eval::EvalError;
use ndarray::Array3;
use rayon::prelude::*;
use video_rs::{encode::Settings, Encoder, Time};

/// A camera that can render a scene.
#[derive(Clone, PartialEq, Debug)]
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

    pub fn reposition(
        &self,
        position: Point3,
        center: Point3,
        up: Vector3,
        fovy: Scalar,
    ) -> Camera {
        Camera::new(position, center, up, fovy, self.width, self.height)
    }

    pub fn render_animation<
        SFn: Fn(u32) -> Result<Scene, EvalError>,
        CFn: Fn(u32, &Camera) -> Result<Camera, EvalError>,
    >(
        &self,
        path: &Path,
        scene_fn: SFn,
        update_cam: CFn,
        frames: u32,
        fps: u32,
        depth: u32,
        subp: u32,
    ) -> Result<(), RTError> {
        let mut encoder = Encoder::new(
            path,
            Settings::preset_h264_yuv420p(self.width, self.height, false),
        )?;
        let frame_duration = Time::from_nth_of_a_second(fps as usize);
        let mut timestamp = Time::zero();

        let mut cam = self.to_owned();
        for t in 0..frames {
            println!(
                "Rendering frame {}/{} for {}",
                t + 1,
                frames,
                path.display()
            );
            cam = update_cam(t, &cam)?;
            let img = cam.render(&scene_fn(t)?, depth, subp);

            let frame = Array3::from_shape_fn((self.height, self.width, 3), |(y, x, c)| {
                img.get_pixel(x as u32, y as u32)[c]
            });

            encoder.encode(&frame, timestamp)?;
            timestamp = timestamp.aligned_with(frame_duration).add();
        }

        encoder.finish()?;
        Ok(())
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Camera {{ position: {}, lower_left: {}, x_dir: {}, y_dir: {}, width: {}, height: {} }}",
            self.position, self.lower_left, self.x_dir, self.y_dir, self.width, self.height)
    }
}

impl PartialOrd for Camera {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
