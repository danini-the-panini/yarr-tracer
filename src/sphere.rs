use crate::{
    interval::Interval,
    material::Material,
    math::Vec3,
    object::{Hit, Object},
    ray::Ray,
    vec3,
};

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

impl Sphere {
    pub fn stationary(center: Vec3, radius: f64, mat: Box<dyn Material>) -> Self {
        Self {
            center: Ray::new(center, vec3!(0.0, 0.0, 0.0), 0.0),
            radius,
            mat,
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f64, mat: Box<dyn Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius,
            mat,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit<'_>> {
        let center = self.center.at(r.time);
        let oc = center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);

        Some(Hit::new(root, p, r, (p - center) / self.radius, &self.mat))
    }
}
