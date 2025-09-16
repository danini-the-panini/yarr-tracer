use std::sync::Arc;

use rand::random;

use crate::{
    color::Color,
    interval::Interval,
    material::{Material, Scatter},
    math::Vec3,
    object::{Hit, Object},
    ray::Ray,
    solid_color::SolidColor,
    texture::Texture,
    vec3,
};

pub struct ConstantMedium {
    pub boundary: Box<dyn Object>,
    pub phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Object>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    pub fn solid(boundary: Box<dyn Object>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::solid(color)),
        }
    }
}

impl Object for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &crate::interval::Interval) -> Option<crate::object::Hit> {
        self.boundary
            .hit(r, &Interval::universe())
            .and_then(|hit1| {
                self.boundary
                    .hit(r, &Interval::from(hit1.t + 0.0001))
                    .map(|hit2| (hit1.t, hit2.t))
            })
            .filter(|(mut t1, mut t2)| {
                if t1 < ray_t.min {
                    t1 = ray_t.min;
                }
                if t2 > ray_t.max {
                    t2 = ray_t.max;
                }
                t1 <= t2
            })
            .and_then(|(mut t1, t2)| {
                if t1 < 0.0 {
                    t1 = 0.0
                }

                let ray_length = r.direction.length();
                let distance_inside_boundary = (t2 - t1) * ray_length;
                let hit_distance = self.neg_inv_density * random::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    None
                } else {
                    let t = t1 + hit_distance / ray_length;
                    Some(Hit {
                        t,
                        p: r.at(t),
                        uv: Default::default(),
                        normal: vec3!(1.0, 0.0, 0.0),
                        front_face: true,
                        mat: Arc::clone(&self.phase_function),
                    })
                }
            })
    }

    fn bbox(&self) -> &crate::aabb::AABB {
        self.boundary.bbox()
    }
}

pub struct Isotropic(Arc<dyn Texture>);

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self(tex)
    }

    pub fn solid(color: Color) -> Self {
        Self(Arc::new(SolidColor(color)))
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        hit: &crate::object::Hit,
    ) -> Option<crate::material::Scatter> {
        Some(Scatter {
            ray: Ray::new(hit.p, Vec3::random_unit(), r_in.time),
            att: self.0.sample_tex(&hit.uv, &hit.p),
        })
    }
}
