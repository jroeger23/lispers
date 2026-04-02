use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;

use as_any::AsAny;
use nalgebra as na;

use super::types::Color;
use super::types::Material;
use super::types::Point2;
use super::types::Scalar;

pub trait Texture: Display + Debug + AsAny + Sync + Send {
    fn material_at(&self, pt: Point2) -> Material;
}

#[derive(Clone)]
pub struct TextureWrapper(Arc<dyn Texture>);

impl TextureWrapper {
    pub fn new<T: Texture>(texture: T) -> Self {
        Self(Arc::new(texture))
    }
}

impl Display for TextureWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for TextureWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl TextureWrapper {
    pub fn material_at(&self, pt: Point2) -> Material {
        self.0.material_at(pt)
    }
}

impl PartialEq for TextureWrapper {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for TextureWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&Arc::as_ptr(&self.0).addr(), &Arc::as_ptr(&other.0).addr())
    }
}

pub struct MandelbrotTexture {
    scale: Scalar,
    at: Point2,
    max_iter: u32,
    ambient_color: Color,
    diffuse_color: Color,
    specular_color: Color,
}

impl MandelbrotTexture {
    pub fn new(
        scale: Scalar,
        at: Point2,
        max_iter: u32,
        ambient_color: Color,
        diffuse_color: Color,
        specular_color: Color,
    ) -> Self {
        Self {
            scale,
            at,
            max_iter,
            ambient_color,
            diffuse_color,
            specular_color,
        }
    }
}

impl Texture for MandelbrotTexture {
    fn material_at(&self, pt: Point2) -> Material {
        let x = (pt.x / self.scale) + self.at.x;
        let y = (pt.y / self.scale) + self.at.y;
        let mut z = na::Vector2::new(0.0, 0.0);
        let mut n = 0;
        while z.norm() < 2.0 && n < self.max_iter {
            let xtemp = z.x * z.x - z.y * z.y + x;
            z.y = 2.0 * z.x * z.y + y;
            z.x = xtemp;
            n += 1;
        }
        let c = n as f64 / self.max_iter as f64;

        Material {
            ambient_color: self.ambient_color * c,
            diffuse_color: self.diffuse_color * c,
            specular_color: self.specular_color * c,
            shininess: (1.0 - c) * 10.0,
            mirror: 1.0 - c,
        }
    }
}

impl Display for MandelbrotTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MandelbrotTexture{{at={}, max_iter={}}}",
            self.at, self.max_iter
        )
    }
}

impl Debug for MandelbrotTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MandelbrotTexture{{at={:?}, max_iter={:?}}}",
            self.at, self.max_iter
        )
    }
}
