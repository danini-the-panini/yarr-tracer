use std::sync::Arc;

use crate::{material::Material, object::Hit, ray::Ray, texture::Texture};

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: &Arc<dyn Texture>) -> Self {
        Self {
            tex: Arc::clone(tex),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, hit: &Hit) -> crate::color::Color {
        self.tex.sample_tex(&hit.uv, &hit.p)
    }
}
