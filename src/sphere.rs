use crate::{
    interval::Interval,
    material::Material,
    math::Vec3,
    object::{Hit, Object},
    ray::Ray,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit<'_>> {
        let oc = self.center - r.origin;
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

        Some(Hit::new(
            root,
            p,
            r,
            (p - self.center) / self.radius,
            &self.mat,
        ))
    }
}
