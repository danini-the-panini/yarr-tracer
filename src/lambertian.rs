use crate::{
    color::Color,
    material::{Material, Scatter},
    math::Vec3,
    object::Hit,
    ray::Ray,
    solid_color::SolidColor,
    texture::Texture,
};

pub struct Lambertian {
    pub tex: Box<dyn Texture>,
}

impl Lambertian {
    pub fn solid(albedo: Color) -> Self {
        Self {
            tex: Box::new(SolidColor(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        Some(Scatter {
            att: self.tex.sample(hit.uv, hit.p),
            ray: Ray::new(hit.p, scatter_direction, r_in.time),
        })
    }
}
