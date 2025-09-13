use std::sync::Arc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    math::{Point3, Vec2, Vec3},
    ray::Ray,
};

pub struct Hit {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub uv: Vec2,
    pub mat: Arc<dyn Material>,
}

impl Hit {
    pub fn new(
        t: f64,
        p: Vec3,
        r: &Ray,
        outward_normal: Vec3,
        uv: Vec2,
        mat: &Arc<dyn Material>,
    ) -> Self {
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
            uv,
            mat: Arc::clone(mat),
        }
    }
}

pub trait Object: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit>;
    fn bbox(&self) -> &AABB;
}
