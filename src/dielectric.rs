use rand::random;

use crate::{
    color::Color,
    material::{Material, Scatter},
    object::Hit,
    ray::Ray,
    rgb,
};

pub struct Dielectric {
    pub refraction_index: f64,
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.unit();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random() {
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, ri)
        };

        Some(Scatter {
            att: rgb!(1.0, 1.0, 1.0),
            ray: Ray::new(hit.p, direction, 0.0),
        })
    }
}
