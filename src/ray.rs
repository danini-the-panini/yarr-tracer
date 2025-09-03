use crate::math::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub direction: Vec3,
    pub origin: Point3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod test {
    use crate::{point, vec3};

    use super::*;

    #[test]
    fn test_at() {
        let r = Ray::new(point!(1.0, 2.0, 3.0), vec3!(4.0, 5.0, 6.0), 0.0);
        assert_eq!(r.at(0.0), r.origin);
        assert_eq!(r.at(1.0), point!(5.0, 7.0, 9.0));
        assert_eq!(r.at(1.5), point!(7.0, 9.5, 12.0));
    }
}
