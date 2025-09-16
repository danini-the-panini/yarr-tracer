use crate::{
    interval::Interval,
    math::{Point3, Vec3},
    ray::Ray,
};
use std::ops;

#[derive(Debug, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self::new(
            if a.x() <= b.x() {
                Interval::new(a.x(), b.x())
            } else {
                Interval::new(b.x(), a.x())
            },
            if a.y() <= b.y() {
                Interval::new(a.y(), b.y())
            } else {
                Interval::new(b.y(), a.y())
            },
            if a.z() <= b.z() {
                Interval::new(a.z(), b.z())
            } else {
                Interval::new(b.z(), a.z())
            },
        )
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    pub fn axis(&self, i: usize) -> &Interval {
        if i == 1 {
            &self.y
        } else if i == 2 {
            &self.z
        } else {
            &self.x
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0_usize..3_usize {
            let ax = self.axis(axis);
            let adinv = 1.0 / r.direction[axis];

            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0
                }
                if t1 < ray_t.max {
                    ray_t.max = t1
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1
                }
                if t0 < ray_t.max {
                    ray_t.max = t0
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(&mut self) -> &Self {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.pad(delta);
        }
        if self.y.size() < delta {
            self.y.pad(delta);
        }
        if self.z.size() < delta {
            self.z.pad(delta);
        }
        self
    }
}

impl ops::Add for AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        AABB {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add for &AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        AABB {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<&AABB> for AABB {
    fn add_assign(&mut self, rhs: &AABB) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        AABB::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl ops::Add<Vec3> for &AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        AABB::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}
