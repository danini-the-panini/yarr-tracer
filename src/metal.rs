use std::sync::Arc;

use crate::{
    color::Color,
    material::{Material, Scatter},
    math::Vec3,
    object::Hit,
    ray::Ray,
    solid_color::SolidColor,
    texture::Texture,
};

pub struct Metal {
    pub tex: Arc<dyn Texture>,
    pub fuzz: f64,
}

impl Metal {
    pub fn solid(albedo: Color, fuzz: f64) -> Self {
        Self {
            tex: Arc::new(SolidColor(albedo)),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut reflected = r_in.direction.reflect(&hit.normal);
        reflected = reflected.unit() + (self.fuzz * Vec3::random_unit());

        if reflected.dot(&hit.normal) <= 0.0 {
            return None;
        }

        Some(Scatter {
            att: self.tex.sample_tex(&hit.uv, &hit.p),
            ray: Ray::new(hit.p, reflected, r_in.time),
        })
    }
}
