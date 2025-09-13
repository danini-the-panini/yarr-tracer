use std::sync::Arc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    math::{Point3, Vec2, Vec3},
    object::{Hit, Object},
    util::EPSILON,
};

static UNIT_INVERVAL: Interval = Interval { min: 0.0, max: 1.0 };

fn quad_uv(a: f64, b: f64) -> Option<Vec2> {
    if UNIT_INVERVAL.contains(a) && UNIT_INVERVAL.contains(b) {
        Some(Vec2::new(a, b))
    } else {
        None
    }
}

pub struct Quad {
    pub q: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: &Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = normal / n.length();
        let bbox1 = AABB::from_points(q, q + u + v);
        let bbox2 = AABB::from_points(q + u, q + v);
        Self {
            q,
            u,
            v,
            w,
            mat: Arc::clone(mat),
            bbox: bbox1 + bbox2,
            normal,
            d,
        }
    }
}

impl Object for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
    ) -> Option<crate::object::Hit> {
        let denom = self.normal.dot(&r.direction);

        if denom.abs() < EPSILON {
            return None;
        }

        let t = (self.d - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let hitp = intersection - self.q;
        let alpha = self.w.dot(&hitp.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&hitp));

        match quad_uv(alpha, beta) {
            Some(uv) => Some(Hit::new(t, intersection, r, self.normal, uv, &self.mat)),
            None => None,
        }
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
