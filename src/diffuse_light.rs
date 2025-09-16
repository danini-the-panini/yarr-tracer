use std::sync::Arc;

use crate::{
    color::Color, material::Material, object::Hit, ray::Ray, solid_color::SolidColor,
    texture::Texture,
};

pub struct DiffuseLight {
    pub tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn solid(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor(albedo)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, hit: &Hit) -> crate::color::Color {
        self.tex.sample_tex(&hit.uv, &hit.p)
    }
}
