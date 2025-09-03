use std::f64::consts::PI;

use rand::random;

use crate::{
    math::{Point3, Vec2},
    vec2,
};

pub const EPSILON: f64 = 1e-8;

pub fn random_in_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random::<f64>()
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn gamma_to_linear(gamma_component: f64, gamma: f64) -> f64 {
    if gamma_component > 0.0 {
        return gamma_component.powf(gamma);
    }
    0.0
}

pub fn sphere_uv(p: &Point3) -> Vec2 {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    vec2!(phi / (2.0 * PI), theta / PI)
}

#[cfg(test)]
mod test {
    use crate::assert_in_delta;

    use super::*;

    #[test]
    fn test_random_in_range() {
        let x = random_in_range(-10.0, 10.0);
        assert!(-10.0 < x && x < 10.0);
    }

    #[test]
    fn test_linear_to_gamma() {
        assert_eq!(linear_to_gamma(0.0), 0.0);
        assert_eq!(linear_to_gamma(-1.0), 0.0);
        assert_in_delta!(linear_to_gamma(0.25), 0.5);
    }

    #[test]
    fn test_gamma_to_linear() {
        assert_eq!(gamma_to_linear(0.0, 2.2), 0.0);
        assert_eq!(gamma_to_linear(-1.0, 2.2), 0.0);
        assert_in_delta!(gamma_to_linear(0.5, 2.2), 0.217637640824031);
        assert_in_delta!(gamma_to_linear(0.5, 2.0), 0.25);
    }
}
