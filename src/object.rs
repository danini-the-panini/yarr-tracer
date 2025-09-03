use crate::{
    interval::Interval,
    math::{Point3, Vec3},
    ray::Ray,
};

pub struct Hit {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn new(t: f64, p: Vec3, r: &Ray, outward_normal: Vec3) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        Self {
            t,
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            front_face,
        }
    }
}

pub trait Object: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit>;
}
