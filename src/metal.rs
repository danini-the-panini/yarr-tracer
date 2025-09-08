use crate::{
    color::Color,
    material::{Material, Scatter},
    math::Vec3,
    object::Hit,
    ray::Ray,
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut reflected = r_in.direction.reflect(&hit.normal);
        reflected = reflected.unit() + (self.fuzz * Vec3::random_unit());

        if reflected.dot(&hit.normal) <= 0.0 {
            return None;
        }

        Some(Scatter {
            att: self.albedo,
            ray: Ray::new(hit.p, reflected, r_in.time),
        })
    }
}
