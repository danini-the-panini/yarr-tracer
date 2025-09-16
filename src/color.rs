use image::{Rgb, Rgba};

use crate::math::Vec3;
use crate::{interval::Interval, util::linear_to_gamma};

pub type Color = Vec3;

#[macro_export]
macro_rules! rgb {
    ( $r:expr, $g:expr, $b:expr ) => {
        Color::new($r, $g, $b)
    };
    ( $x:expr) => {
        Color::repeat($x)
    };
}

impl Color {
    pub fn r(&self) -> f64 {
        self.0[0]
    }
    pub fn g(&self) -> f64 {
        self.0[1]
    }
    pub fn b(&self) -> f64 {
        self.0[2]
    }

    pub fn to_pixel(&self) -> (u8, u8, u8) {
        let r = self.r();
        let g = self.g();
        let b = self.b();

        // Apply a linear to gamma transform for gamma 2
        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        // Translate the [0,1] component values to the byte range [0,255].
        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as u8;
        let gbyte = (256.0 * intensity.clamp(g)) as u8;
        let bbyte = (256.0 * intensity.clamp(b)) as u8;

        // Return the pixel color components.
        (rbyte, gbyte, bbyte)
    }
}

impl From<&Rgb<f32>> for Color {
    fn from(Rgb(pixel): &Rgb<f32>) -> Self {
        rgb!(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_getters() {
        let c = rgb!(0.1, 0.2, 0.3);
        assert_eq!(c.r(), 0.1);
        assert_eq!(c.g(), 0.2);
        assert_eq!(c.b(), 0.3);
    }

    #[test]
    fn test_to_pixel() {
        let c = rgb!(0.1, 0.2, 0.3);
        assert_eq!(c.to_pixel(), (80, 114, 140));
    }
}
