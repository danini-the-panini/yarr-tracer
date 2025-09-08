use crate::{
    aabb::AABB,
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
    bbox: AABB,
}

impl Sphere {
    pub fn stationary(center: Vec3, radius: f64, mat: Box<dyn Material>) -> Self {
        let rvec = vec3!(radius, radius, radius);
        Self {
            center: Ray::new(center, vec3!(0.0, 0.0, 0.0), 0.0),
            radius,
            mat,
            bbox: AABB::from_points(center - rvec, center + rvec),
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f64, mat: Box<dyn Material>) -> Self {
        let rvec = vec3!(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1, 0.0);
        Self {
            center,
            radius,
            mat,
            bbox: AABB::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec)
                + AABB::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec),
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

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
