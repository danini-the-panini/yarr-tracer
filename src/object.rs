use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    math::{Point3, Vec3},
    ray::Ray,
};

pub struct Hit<'a> {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: &'a Box<dyn Material>,
}

impl<'a> Hit<'a> {
    pub fn new(t: f64, p: Vec3, r: &Ray, outward_normal: Vec3, mat: &'a Box<dyn Material>) -> Self {
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
            mat,
        }
    }
}

pub trait Object: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit<'_>>;
    fn bbox(&self) -> &AABB;
}
