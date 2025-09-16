use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    aabb::AABB,
    interval::Interval,
    math::{Point3, Vec3},
    object::{Hit, Object},
    point,
    ray::Ray,
    vec3,
};

pub struct Translate {
    pub obj: Box<dyn Object>,
    pub offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(obj: Box<dyn Object>, offset: Vec3) -> Self {
        let bbox = obj.bbox() + offset;
        Self { obj, offset, bbox }
    }
}

impl Object for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit> {
        let offset_r = Ray::new(r.origin - self.offset, r.direction, r.time);

        self.obj.hit(&offset_r, ray_t).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

pub struct RotateY {
    pub obj: Box<dyn Object>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(obj: Box<dyn Object>, angle: f64) -> Self {
        let angle = angle.to_radians();
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();
        let bbox = obj.bbox();

        let mut min = point!(INFINITY, INFINITY, INFINITY);
        let mut max = point!(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.x.max + ((1 - i) as f64) * bbox.x.min;
                    let y = (j as f64) * bbox.y.max + ((1 - j) as f64) * bbox.y.min;
                    let z = (k as f64) * bbox.z.max + ((1 - k) as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = vec3!(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            obj,
            sin_theta,
            cos_theta,
            bbox: AABB::from_points(min, max),
        }
    }
}

impl Object for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit> {
        // Transform the ray from world space to object space.

        let origin = point!(
            (self.cos_theta * r.origin.x()) - (self.sin_theta * r.origin.z()),
            r.origin.y(),
            (self.sin_theta * r.origin.x()) + (self.cos_theta * r.origin.z())
        );

        let direction = vec3!(
            (self.cos_theta * r.direction.x()) - (self.sin_theta * r.direction.z()),
            r.direction.y(),
            (self.sin_theta * r.direction.x()) + (self.cos_theta * r.direction.z())
        );

        let rotated_r = Ray::new(origin, direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where).

        self.obj.hit(&rotated_r, ray_t).map(|mut hit| {
            // Transform the intersection from object space back to world space.
            hit.p = point!(
                (self.cos_theta * hit.p.x()) + (self.sin_theta * hit.p.z()),
                hit.p.y(),
                (-self.sin_theta * hit.p.x()) + (self.cos_theta * hit.p.z())
            );

            hit.normal = vec3!(
                (self.cos_theta * hit.normal.x()) + (self.sin_theta * hit.normal.z()),
                hit.normal.y(),
                (-self.sin_theta * hit.normal.x()) + (self.cos_theta * hit.normal.z())
            );

            hit
        })
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
