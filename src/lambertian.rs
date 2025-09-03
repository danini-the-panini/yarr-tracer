use crate::{
    color::Color,
    material::{Material, Scatter},
    math::Vec3,
    object::Hit,
    ray::Ray,
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        Some(Scatter {
            att: self.albedo,
            ray: Ray::new(hit.p, scatter_direction, 0.0),
        })
    }
}
