use std::{cmp::Ordering, fmt, ops};

use rand::random;

use crate::util::{random_in_range, EPSILON};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub [f64; N]);

fn gen_arr<const N: usize, T: Default + Copy, F>(gen: F) -> [T; N]
where
    F: Fn(usize) -> T,
{
    let mut arr = [T::default(); N];
    for i in 0..N {
        arr[i] = gen(i)
    }
    arr
}

macro_rules! arr {
    ($n:ident, $g:expr) => {
        gen_arr::<$n, _, _>(|_| $g)
    };
    ($n:ident, $g:expr, $i:ident) => {
        gen_arr::<$n, _, _>(|$i| $g)
    };
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> Vector<N> {
    pub fn repeat(t: f64) -> Self {
        Self([t; N])
    }

    pub fn random() -> Self {
        Self(arr!(N, random()))
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self(arr!(N, random_in_range(min, max)))
    }

    pub fn negate(&mut self) -> &Self {
        for i in 0..N {
            self.0[i] = -self.0[i];
        }
        self
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        self.0.iter().all(|e| e.abs() < EPSILON)
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.0.iter().zip(v.0).map(|(a, b)| a * b).sum()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn normalize(&mut self) -> &Self {
        let len = self.length();
        for i in 0..N {
            self.0[i] /= len;
        }
        self
    }

    pub fn floor(&mut self) -> &Self {
        for i in 0..N {
            self.0[i] = self.0[i].floor()
        }
        self
    }

    pub fn floored(&self) -> Self {
        Vector::<N>(self.0.map(|e| e.floor()))
    }

    pub fn ceil(&mut self) -> &Self {
        for i in 0..N {
            self.0[i] = self.0[i].ceil()
        }
        self
    }

    pub fn ceiled(&self) -> Self {
        Vector::<N>(self.0.map(|e| e.ceil()))
    }

    pub fn round(&mut self) -> &Self {
        for i in 0..N {
            self.0[i] = self.0[i].round()
        }
        self
    }

    pub fn rounded(&self) -> Self {
        Vector::<N>(self.0.map(|e| e.round()))
    }

    pub fn smooth(&mut self) -> &Self {
        for i in 0..N {
            self.0[i] = self.0[i] * self.0[i] * (3.0 - 2.0 * self.0[i]);
        }
        self
    }

    pub fn smoothed(&self) -> Self {
        Vector::<N>(self.0.map(|e| e * e * (3.0 - 2.0 * e)))
    }

    pub(crate) fn abs(&self) -> Self {
        Vector::<N>(self.0.map(|e| e.abs()))
    }
}

impl<const N: usize> PartialEq for Vector<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if (self.0[i] - other.0[i]).abs() >= EPSILON {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> ops::Neg for Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Vector<N> {
        Vector::<N>(self.0.map(|e| -e))
    }
}

impl<const N: usize> ops::Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.0[i]
    }
}

impl<const N: usize> ops::IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.0[i]
    }
}

impl<const N: usize> ops::AddAssign for Vector<N> {
    fn add_assign(&mut self, v: Self) {
        for i in 0..N {
            self.0[i] += v.0[i]
        }
    }
}

impl<const N: usize> ops::SubAssign for Vector<N> {
    fn sub_assign(&mut self, v: Self) {
        for i in 0..N {
            self.0[i] -= v.0[i]
        }
    }
}

impl<const N: usize> ops::MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, t: f64) {
        for i in 0..N {
            self.0[i] *= t
        }
    }
}

impl<const N: usize> ops::MulAssign<Vector<N>> for Vector<N> {
    fn mul_assign(&mut self, v: Vector<N>) {
        for i in 0..N {
            self.0[i] *= v.0[i]
        }
    }
}

impl<const N: usize> ops::DivAssign<f64> for Vector<N> {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl<const N: usize> ops::Add for Vector<N> {
    type Output = Vector<N>;

    fn add(self, v: Self) -> Vector<N> {
        let mut result = Vector::<N>::default();
        for i in 0..N {
            result.0[i] = self.0[i] + v.0[i]
        }
        result
    }
}

impl<const N: usize> ops::Add<f64> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, t: f64) -> Self::Output {
        let mut result = Vector::<N>::default();
        for i in 0..N {
            result.0[i] = self.0[i] + t;
        }
        result
    }
}

impl<const N: usize> ops::Add<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn add(self, v: Vector<N>) -> Self::Output {
        v + self
    }
}

impl<const N: usize> ops::Sub for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, v: Self) -> Vector<N> {
        let mut result = Vector::<N>::default();
        for i in 0..N {
            result.0[i] = self.0[i] - v.0[i]
        }
        result
    }
}

impl<const N: usize> ops::Sub for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, v: Self) -> Vector<N> {
        let mut result = Vector::<N>::default();
        for i in 0..N {
            result.0[i] = self.0[i] - v.0[i]
        }
        result
    }
}

impl<const N: usize> ops::Sub<Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, v: Vector<N>) -> Vector<N> {
        let mut result = Vector::<N>::default();
        for i in 0..N {
            result.0[i] = self.0[i] - v.0[i]
        }
        result
    }
}

impl<const N: usize> ops::Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, t: f64) -> Vector<N> {
        Vector::<N>(self.0.map(|e| e * t))
    }
}

impl<const N: usize> ops::Mul for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, v: Self) -> Vector<N> {
        let mut result = Vector::<N>::default();
        for i in 0..N {
            result.0[i] = self.0[i] * v.0[i]
        }
        result
    }
}

impl<const N: usize> ops::Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, v: Vector<N>) -> Vector<N> {
        Vector::<N>(v.0.map(|e| e * self))
    }
}

impl<const N: usize> ops::Div<f64> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, t: f64) -> Vector<N> {
        (1.0 / t) * self
    }
}

impl<const N: usize> fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl<const N: usize> PartialOrd for Vector<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self - other).near_zero() {
            Some(Ordering::Equal)
        } else {
            self.length_squared().partial_cmp(&other.length_squared())
        }
    }
}

impl<const N: usize> Eq for Vector<N> {}

impl<const N: usize> Ord for Vector<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self - other).near_zero() {
            Ordering::Equal
        } else {
            self.length_squared().total_cmp(&other.length_squared())
        }
    }
}

pub type Vec3 = Vector<3>;

#[macro_export]
macro_rules! vec3 {
    ( $x:expr, $y:expr, $z:expr ) => {
        Vec3::new($x, $y, $z)
    };
    ( $x:expr ) => {
        Vec3::repeat($x)
    };
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq < 1.0 {
                break p;
            };
        }
    }
    pub fn random_unit() -> Self {
        Vec3::random_in_unit_sphere().unit()
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                break p;
            };
        }
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.0[1] * v.0[2] - self.0[2] * v.0[1],
            self.0[2] * v.0[0] - self.0[0] * v.0[2],
            self.0[0] * v.0[1] - self.0[1] * v.0[0],
        )
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * (*n)
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * (*n));
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * (*n);
        r_out_perp + r_out_parallel
    }
}

pub type Point3 = Vec3;

#[macro_export]
macro_rules! point {
    ( $x:expr, $y:expr, $z:expr ) => {
        Point3::new($x, $y, $z)
    };
    ( $x:expr ) => {
        Point3::repeat($x)
    };
}

pub type Vec2 = Vector<2>;

#[macro_export]
macro_rules! vec2 {
    ( $x:expr, $y:expr ) => {
        Vec2::new($x, $y)
    };
    ( $x:expr ) => {
        Vec2::repeat($x)
    };
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn u(&self) -> f64 {
        self.0[0]
    }
    pub fn v(&self) -> f64 {
        self.0[1]
    }

    pub fn sample_square() -> Self {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec2::new(random::<f64>() - 0.5, random::<f64>() - 0.5)
    }
}

#[cfg(test)]
mod vec3_tests {
    use crate::assert_in_delta;

    use super::*;

    #[test]
    fn test_repeat() {
        assert_eq!(Vec3::repeat(2.7), vec3!(2.7, 2.7, 2.7));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-vec3!(1.0, 2.0, 3.0), vec3!(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_index() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut v = vec3!(99.0, 99.0, 99.0);
        v[0] = 1.0;
        v[1] = 2.0;
        v[2] = 3.0;
        assert_eq!(v, vec3!(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_getters() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_add() {
        let u = vec3!(1.0, 2.0, 3.0);
        let v = vec3!(4.0, 5.0, 6.0);
        assert_eq!(u + v, vec3!(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v += vec3!(4.0, 5.0, 6.0);
        assert_eq!(v, vec3!(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let u = vec3!(1.0, 2.0, 3.0);
        let v = vec3!(6.0, 5.0, 4.0);
        assert_eq!(u - v, vec3!(-5.0, -3.0, -1.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v -= vec3!(6.0, 5.0, 4.0);
        assert_eq!(v, vec3!(-5.0, -3.0, -1.0));
    }

    #[test]
    fn test_mul() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, vec3!(2.0, 4.0, 6.0));
        assert_eq!(2.0 * v, vec3!(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, vec3!(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vec() {
        let u = vec3!(1.0, 2.0, 3.0);
        let v = vec3!(7.0, 3.0, 0.5);
        assert_eq!(u * v, vec3!(7.0, 6.0, 1.5));
    }

    #[test]
    fn test_mul_assign_vec() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v *= vec3!(7.0, 3.0, 0.5);
        assert_eq!(v, vec3!(7.0, 6.0, 1.5));
    }

    #[test]
    fn test_div() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, vec3!(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_div_assign() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, vec3!(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_negate() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v.negate();
        assert_eq!(v, vec3!(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_length() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_in_delta!(v.length(), 3.741657387);
    }

    #[test]
    fn test_length_squared() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_eq!(v.length_squared(), 14.0);
    }

    #[test]
    fn test_dot() {
        let u = vec3!(1.0, 2.0, 3.0);
        let v = vec3!(4.0, 5.0, 6.0);
        assert_eq!(u.dot(&v), 32.0);
    }

    #[test]
    fn test_unit() {
        let u = vec3!(1.0, 2.0, 3.0);
        assert_eq!(u.unit(), vec3!(0.267261242, 0.534522484, 0.801783726));
    }

    #[test]
    fn test_normalize() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v.normalize();
        assert_eq!(v, vec3!(0.267261242, 0.534522484, 0.801783726));
    }

    #[test]
    fn test_floored() {
        let v = vec3!(1.2, 2.3, 3.4);
        assert_eq!(v.floored(), vec3!(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_floor() {
        let mut v = vec3!(1.2, 2.3, 3.4);
        v.floor();
        assert_eq!(v, vec3!(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_smooth() {
        let mut v = vec3!(1.0, 2.0, 3.0);
        v.smooth();
        assert_eq!(v, vec3!(1.0, -4.0, -27.0));
    }

    #[test]
    fn test_smoothed() {
        let v = vec3!(1.0, 2.0, 3.0);
        assert_eq!(v.smoothed(), vec3!(1.0, -4.0, -27.0));
    }

    #[test]
    fn test_near_zero() {
        assert!(!vec3!(1.0, 2.0, 3.0).near_zero());
        assert!(vec3!(1e-9, -1e-9, 1e-9).near_zero());
    }

    #[test]
    fn test_random() {
        let v = Vec3::random();
        assert!(0.0 < v.x() && v.x() < 1.0);
        assert!(0.0 < v.y() && v.y() < 1.0);
        assert!(0.0 < v.z() && v.z() < 1.0);
    }

    #[test]
    fn test_random_in_range() {
        let v = Vec3::random_in_range(-10.0, 10.0);
        assert!(-10.0 < v.x() && v.x() < 10.0);
        assert!(-10.0 < v.y() && v.y() < 10.0);
        assert!(-10.0 < v.z() && v.z() < 10.0);
    }

    #[test]
    fn test_reflect() {
        let v = vec3!(1.0, 2.0, 3.0);
        let n = vec3!(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(&n), vec3!(1.0, -2.0, 3.0));
    }

    #[test]
    fn test_refract() {
        let v = vec3!(1.0, 2.0, 3.0);
        let n = vec3!(0.0, 1.0, 0.0);
        assert_eq!(v.refract(&n, 1.5), vec3!(1.5, -4.636809247747852, 4.5));
    }

    #[test]
    fn test_random_in_unit_sphere() {
        let v = Vec3::random_in_unit_sphere();
        assert!(v.length() < 1.0);
    }

    #[test]
    fn test_random_unit_vector() {
        let v = Vec3::random_unit();
        assert_in_delta!(v.length(), 1.0);
    }

    #[test]
    fn test_random_on_hemisphere() {
        let n = vec3!(0.0, 1.0, 0.0);
        let v = Vec3::random_on_hemisphere(&n);

        assert_in_delta!(v.length(), 1.0);
        assert!(v.y() > 0.0);
    }
}

#[cfg(test)]
mod test_vec2 {
    use super::*;

    #[test]
    fn test_getters() {
        let v = vec2!(1.0, 2.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.u(), 1.0);
        assert_eq!(v.v(), 2.0);
    }
}
