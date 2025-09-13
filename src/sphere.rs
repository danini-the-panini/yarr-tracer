use std::sync::Arc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    math::Vec3,
    object::{Hit, Object},
    ray::Ray,
    util::sphere_uv,
    vec3,
};

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn stationary(center: Vec3, radius: f64, mat: &Arc<dyn Material>) -> Self {
        let rvec = vec3!(radius, radius, radius);
        Self {
            center: Ray::new(center, vec3!(0.0, 0.0, 0.0), 0.0),
            radius,
            mat: Arc::clone(mat),
            bbox: AABB::from_points(center - rvec, center + rvec),
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f64, mat: &Arc<dyn Material>) -> Self {
        let rvec = vec3!(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1, 0.0);
        Self {
            center,
            radius,
            mat: Arc::clone(mat),
            bbox: AABB::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec)
                + AABB::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec),
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit> {
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
        let normal = (p - center) / self.radius;

        Some(Hit::new(root, p, r, normal, sphere_uv(&normal), &self.mat))
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
